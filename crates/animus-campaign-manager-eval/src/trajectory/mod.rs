//! The ATIF-v1.7 trajectory of one trial, validated and projected to metadata through
//! `animus_runtime::harbor_eval::project_atif`.
//!
//! Each manager turn is an agent step with one `campaign.advise` call and its correlated review
//! observation. Each host step is a system step with one call: `campaign.execute`,
//! `campaign.wait`, or `campaign.stop`. Prompts, replies, and labels stay out of the projection.

/// Function of a manager turn's call.
const ADVISE: &str = "campaign.advise";
/// Function of a host step that ran a selection.
const EXECUTE: &str = "campaign.execute";
/// Function of a host step that found nothing to select.
const WAIT: &str = "campaign.wait";
/// Function of a host step that stopped the campaign.
const STOP: &str = "campaign.stop";

/// The trial's validated ATIF projection.
///
/// # Errors
///
/// Returns the `AtifError` of `project_atif`, such as `Bound` for a trial without turns.
// r[impl agent_harness_runtime.campaign_manager_eval.trajectory]
pub fn project_turns(
    trial: &str,
    turns: &[crate::TurnRecord],
) -> Result<animus_runtime::harbor_eval::AtifProjection, animus_runtime::harbor_eval::AtifError> {
    let calls = step_calls(turns);
    let tool_calls: Vec<[animus_runtime::harbor_eval::ToolCall<'_>; 1]> = calls
        .iter()
        .map(|(id, function, _)| {
            [animus_runtime::harbor_eval::ToolCall {
                id: id.as_str(),
                function,
            }]
        })
        .collect();
    let observations: Vec<[animus_runtime::harbor_eval::ObservationRef<'_>; 1]> = calls
        .iter()
        .map(|(id, _, _)| {
            [animus_runtime::harbor_eval::ObservationRef {
                source_call_id: Some(id.as_str()),
                subagent_trajectory_id: None,
            }]
        })
        .collect();
    let last = calls.len();
    let steps: Vec<animus_runtime::harbor_eval::AtifStep<'_>> = calls
        .iter()
        .zip(&tool_calls)
        .zip(&observations)
        .enumerate()
        .map(|(index, (((_, _, source), call), observation))| {
            let id = index.saturating_add(1);
            animus_runtime::harbor_eval::AtifStep {
                id,
                source: *source,
                calls: call,
                observations: observation,
                hidden_reasoning_omitted: false,
                terminal: id == last,
            }
        })
        .collect();
    let identity = format!("atif:{trial}");
    debug_assert_eq!(steps.len(), calls.len());
    debug_assert_eq!(tool_calls.len(), observations.len());
    animus_runtime::harbor_eval::project_atif(&animus_runtime::harbor_eval::AtifTrajectory {
        schema: animus_adapters::harbor::ATIF_SCHEMA,
        identity: &identity,
        steps: &steps,
        subagents: &[],
    })
}

/// Each step's call identity, function, and source, in order.
fn step_calls(
    turns: &[crate::TurnRecord],
) -> Vec<(
    String,
    &'static str,
    animus_runtime::harbor_eval::AtifSource,
)> {
    turns
        .iter()
        .flat_map(|record| {
            let advice = (
                format!("turn-{}", record.turn),
                ADVISE,
                animus_runtime::harbor_eval::AtifSource::Agent,
            );
            let host = record.action.map(|action| {
                (
                    format!("host-{}", record.turn),
                    host_function(action),
                    animus_runtime::harbor_eval::AtifSource::System,
                )
            });
            core::iter::once(advice).chain(host)
        })
        .collect()
}

/// The call function of a host step.
const fn host_function(action: crate::HostAction) -> &'static str {
    match action {
        crate::HostAction::Selected { .. } => EXECUTE,
        crate::HostAction::Waited => WAIT,
        crate::HostAction::Stopped => STOP,
    }
}
