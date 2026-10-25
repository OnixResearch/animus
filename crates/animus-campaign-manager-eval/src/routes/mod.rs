//! Model routes: the adapter a trial's manager calls, and the deterministic scripted policies.
//!
//! A scripted policy reads only the rendered brief, exactly what a model sees, and answers in the
//! manager's reply schema.

mod reading;

/// Rank a policy gives a candidate it has not seen executed.
const EXPLORE_RANK: i64 = 10;
/// Rank a policy gives the unexecuted members of the family it exploits.
const EXPLOIT_RANK: i64 = 20;
/// Smallest observed signal that makes the signal-greedy policy exploit a family.
const EXPLOIT_SIGNAL: u32 = 6;
/// The malformed policy's reply.
const MALFORMED_REPLY: &str = "I would rather not commit to a plan yet.";

/// Why a model route failed to answer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteFailure {
    /// The route could not be reached.
    Unavailable,
    /// The route refused or failed the request.
    Rejected,
    /// The route's answer could not become a model response.
    Malformed,
}

/// A model route as the trial environment calls it.
pub type RouteAdapter = dyn animus_adapters::ComponentAdapter<
        animus_adapters::ModelRequest,
        animus_adapters::ModelResponse,
        Error = RouteFailure,
    >;

/// One named model route of a job.
pub struct Route<'model> {
    /// Harbor model-route identity.
    pub identity: String,
    /// The route's model adapter.
    pub model: &'model mut RouteAdapter,
}

/// A sized model adapter over one route, for the manager's generic `advise`.
pub struct RouteModel<'model>(pub &'model mut RouteAdapter);

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for RouteModel<'_>
{
    type Error = RouteFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        self.0.call(request)
    }
}

/// A deterministic offline policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScriptedPolicy {
    /// Explores unexecuted candidates; once a family shows a strong signal, adds that family's
    /// offers and runs them first.
    SignalGreedy,
    /// Explores unexecuted candidates, then reruns the best one; never adds offers.
    ExploreOnly,
    /// Never answers in the reply schema.
    Malformed,
}

impl ScriptedPolicy {
    /// Every scripted policy, in route order.
    pub const ALL: [Self; 3] = [Self::SignalGreedy, Self::ExploreOnly, Self::Malformed];

    /// The policy's Harbor model-route identity.
    #[must_use]
    pub const fn identity(self) -> &'static str {
        match self {
            Self::SignalGreedy => "model:scripted:signal-greedy",
            Self::ExploreOnly => "model:scripted:explore-only",
            Self::Malformed => "model:scripted:malformed",
        }
    }

    /// The policy's reply to one rendered brief.
    #[must_use]
    pub fn reply(self, prompt: &str) -> String {
        let view = reading::read_prompt(prompt);
        match self {
            Self::SignalGreedy => scores_reply(&view, exploited_family(&view)),
            Self::ExploreOnly => scores_reply(&view, None),
            Self::Malformed => String::from(MALFORMED_REPLY),
        }
    }
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for ScriptedPolicy
{
    type Error = RouteFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        let reply = self.reply(request.prompt());
        animus_adapters::ModelResponse::new(&reply).map_err(|_| RouteFailure::Malformed)
    }
}

/// The family to exploit: the best observed signal's family, once it is strong or nothing is
/// left to explore.
fn exploited_family(view: &reading::PromptView) -> Option<&str> {
    let best = view
        .candidates
        .iter()
        .filter_map(|listed| listed.signal.map(|signal| (signal, listed.family.as_str())))
        .max_by_key(|(signal, _)| *signal)?;
    let is_explored = view.candidates.iter().all(|listed| listed.signal.is_some());
    (best.0 >= EXPLOIT_SIGNAL || is_explored).then_some(best.1)
}

/// A reply that scores every listed candidate and, for an exploited family, adds its offers.
fn scores_reply(view: &reading::PromptView, family: Option<&str>) -> String {
    format!(
        r#"{{"scores":[{}],"add":[{}]}}"#,
        scored(view, family).join(","),
        added(view, family).join(",")
    )
}

/// One score per listed candidate: its observed signal once run, else the exploit rank for the
/// exploited family or the explore rank.
fn scored(view: &reading::PromptView, family: Option<&str>) -> Vec<String> {
    let rank = |listed: &reading::Listed| match listed.signal {
        Some(signal) => i64::from(signal),
        None if family == Some(listed.family.as_str()) => EXPLOIT_RANK,
        None => EXPLORE_RANK,
    };
    view.candidates
        .iter()
        .map(|listed| {
            let ranks = reading::ranks(rank(listed), view.dimensions);
            format!(r#"{{"candidate":"{}","ranks":{ranks}}}"#, listed.handle)
        })
        .collect()
}

/// The exploited family's offers, each with the exploit rank.
fn added(view: &reading::PromptView, family: Option<&str>) -> Vec<String> {
    view.offers
        .iter()
        .filter(|listed| family == Some(listed.family.as_str()))
        .map(|listed| {
            let ranks = reading::ranks(EXPLOIT_RANK, view.dimensions);
            format!(r#"{{"offer":"{}","ranks":{ranks}}}"#, listed.handle)
        })
        .collect()
}
