//! Runs the campaign-manager evaluation job and prints a metadata-only report.
//!
//! Scripted mode, the default, runs every task against every scripted policy offline:
//!
//! ```text
//! animus-campaign-manager-eval
//! ```
//!
//! Live mode, built with `--features live`, runs every task against one reviewed endpoint and
//! model. It reads the credential from standard input:
//!
//! ```text
//! animus-campaign-manager-eval --live openrouter --model <MODEL> --credential-stdin < key
//! ```
//!
//! The report holds identities, counts, and mean rewards. It never includes prompts, replies,
//! labels, or credentials.

/// Job identity of every run.
const JOB: &str = "job:campaign-manager-eval";
/// Exit status of a refused invocation or job.
const FAILURE: u8 = 2;

fn main() -> std::process::ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match run(&arguments) {
        Ok(report) => {
            print!("{report}");
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            std::process::ExitCode::from(FAILURE)
        }
    }
}

/// Runs the requested job and renders its report.
fn run(arguments: &[String]) -> Result<String, String> {
    let set = animus_campaign_manager_eval::embedded_task_set()
        .map_err(|error| format!("task set refused: {error:?}"))?;
    if arguments.is_empty() {
        return scripted(&set);
    }
    live(&set, arguments)
}

/// Runs every scripted policy.
fn scripted(set: &animus_campaign_manager_eval::TaskSet) -> Result<String, String> {
    let mut policies = animus_campaign_manager_eval::ScriptedPolicy::ALL;
    let mut routes: Vec<animus_campaign_manager_eval::Route<'_>> = policies
        .iter_mut()
        .map(|policy| animus_campaign_manager_eval::Route {
            identity: String::from(policy.identity()),
            model: policy,
        })
        .collect();
    let finished = animus_campaign_manager_eval::run_job(set, &mut routes, JOB)
        .map_err(|error| format!("job refused: {error:?}"))?;
    Ok(render(set, &finished))
}

#[cfg(feature = "live")]
/// Runs the live route named by `arguments`.
fn live(
    set: &animus_campaign_manager_eval::TaskSet,
    arguments: &[String],
) -> Result<String, String> {
    let (endpoint, model) = live_arguments(arguments)?;
    let credential = read_credential()?;
    let mut route = animus_campaign_manager_eval::LiveRoute::new(endpoint, &model, credential)
        .map_err(|error| format!("live route refused: {error:?}"))?;
    let identity = String::from(route.identity());
    let mut routes = [animus_campaign_manager_eval::Route {
        identity,
        model: &mut route,
    }];
    let finished = animus_campaign_manager_eval::run_job(set, &mut routes, JOB)
        .map_err(|error| format!("job refused: {error:?}"))?;
    Ok(render(set, &finished))
}

#[cfg(not(feature = "live"))]
/// Live mode is unavailable without the `live` feature.
fn live(
    _set: &animus_campaign_manager_eval::TaskSet,
    arguments: &[String],
) -> Result<String, String> {
    Err(format!(
        "unsupported arguments {arguments:?}; live mode needs a build with `--features live`"
    ))
}

#[cfg(feature = "live")]
/// The endpoint and model of `--live <openai|openrouter> --model <ID> --credential-stdin`.
fn live_arguments(
    arguments: &[String],
) -> Result<(animus_campaign_manager_eval::Endpoint, String), String> {
    const USAGE: &str = "usage: --live <openai|openrouter> --model <MODEL> --credential-stdin";
    let [live, endpoint, model_flag, model, credential_flag] = arguments else {
        return Err(String::from(USAGE));
    };
    let flags = [live.as_str(), model_flag.as_str(), credential_flag.as_str()];
    if flags != ["--live", "--model", "--credential-stdin"] {
        return Err(String::from(USAGE));
    }
    Ok((endpoint_named(endpoint)?, model.clone()))
}

#[cfg(feature = "live")]
/// The reviewed endpoint called `name`.
fn endpoint_named(name: &str) -> Result<animus_campaign_manager_eval::Endpoint, String> {
    match name {
        "openai" => Ok(animus_campaign_manager_eval::Endpoint::OpenAi),
        "openrouter" => Ok(animus_campaign_manager_eval::Endpoint::OpenRouter),
        other => Err(format!("unsupported endpoint {other}")),
    }
}

#[cfg(feature = "live")]
/// The single-line credential on standard input.
fn read_credential() -> Result<String, String> {
    /// Bytes a credential may take, plus one to detect overflow.
    const CREDENTIAL_BYTES: u64 = 4_097;
    let mut credential = String::new();
    std::io::Read::read_to_string(
        &mut std::io::Read::take(std::io::stdin().lock(), CREDENTIAL_BYTES),
        &mut credential,
    )
    .map_err(|_| String::from("credential_stdin_read_failed"))?;
    let trimmed = credential.trim_end_matches(['\r', '\n']);
    let is_single_line = !trimmed.is_empty() && !trimmed.contains(['\r', '\n']);
    if !is_single_line
        || u64::try_from(trimmed.len()).map_or(true, |bytes| bytes >= CREDENTIAL_BYTES)
    {
        return Err(String::from("credential_stdin_invalid"));
    }
    Ok(String::from(trimmed))
}

/// The metadata-only report of `finished`.
// r[impl agent_harness_runtime.campaign_manager_eval.report]
fn render(
    set: &animus_campaign_manager_eval::TaskSet,
    finished: &animus_campaign_manager_eval::JobReport,
) -> String {
    let receipt = &finished.receipt;
    let header = [
        String::from("CAMPAIGN_MANAGER_EVAL status=completed"),
        format!("job={}", receipt.job_identity),
        format!("profile={}", set.profile_id),
        format!(
            "planned={} completed={} failed={}",
            receipt.planned,
            receipt.completed,
            finished.failures.len()
        ),
    ];
    let footer = [
        format!("trajectories={}", receipt.trajectories.len()),
        String::from("prompts=omitted replies=omitted credential=omitted"),
        format!("non_claims={}", set.non_claims.join(",")),
        String::new(),
    ];
    let lines: Vec<String> = header
        .into_iter()
        .chain(finished.routes.iter().map(route_line))
        .chain(finished.failures.iter().map(failure_line))
        .chain(footer)
        .collect();
    debug_assert!(receipt.completed <= receipt.planned);
    debug_assert!(lines.iter().all(|line| !line.contains('\n')));
    lines.join("\n")
}

/// One route's completion count and mean rewards, advisory ones marked.
fn route_line(route: &animus_campaign_manager_eval::RouteSummary) -> String {
    let metrics: Vec<String> = route
        .metrics
        .iter()
        .map(|metric| {
            let marker = if metric.advisory_count > 0 {
                "(advisory)"
            } else {
                ""
            };
            format!("{}={:.3}{marker}", metric.metric, metric.mean)
        })
        .collect();
    debug_assert_eq!(metrics.len(), route.metrics.len());
    debug_assert!(route.completed > 0 || route.metrics.is_empty());
    format!(
        "route={} completed={} {}",
        route.route,
        route.completed,
        metrics.join(" ")
    )
}

/// One failed trial's task, route, and refusal class.
fn failure_line(failure: &animus_campaign_manager_eval::TrialFailure) -> String {
    format!(
        "failure task={} route={} error={:?}",
        failure.task, failure.route, failure.error
    )
}
