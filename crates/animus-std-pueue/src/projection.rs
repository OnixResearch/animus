use std::collections::BTreeMap;

use animus_adapters::{
    QueueEnqueueRequest, QueueGroupStatus, QueueGroupStatusDraft, QueueLogRequest,
    QueueLogViewport, QueueLogViewportDraft, QueueStatusCountDraft, QueueStatusCounts,
    QueueTaskObservation, QueueTaskObservationDraft, QueueTaskStatus,
};
use serde::Deserialize;

#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "bounded UTF-8 and line helpers keep pueue projection errors explicit"
    )
)]
mod text;

use text::{count_lines, tail_bytes, tail_lines};

const DEFAULT_GROUP: &str = "default";
const UNKNOWN_BACKEND_STATUS: &str = "Unknown";
const UNLIMITED_PARALLELISM_PROJECTION: u32 = u32::MAX;
const FULL_LOG_REFERENCE_PREFIX: &str = "pueue-job-";
const FULL_LOG_REFERENCE_SUFFIX: &str = "-log";

/// Stable pueue backend projection failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PueueProjectionError {
    /// Backend output was not valid JSON or UTF-8.
    InvalidEncoding,
    /// Backend JSON omitted required cohort fields.
    InvalidShape,
    /// A projected value violated an Animus queue contract.
    Contract,
    /// A backend count exceeded the supported projection range.
    BoundExceeded,
    /// The requested task was absent from backend output.
    MissingTask,
}

#[derive(Deserialize)]
struct StatusRoot {
    tasks: BTreeMap<String, BackendTask>,
    groups: BTreeMap<String, BackendGroup>,
}

#[derive(Clone, Deserialize)]
struct BackendTask {
    id: Option<u64>,
    group: Option<String>,
    status: serde_json::Value,
}

#[derive(Deserialize)]
struct BackendGroup {
    status: Option<String>,
    parallel_tasks: Option<u32>,
}

#[derive(Deserialize)]
struct BackendLogEntry {
    output: Option<String>,
}

pub(crate) fn enqueue_observation(
    output: &[u8],
    request: &QueueEnqueueRequest,
) -> Result<QueueTaskObservation, PueueProjectionError> {
    let task_id = std::str::from_utf8(output)
        .map_err(|_error| PueueProjectionError::InvalidEncoding)?
        .trim();
    if task_id.is_empty() || !task_id.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(PueueProjectionError::InvalidShape);
    }
    QueueTaskObservation::new(QueueTaskObservationDraft {
        task_id,
        group: request.group().unwrap_or(DEFAULT_GROUP),
        status: QueueTaskStatus::Pending,
        backend_status_label: Some("Queued"),
    })
    .map_err(|_error| PueueProjectionError::Contract)
}

pub(crate) fn task_observation(
    output: &[u8],
    task_id: &str,
) -> Result<QueueTaskObservation, PueueProjectionError> {
    let status = decode_status(output)?;
    let task = status
        .tasks
        .get(task_id)
        .or_else(|| {
            status
                .tasks
                .values()
                .find(|task| task_id_matches(task, task_id))
        })
        .ok_or(PueueProjectionError::MissingTask)?;
    project_task(task_id, task)
}

pub(crate) fn group_observations(
    output: &[u8],
) -> Result<Vec<QueueGroupStatus>, PueueProjectionError> {
    let status = decode_status(output)?;
    let mut counts = group_counts(&status.tasks);
    status
        .groups
        .iter()
        .map(|(name, group)| {
            let parallelism = match group.parallel_tasks.unwrap_or(1) {
                0 => UNLIMITED_PARALLELISM_PROJECTION,
                value => value,
            };
            let count = counts.remove(name).unwrap_or_default();
            let mut backend_labels = Vec::new();
            if let Some(label) = group.status.as_deref() {
                backend_labels.push(label);
            }
            if group.parallel_tasks == Some(0) {
                backend_labels.push("unlimited");
            }
            QueueGroupStatus::new(QueueGroupStatusDraft {
                group: name,
                parallelism_limit: parallelism,
                counts: count.into_queue_counts(),
                backend_labels: backend_labels.as_slice(),
            })
            .map_err(|_error| PueueProjectionError::Contract)
        })
        .collect()
}

pub(crate) fn log_viewport(
    output: &[u8],
    request: &QueueLogRequest,
) -> Result<QueueLogViewport, PueueProjectionError> {
    let text =
        std::str::from_utf8(output).map_err(|_error| PueueProjectionError::InvalidEncoding)?;
    let entries: BTreeMap<String, BackendLogEntry> =
        serde_json::from_str(text).map_err(|_error| PueueProjectionError::InvalidEncoding)?;
    let full_output = entries
        .get(request.task_id())
        .and_then(|entry| entry.output.as_deref())
        .ok_or(PueueProjectionError::MissingTask)?;
    build_log_viewport(full_output, request)
}

fn decode_status(output: &[u8]) -> Result<StatusRoot, PueueProjectionError> {
    let text =
        std::str::from_utf8(output).map_err(|_error| PueueProjectionError::InvalidEncoding)?;
    serde_json::from_str(text).map_err(|_error| PueueProjectionError::InvalidEncoding)
}

