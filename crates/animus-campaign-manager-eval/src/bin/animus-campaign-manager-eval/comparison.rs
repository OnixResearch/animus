//! The predeclared scorer comparisons: their arms, their timing, and the job over them.
//!
//! Timing belongs to this shell: every arm's calls are wrapped, and latency is reported as a
//! non-deterministic observation beside the deterministic cost units the scorer records.

mod report;

/// Job identity of the linear local-scorer comparison.
const JOB: &str = "job:campaign-manager-local-scorer-eval";
/// Job identity of the frozen-policy comparison.
const FROZEN_POLICY_JOB: &str = "job:campaign-manager-frozen-policy-eval";
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

/// A scorer route whose decision records the report reads.
trait Recorded {
    /// The route's Harbor model-route identity.
    fn route_identity(&self) -> &str;
    /// The route's retained decision records.
    fn records(&self) -> &[animus_campaign_manager_eval::DecisionRecord];
}

impl Recorded for animus_campaign_manager_eval::LocalScorerRoute {
    fn route_identity(&self) -> &str {
        self.identity()
    }

    fn records(&self) -> &[animus_campaign_manager_eval::DecisionRecord] {
        self.decisions()
    }
}

impl Recorded for animus_campaign_manager_eval::ClassifierRoute {
    fn route_identity(&self) -> &str {
        self.identity()
    }

    fn records(&self) -> &[animus_campaign_manager_eval::DecisionRecord] {
        self.decisions()
    }
}

/// The arms of one comparison, borrowed by the job's routes.
struct Arms<S> {
    baseline: Timed<animus_campaign_manager_eval::ScriptedPolicy>,
    references: Vec<Timed<animus_campaign_manager_eval::ScriptedPolicy>>,
    scorer: Timed<S>,
    randoms: Vec<Timed<animus_campaign_manager_eval::RandomRankRoute>>,
}

/// What the report's header names.
struct Header {
    title: &'static str,
    provenance: String,
}

/// One comparison's scorer arm, random-rank bounds, job, and header.
struct ArmSetup<S> {
    scorer: S,
    bounds: animus_campaign_manager_eval::RankBounds,
    job: &'static str,
    header: Header,
}

/// Runs the embedded linear local-scorer protocol over `set` and renders its report.
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
    debug_assert_eq!(
        protocol.scorer_arm(),
        Some(animus_campaign_manager_eval::ScorerArm::LocalScorer)
    );
    debug_assert_eq!(admitted.identity, protocol.artifact_identity);
    let setup = ArmSetup {
        scorer: animus_campaign_manager_eval::LocalScorerRoute::new(Ok(admitted)),
        bounds,
        job: JOB,
        header: Header {
            title: "LOCAL_SCORER_COMPARISON",
            provenance: String::from("fixture"),
        },
    };
    run_protocol(set, &protocol, setup)
}

/// Runs the embedded frozen-policy protocol over `set` and renders its report.
// r[impl agent_harness_runtime.frozen_policy.evaluation]
pub fn run_frozen_policy(set: &animus_campaign_manager_eval::TaskSet) -> Result<String, String> {
    let protocol = animus_campaign_manager_eval::embedded_frozen_policy_protocol()
        .map_err(|error| format!("comparison protocol refused: {error:?}"))?;
    let admitted = animus_frozen_policy::admit_policy(
        animus_campaign_manager_eval::embedded_frozen_policy_fixture(),
        &protocol.artifact_identity,
    )
    .map_err(|error| format!("selected artifact refused: {error:?}"))?;
    let provenance = String::from(admitted.provenance_kind());
    let bounds = animus_campaign_manager_eval::ClassifierRoute::rank_bounds()
        .ok_or_else(|| String::from("the route's rank bounds are inverted"))?;
    debug_assert_eq!(
        protocol.scorer_arm(),
        Some(animus_campaign_manager_eval::ScorerArm::FrozenPolicy)
    );
    debug_assert_eq!(admitted.identity(), protocol.artifact_identity);
    let setup = ArmSetup {
        scorer: animus_campaign_manager_eval::ClassifierRoute::new(
            Ok(admitted),
            &protocol.artifact_identity,
        ),
        bounds,
        job: FROZEN_POLICY_JOB,
        header: Header {
            title: "FROZEN_POLICY_COMPARISON",
            provenance,
        },
    };
    run_protocol(set, &protocol, setup)
}

/// Runs `protocol`'s arms with `setup`'s scorer over `set` and renders the report.
fn run_protocol<S>(
    set: &animus_campaign_manager_eval::TaskSet,
    protocol: &animus_campaign_manager_eval::ComparisonProtocol,
    setup: ArmSetup<S>,
) -> Result<String, String>
where
    S: animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
            Error = animus_campaign_manager_eval::RouteFailure,
        > + Recorded
        + 'static,
{
    let baseline = animus_campaign_manager_eval::FALLBACK_POLICY;
    let scorer_identity = String::from(setup.scorer.route_identity());
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
        scorer: Timed::new(scorer_identity, setup.scorer),
        randoms: protocol
            .random_seeds
            .iter()
            .map(|seed| {
                let random =
                    animus_campaign_manager_eval::RandomRankRoute::new(*seed, setup.bounds);
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
    let finished = run_arms(set, &mut arms, setup.job)?;
    report::render(protocol, &finished, &arms, &setup.header)
}

/// Runs every arm on every task of `set` as one job.
fn run_arms<S>(
    set: &animus_campaign_manager_eval::TaskSet,
    arms: &mut Arms<S>,
    job: &str,
) -> Result<animus_campaign_manager_eval::JobReport, String>
where
    S: animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
            Error = animus_campaign_manager_eval::RouteFailure,
        > + 'static,
{
    let mut routes = vec![route(&mut arms.baseline)];
    routes.extend(arms.references.iter_mut().map(route));
    routes.push(route(&mut arms.scorer));
    routes.extend(arms.randoms.iter_mut().map(route));
    animus_campaign_manager_eval::run_job(set, &mut routes, job)
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
