//! The predeclared local-scorer comparison: its arms, their timing, and the job over them.
//!
//! Timing belongs to this shell: every arm's calls are wrapped, and latency is reported as a
//! non-deterministic observation beside the deterministic cost units the scorer records.

mod report;

/// Job identity of the comparison.
const JOB: &str = "job:campaign-manager-local-scorer-eval";
/// Latency samples one arm retains.
const MAX_LATENCY_SAMPLES: usize = 4_096;

/// A route whose calls are timed.
struct Timed<A> {
    identity: String,
    inner: A,
    latencies: Vec<std::time::Duration>,
}

impl<A> Timed<A> {
    const fn new(identity: String, inner: A) -> Self {
        Self {
            identity,
            inner,
            latencies: Vec::new(),
        }
    }
}

impl<A>
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for Timed<A>
where
    A: animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
            Error = animus_campaign_manager_eval::RouteFailure,
        >,
{
    type Error = animus_campaign_manager_eval::RouteFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        let (result, latency) = timed(|| self.inner.call(request));
        if self.latencies.len() < MAX_LATENCY_SAMPLES {
            self.latencies.push(latency);
        }
        result
    }
}

/// Runs `call` and returns its result with the wall-clock time it took.
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::ambient_clock,
        reason = "the comparison shell times each arm's calls; the report marks this latency as a \
                  non-deterministic observation, and no decision or record reads it"
    )
)]
fn timed<T>(call: impl FnOnce() -> T) -> (T, std::time::Duration) {
    let started = std::time::Instant::now();
    let value = call();
    (value, started.elapsed())
}

/// The arms of one comparison, borrowed by the job's routes.
struct Arms {
    baseline: Timed<animus_campaign_manager_eval::ScriptedPolicy>,
    references: Vec<Timed<animus_campaign_manager_eval::ScriptedPolicy>>,
    scorer: Timed<animus_campaign_manager_eval::LocalScorerRoute>,
    randoms: Vec<Timed<animus_campaign_manager_eval::RandomRankRoute>>,
}

/// Runs the embedded protocol over `set` and renders its report.
// r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]
pub fn run(set: &animus_campaign_manager_eval::TaskSet) -> Result<String, String> {
    let protocol = animus_campaign_manager_eval::embedded_comparison_protocol()
        .map_err(|error| format!("comparison protocol refused: {error:?}"))?;
    let admitted = animus_campaign_manager_eval::admit_artifact(
        animus_campaign_manager_eval::embedded_local_scorer_fixture(),
        &protocol.artifact_identity,
    )
    .map_err(|error| format!("selected artifact refused: {error:?}"))?;
    let bounds = admitted.rank_bounds;
    let scorer = animus_campaign_manager_eval::LocalScorerRoute::new(Ok(admitted));
    let baseline = animus_campaign_manager_eval::FALLBACK_POLICY;
    let mut arms = Arms {
        baseline: Timed::new(String::from(baseline.identity()), baseline),
        references: protocol
            .references
            .iter()
            .filter_map(|identity| {
                animus_campaign_manager_eval::ScriptedPolicy::ALL
                    .into_iter()
                    .find(|policy| policy.identity() == identity)
            })
            .map(|policy| Timed::new(String::from(policy.identity()), policy))
            .collect(),
        scorer: Timed::new(String::from(scorer.identity()), scorer),
        randoms: protocol
            .random_seeds
            .iter()
            .map(|seed| {
                let random = animus_campaign_manager_eval::RandomRankRoute::new(*seed, bounds);
                Timed::new(String::from(random.identity()), random)
            })
            .collect(),
    };
    debug_assert_eq!(
        arms.references.len(),
        protocol.references.len(),
        "the protocol admits only scripted references"
    );
    debug_assert_eq!(arms.baseline.identity, protocol.baseline);
    let finished = run_arms(set, &mut arms)?;
    report::render(&protocol, &finished, &arms)
}

/// Runs every arm on every task of `set` as one job.
fn run_arms(
    set: &animus_campaign_manager_eval::TaskSet,
    arms: &mut Arms,
) -> Result<animus_campaign_manager_eval::JobReport, String> {
    let mut routes = vec![route(&mut arms.baseline)];
    routes.extend(arms.references.iter_mut().map(route));
    routes.push(route(&mut arms.scorer));
    routes.extend(arms.randoms.iter_mut().map(route));
    animus_campaign_manager_eval::run_job(set, &mut routes, JOB)
        .map_err(|error| format!("job refused: {error:?}"))
}

/// `arm` as one route of the job.
fn route<A>(arm: &mut Timed<A>) -> animus_campaign_manager_eval::Route<'_>
where
    A: animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
            Error = animus_campaign_manager_eval::RouteFailure,
        > + 'static,
{
    animus_campaign_manager_eval::Route {
        identity: arm.identity.clone(),
        model: arm,
    }
}
