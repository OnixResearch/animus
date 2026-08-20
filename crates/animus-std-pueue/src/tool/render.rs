use animus_adapters::QueueServiceResponse;

use super::{PueueToolError, UNSAFE_OUTPUT_MARKERS};

pub(super) fn response(value: &QueueServiceResponse) -> Result<String, PueueToolError> {
    let value = match value {
        QueueServiceResponse::Task(task) => serde_json::json!({
            "kind": "task",
            "task_id": task.task_id(),
            "group": task.group(),
            "status": task.status().as_str(),
            "backend_status": task.backend_status_label(),
        }),
        QueueServiceResponse::ControlAccepted(operation) => serde_json::json!({
            "kind": "control",
            "operation": operation.as_str(),
            "accepted": true,
        }),
        QueueServiceResponse::InteractionAccepted(interaction) => serde_json::json!({
            "kind": "interaction",
            "interaction": format!("{interaction:?}").to_ascii_lowercase(),
            "accepted": true,
        }),
        QueueServiceResponse::Log(log) => serde_json::json!({
            "kind": "log",
            "task_id": log.task_id(),
            "visible_text": sanitize(log.visible_text()),
            "total_line_count": log.total_line_count(),
            "total_byte_count": log.total_byte_count(),
            "visible_line_count": log.visible_line_count(),
            "visible_byte_count": log.visible_byte_count(),
            "truncated": log.truncated(),
            "full_output_hash": log.full_output_hash().map(animus_adapters::SafeHash::as_str),
            "safe_reference": log.safe_reference(),
        }),
        QueueServiceResponse::Groups(groups) => serde_json::json!({
            "kind": "groups",
            "groups": groups.iter().map(group_value).collect::<Vec<_>>(),
        }),
    };
    serde_json::to_string(&value).map_err(|_error| PueueToolError::Contract)
}

fn group_value(group: &animus_adapters::QueueGroupStatus) -> serde_json::Value {
    let counts = group.counts();
    serde_json::json!({
        "group": group.group(),
        "parallelism_limit": group.parallelism_limit(),
        "counts": {
            "pending": counts.pending(),
            "running": counts.running(),
            "paused": counts.paused(),
            "stashed": counts.stashed(),
            "completed": counts.completed(),
        },
        "backend_labels": group.backend_labels(),
    })
}

fn sanitize(value: &str) -> String {
    UNSAFE_OUTPUT_MARKERS
        .iter()
        .fold(String::from(value), |text, marker| {
            text.replace(marker, "[redacted]")
        })
}
