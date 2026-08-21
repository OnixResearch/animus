//! Exact pueue control argument construction.

use animus_adapters::{QueueControlRequest, QueueOperationKind, QueueTargetKind};

use super::{DEFAULT_GROUP, PueueCommandError, base_args, validated_task_id};

pub(super) fn control_args(
    request: &QueueControlRequest,
) -> Result<Vec<String>, PueueCommandError> {
    let target = request.target().ok_or(PueueCommandError::Denied)?;
    if request.operation() == QueueOperationKind::SetGroupParallelism {
        return parallelism_args(target, request.parallelism());
    }
    let command = match request.operation() {
        QueueOperationKind::Pause => "pause",
        QueueOperationKind::Resume => "start",
        QueueOperationKind::Restart => "restart",
        QueueOperationKind::Cancel => "kill",
        QueueOperationKind::Clean => "clean",
        QueueOperationKind::Enqueue
        | QueueOperationKind::Status
        | QueueOperationKind::Wait
        | QueueOperationKind::Logs
        | QueueOperationKind::SendInput
        | QueueOperationKind::SetGroupParallelism => return Err(PueueCommandError::Denied),
    };
    if request.operation() == QueueOperationKind::Restart && target.kind() != QueueTargetKind::Task
    {
        return Err(PueueCommandError::Denied);
    }
    if request.operation() == QueueOperationKind::Clean {
        return clean_args(target);
    }
    let mut args = base_args(command);
    args.extend(target_args(target)?);
    Ok(args)
}

fn parallelism_args(
    target: &animus_adapters::QueueTarget,
    parallelism: Option<u32>,
) -> Result<Vec<String>, PueueCommandError> {
    let mut args = base_args("parallel");
    match target.kind() {
        QueueTargetKind::Group => args.extend([
            String::from("--group"),
            String::from(target.label().ok_or(PueueCommandError::Denied)?),
        ]),
        QueueTargetKind::DefaultGroup => {}
        QueueTargetKind::Task | QueueTargetKind::AllGroups => {
            return Err(PueueCommandError::Denied);
        }
    }
    args.push(parallelism.ok_or(PueueCommandError::Denied)?.to_string());
    Ok(args)
}

fn clean_args(target: &animus_adapters::QueueTarget) -> Result<Vec<String>, PueueCommandError> {
    let mut args = base_args("clean");
    match target.kind() {
        QueueTargetKind::Group => args.extend([
            String::from("--group"),
            String::from(target.label().ok_or(PueueCommandError::Denied)?),
        ]),
        QueueTargetKind::DefaultGroup => {
            args.extend([String::from("--group"), String::from(DEFAULT_GROUP)]);
        }
        QueueTargetKind::AllGroups => {}
        QueueTargetKind::Task => return Err(PueueCommandError::Denied),
    }
    Ok(args)
}

fn target_args(target: &animus_adapters::QueueTarget) -> Result<Vec<String>, PueueCommandError> {
    match target.kind() {
        QueueTargetKind::Task => Ok(vec![validated_task_id(
            target.label().ok_or(PueueCommandError::Denied)?,
        )?]),
        QueueTargetKind::Group => Ok(vec![
            String::from("--group"),
            String::from(target.label().ok_or(PueueCommandError::Denied)?),
        ]),
        QueueTargetKind::DefaultGroup => {
            Ok(vec![String::from("--group"), String::from(DEFAULT_GROUP)])
        }
        QueueTargetKind::AllGroups => Ok(vec![String::from("--all")]),
    }
}