fn project_task(
    map_key: &str,
    task: &BackendTask,
) -> Result<QueueTaskObservation, PueueProjectionError> {
    let task_id = task
        .id
        .map_or_else(|| String::from(map_key), |id| id.to_string());
    let (status, backend_label) = project_status(&task.status);
    QueueTaskObservation::new(QueueTaskObservationDraft {
        task_id: task_id.as_str(),
        group: task.group.as_deref().unwrap_or(DEFAULT_GROUP),
        status,
        backend_status_label: Some(backend_label),
    })
    .map_err(|_error| PueueProjectionError::Contract)
}

fn project_status(value: &serde_json::Value) -> (QueueTaskStatus, &str) {
    let Some((variant, detail)) = value.as_object().and_then(|object| object.iter().next()) else {
        return (QueueTaskStatus::Failed, UNKNOWN_BACKEND_STATUS);
    };
    let status = match variant.to_ascii_lowercase().as_str() {
        "running" => QueueTaskStatus::Running,
        "paused" => QueueTaskStatus::Paused,
        "stashed" => QueueTaskStatus::Stashed,
        "done" => terminal_status(detail),
        "failed" | "dependencyfailed" => QueueTaskStatus::Failed,
        "killed" | "cancelled" => QueueTaskStatus::Cancelled,
        _other => QueueTaskStatus::Pending,
    };
    (status, variant.as_str())
}

fn terminal_status(value: &serde_json::Value) -> QueueTaskStatus {
    let result = value
        .as_object()
        .and_then(|object| object.get("result"))
        .map(serde_json::Value::to_string)
        .unwrap_or_default()
        .to_ascii_lowercase();
    if result.contains("cancel") || result.contains("kill") {
        QueueTaskStatus::Cancelled
    } else if !result.is_empty() && !result.contains("success") {
        QueueTaskStatus::Failed
    } else {
        QueueTaskStatus::Completed
    }
}

fn task_id_matches(task: &BackendTask, expected: &str) -> bool {
    task.id.is_some_and(|id| id.to_string() == expected)
}

#[derive(Clone, Copy, Default)]
struct MutableCounts {
    pending: u32,
    running: u32,
    paused: u32,
    stashed: u32,
    completed: u32,
}

impl MutableCounts {
    fn add(&mut self, status: QueueTaskStatus) {
        match status {
            QueueTaskStatus::Pending => self.pending = self.pending.saturating_add(1),
            QueueTaskStatus::Running => self.running = self.running.saturating_add(1),
            QueueTaskStatus::Paused => self.paused = self.paused.saturating_add(1),
            QueueTaskStatus::Stashed => self.stashed = self.stashed.saturating_add(1),
            QueueTaskStatus::Completed | QueueTaskStatus::Failed | QueueTaskStatus::Cancelled => {
                self.completed = self.completed.saturating_add(1);
            }
        }
    }

    const fn into_queue_counts(self) -> QueueStatusCounts {
        QueueStatusCounts::new(QueueStatusCountDraft {
            pending: self.pending,
            running: self.running,
            paused: self.paused,
            stashed: self.stashed,
            completed: self.completed,
        })
    }
}

fn group_counts(tasks: &BTreeMap<String, BackendTask>) -> BTreeMap<String, MutableCounts> {
    let mut counts: BTreeMap<String, MutableCounts> = BTreeMap::new();
    for task in tasks.values() {
        let group = task.group.as_deref().unwrap_or(DEFAULT_GROUP);
        let (status, _label) = project_status(&task.status);
        counts.entry(String::from(group)).or_default().add(status);
    }
    counts
}

fn build_log_viewport(
    full_output: &str,
    request: &QueueLogRequest,
) -> Result<QueueLogViewport, PueueProjectionError> {
    let bounds = request.bounds();
    let total_line_count = count_lines(full_output)?;
    let total_byte_count =
        u32::try_from(full_output.len()).map_err(|_error| PueueProjectionError::BoundExceeded)?;
    let line_tail = tail_lines(full_output, bounds.max_visible_lines())?;
    let visible_text = tail_bytes(line_tail.as_str(), bounds.max_visible_bytes())?;
    let visible_line_count = count_lines(visible_text.as_str())?;
    let visible_byte_count =
        u32::try_from(visible_text.len()).map_err(|_error| PueueProjectionError::BoundExceeded)?;
    let is_truncated =
        visible_line_count < total_line_count || visible_byte_count < total_byte_count;
    let full_output_hash = blake3::hash(full_output.as_bytes()).to_hex().to_string();
    let safe_reference = format!(
        "{FULL_LOG_REFERENCE_PREFIX}{}{FULL_LOG_REFERENCE_SUFFIX}",
        request.task_id()
    );
    QueueLogViewport::new(
        QueueLogViewportDraft {
            task_id: request.task_id(),
            visible_text: visible_text.as_str(),
            total_line_count,
            total_byte_count,
            visible_line_count,
            visible_byte_count,
            truncated: is_truncated,
            full_output_hash: Some(full_output_hash.as_str()),
            safe_reference: Some(safe_reference.as_str()),
        },
        bounds,
    )
    .map_err(|_error| PueueProjectionError::Contract)
}
