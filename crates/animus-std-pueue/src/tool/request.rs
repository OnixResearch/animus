use animus_adapters::{
    QueueControlDraft, QueueControlRequest, QueueEnqueueDraft, QueueEnqueueRequest,
    QueueInteractionDraft, QueueInteractionKind, QueueInteractionRequest, QueueLogBounds,
    QueueLogRequest, QueueLogRequestDraft, QueueOperationKind, QueueServiceRequest,
    QueueTargetDraft, QueueTargetKind,
};
use serde::Deserialize;

use super::{
    PUEUE_TOOL_ARGUMENT_REQUEST, PUEUE_TOOL_CAPABILITY, PUEUE_TOOL_NAME,
    PUEUE_TOOL_SCHEMA_IDENTITY, PueueToolError,
};
use crate::PueueHostConfig;

#[derive(Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum Command {
    Enqueue {
        profile: String,
        group: Option<String>,
    },
    Poll {
        task_id: u64,
    },
    Wait {
        task_id: u64,
    },
    Logs {
        task_id: u64,
        max_lines: u32,
        max_bytes: u32,
    },
    Groups,
    Control {
        action: ControlAction,
        target: ControlTarget,
    },
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ControlAction {
    Pause,
    Resume,
    Restart,
    Cancel,
    Clean,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum ControlTarget {
    Task { task_id: u64 },
    Group { group: String },
    DefaultGroup,
    AllGroups,
}

#[derive(Clone, Copy)]
struct LogInput {
    task_id: u64,
    max_lines: u32,
    max_bytes: u32,
}

pub(super) fn decode(
    config: &PueueHostConfig,
    request: &animus_adapters::ToolRequest,
) -> Result<QueueServiceRequest, PueueToolError> {
    let is_exact = request.name() == PUEUE_TOOL_NAME
        && request.schema_identity() == PUEUE_TOOL_SCHEMA_IDENTITY
        && request.required_capability_labels() == [PUEUE_TOOL_CAPABILITY]
        && request.mutation_targets() == [config.namespace_id()]
        && request.approval_posture() == animus_adapters::tool::ApprovalPosture::PreApproved;
    if !is_exact {
        return Err(PueueToolError::InvalidRequest);
    }
    let value = request
        .arguments()
        .iter()
        .find(|field| field.name() == PUEUE_TOOL_ARGUMENT_REQUEST)
        .map(animus_adapters::tool::ArgumentField::value)
        .ok_or(PueueToolError::InvalidRequest)?;
    let command: Command =
        serde_json::from_str(value).map_err(|_error| PueueToolError::InvalidRequest)?;
    queue_request(config, command)
}

fn queue_request(
    config: &PueueHostConfig,
    command: Command,
) -> Result<QueueServiceRequest, PueueToolError> {
    match command {
        Command::Enqueue { profile, group } => enqueue(config, &profile, group.as_deref()),
        Command::Poll { task_id } => interaction(config, QueueInteractionKind::Poll, task_id),
        Command::Wait { task_id } => interaction(config, QueueInteractionKind::Wait, task_id),
        Command::Logs {
            task_id,
            max_lines,
            max_bytes,
        } => logs(LogInput {
            task_id,
            max_lines,
            max_bytes,
        }),
        Command::Groups => Ok(QueueServiceRequest::ListGroups),
        Command::Control { action, target } => control(action, &target),
    }
}

fn enqueue(
    config: &PueueHostConfig,
    profile: &str,
    group: Option<&str>,
) -> Result<QueueServiceRequest, PueueToolError> {
    let output_bounds = config
        .limits()
        .log_bounds()
        .map_err(|_error| PueueToolError::Contract)?;
    let request = QueueEnqueueRequest::new(QueueEnqueueDraft {
        command_label: profile,
        group,
        environment_policy: config.environment_policy.as_str(),
        timeout_policy: config.timeout_policy.as_str(),
        output_bounds,
    })
    .map_err(|_error| PueueToolError::InvalidRequest)?;
    Ok(QueueServiceRequest::Enqueue(request))
}

fn logs(input: LogInput) -> Result<QueueServiceRequest, PueueToolError> {
    let bounds = QueueLogBounds::new(input.max_lines, input.max_bytes)
        .map_err(|_error| PueueToolError::InvalidRequest)?;
    let task_id = input.task_id.to_string();
    let request = QueueLogRequest::new(QueueLogRequestDraft {
        task_id: task_id.as_str(),
        bounds,
    })
    .map_err(|_error| PueueToolError::InvalidRequest)?;
    Ok(QueueServiceRequest::ReadLog(request))
}

fn interaction(
    config: &PueueHostConfig,
    kind: QueueInteractionKind,
    task_id: u64,
) -> Result<QueueServiceRequest, PueueToolError> {
    let task_id = task_id.to_string();
    let request = QueueInteractionRequest::new(QueueInteractionDraft {
        kind,
        task_id: task_id.as_str(),
        timeout_policy: config.timeout_policy.as_str(),
        capabilities: config.capabilities(),
        input_hash: None,
    })
    .map_err(|_error| PueueToolError::InvalidRequest)?;
    Ok(QueueServiceRequest::Interaction(request))
}

fn control(
    action: ControlAction,
    target: &ControlTarget,
) -> Result<QueueServiceRequest, PueueToolError> {
    let operation = match action {
        ControlAction::Pause => QueueOperationKind::Pause,
        ControlAction::Resume => QueueOperationKind::Resume,
        ControlAction::Restart => QueueOperationKind::Restart,
        ControlAction::Cancel => QueueOperationKind::Cancel,
        ControlAction::Clean => QueueOperationKind::Clean,
    };
    let (kind, label) = match target {
        ControlTarget::Task { task_id } => (QueueTargetKind::Task, Some(task_id.to_string())),
        ControlTarget::Group { group } => (QueueTargetKind::Group, Some(group.clone())),
        ControlTarget::DefaultGroup => (QueueTargetKind::DefaultGroup, None),
        ControlTarget::AllGroups => (QueueTargetKind::AllGroups, None),
    };
    let request = QueueControlRequest::new(QueueControlDraft {
        operation,
        target: Some(QueueTargetDraft {
            kind,
            label: label.as_deref(),
        }),
    })
    .map_err(|_error| PueueToolError::InvalidRequest)?;
    Ok(QueueServiceRequest::Control(request))
}
