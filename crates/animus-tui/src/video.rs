//! Pure bounded terminal-video reduction and injected host playback services.
//!
//! The reducer receives explicit ticks, frame metadata, policy, capabilities,
//! commands, and prior state. It does not read clocks, terminals, files, URLs,
//! processes, networks, credentials, codecs, audio devices, or persistence.

use std::io::Write;

use super::pixel::{
    RATS_IMAGE_COHORT_REVISION, TerminalImageByteResolver, TerminalImageCapabilities,
    TerminalImageDigest, TerminalImageError, TerminalImageExecution, TerminalImageGeometry,
    TerminalImageHandle, TerminalImageIntent, TerminalImageLifecycleService, TerminalImageMetadata,
    TerminalImageOperationStatus, TerminalImagePlanInput, TerminalImagePolicy,
    TerminalImageProtocol, execute_terminal_image_plan, plan_terminal_image,
};

const BLAKE3_HEX_CHARS: usize = 64;
const MAX_SAFE_LABEL_CHARS: usize = 96;
const MIN_POSITIVE: u64 = 1;
const MILLIHERTZ_PER_HERTZ: u64 = 1_000;
const TICKS_PER_SECOND: u64 = 1_000_000;
const RECEIPT_SCHEMA: &str = "animus.terminal-video-receipt.v1";
const KITTY_CLEANUP_OBLIGATIONS: u16 = 2;
const RECEIPT_NON_CLAIM_COUNT: usize = 6;
const RECEIPT_DOMAIN: &[u8] = b"animus.terminal-video-receipt.v1\0";
const FORBIDDEN_EVIDENCE_TOKENS: &[&str] = &[
    "media_bytes",
    "frame_bytes",
    "base64",
    "data:",
    "\\x1b",
    "\u{1b}",
    "/home/",
    "/tmp/",
    "http://",
    "https://",
    "decoder_output",
    "credential",
    "term_program",
    "terminal_response",
    "timing_trace",
    "smooth_playback",
];

/// Safe playback identity. It is not a path, URL, provider ID, or credential.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoId(String);

impl TerminalVideoId {
    /// Validates a bounded token identity.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalVideoError::UnsafeIdentity`] for invalid input.
    pub fn try_new(value: &str) -> Result<Self, TerminalVideoError> {
        validate_safe_token(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Returns the safe identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Safe class of the injected decoded-frame source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalVideoSourceClass {
    /// Host-approved decoded frames.
    ApprovedDecodedFrames,
    /// Host-approved thumbnail frames.
    ApprovedThumbnailFrames,
}

impl TerminalVideoSourceClass {
    const fn label(self) -> &'static str {
        match self {
            Self::ApprovedDecodedFrames => "approved-decoded-frames",
            Self::ApprovedThumbnailFrames => "approved-thumbnail-frames",
        }
    }
}

/// Admitted playback descriptor. It contains no media locator or body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoDescriptor {
    /// Stable safe playback identity.
    pub playback_id: TerminalVideoId,
    /// Approved opaque frame-source handle.
    pub source_handle: TerminalImageHandle,
    /// Safe source class.
    pub source_class: TerminalVideoSourceClass,
    /// BLAKE3 identity for admitted source metadata.
    pub source_digest: TerminalImageDigest,
    /// Declared finite frame count.
    pub total_frames: u32,
    /// Declared frame rate in millihertz.
    pub frame_rate_millihertz: u32,
    /// Declared duration in explicit monotonic ticks.
    pub duration_ticks: u64,
    /// Terminal geometry supplied by the host layout reducer.
    pub geometry: TerminalImageGeometry,
}

/// Policy decision for one optional playback behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalVideoPolicyDecision {
    /// Policy admits the behavior.
    Allow,
    /// Policy denies the behavior.
    Deny,
}

impl TerminalVideoPolicyDecision {
    const fn allowed(self) -> bool {
        matches!(self, Self::Allow)
    }
}

/// Named playback bounds exported from Nickel policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalVideoPolicy {
    /// Maximum bytes in one decoded image frame.
    pub max_frame_bytes: u64,
    /// Maximum bytes in the admitted queue.
    pub max_buffered_bytes: u64,
    /// Maximum queued frames.
    pub max_queue_frames: u16,
    /// Maximum source width.
    pub max_frame_width: u32,
    /// Maximum source height.
    pub max_frame_height: u32,
    /// Maximum source pixels.
    pub max_frame_pixels: u64,
    /// Maximum declared frame count.
    pub max_frames: u32,
    /// Maximum frame rate in millihertz.
    pub max_frame_rate_millihertz: u32,
    /// Maximum declared duration in ticks.
    pub max_duration_ticks: u64,
    /// Maximum admitted late-frame drops.
    pub max_dropped_frames: u32,
    /// Maximum actions from one reduction.
    pub max_actions: u16,
    /// Maximum tolerated presentation lag in ticks.
    pub max_lag_ticks: u64,
    /// Maximum host frame-source deadline in ticks.
    pub max_frame_deadline_ticks: u64,
    /// Maximum cleanup obligations.
    pub max_cleanup_obligations: u16,
    /// Whether late or over-capacity frames can be dropped.
    pub frame_drop: TerminalVideoPolicyDecision,
    /// Whether static image replacement is allowed.
    pub static_replacement: TerminalVideoPolicyDecision,
    /// Whether thumbnail fallback is allowed.
    pub thumbnail_fallback: TerminalVideoPolicyDecision,
    /// Whether bounded text fallback is allowed.
    pub text_fallback: TerminalVideoPolicyDecision,
}

/// Support state for one exact playback operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalVideoCapabilityState {
    /// The host admits the operation.
    Supported,
    /// The host does not admit the operation.
    Unsupported,
}

impl TerminalVideoCapabilityState {
    const fn supported(self) -> bool {
        matches!(self, Self::Supported)
    }
}

/// Exact terminal-video capability facts supplied by the host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalVideoCapabilities {
    /// Selected terminal protocol row.
    pub protocol: TerminalImageProtocol,
    /// Static frame transfer is admitted for the selected protocol.
    pub frame_transfer: TerminalVideoCapabilityState,
    /// Kitty animation control is admitted.
    pub kitty_animation_control: TerminalVideoCapabilityState,
    /// Kitty animation placement is admitted.
    pub kitty_animation_placement: TerminalVideoCapabilityState,
    /// Kitty animation deletion is admitted.
    pub kitty_animation_deletion: TerminalVideoCapabilityState,
    /// Bounded static replacement is admitted.
    pub static_replacement: TerminalVideoCapabilityState,
    /// Thumbnail fallback is available from an injected source.
    pub thumbnail_fallback: TerminalVideoCapabilityState,
    /// Text fallback is available.
    pub text_fallback: TerminalVideoCapabilityState,
}

/// Exact selected playback mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalVideoBackendMode {
    /// Kitty frame transfer plus host-admitted animation lifecycle operations.
    KittyAnimation,
    /// Bounded static image replacement.
    StaticReplacement,
    /// One bounded host-supplied thumbnail.
    ThumbnailFallback,
    /// Bounded visible text.
    TextFallback,
    /// No admitted playback mode.
    Unsupported,
}

impl TerminalVideoBackendMode {
    const fn label(self) -> &'static str {
        match self {
            Self::KittyAnimation => "kitty-animation",
            Self::StaticReplacement => "static-replacement",
            Self::ThumbnailFallback => "thumbnail-fallback",
            Self::TextFallback => "text-fallback",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Selects one exact backend without inferring animation from static support.
#[must_use]
pub fn select_terminal_video_backend(
    capabilities: TerminalVideoCapabilities,
    policy: TerminalVideoPolicy,
) -> TerminalVideoBackendMode {
    let kitty_animation = capabilities.protocol == TerminalImageProtocol::Kitty
        && capabilities.frame_transfer.supported()
        && capabilities.kitty_animation_control.supported()
        && capabilities.kitty_animation_placement.supported()
        && capabilities.kitty_animation_deletion.supported();
    if kitty_animation {
        TerminalVideoBackendMode::KittyAnimation
    } else if capabilities.frame_transfer.supported()
        && capabilities.static_replacement.supported()
        && policy.static_replacement.allowed()
    {
        TerminalVideoBackendMode::StaticReplacement
    } else if capabilities.thumbnail_fallback.supported() && policy.thumbnail_fallback.allowed() {
        TerminalVideoBackendMode::ThumbnailFallback
    } else if capabilities.text_fallback.supported() && policy.text_fallback.allowed() {
        TerminalVideoBackendMode::TextFallback
    } else {
        TerminalVideoBackendMode::Unsupported
    }
}

/// Safe metadata for one decoded static frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoFrameMetadata {
    /// Zero-based frame index.
    pub frame_index: u32,
    /// Explicit presentation tick relative to playback start.
    pub presentation_tick: u64,
    /// Static image metadata for the decoded frame.
    pub image: TerminalImageMetadata,
}

/// Playback lifecycle state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalVideoStatus {
    /// Playback has not started.
    Idle,
    /// Playback is requesting and presenting frames.
    Running,
    /// Playback is paused.
    Paused,
    /// Exact cleanup obligations are pending.
    CleanupPending,
    /// Playback finished and cleanup completed.
    Finished,
    /// Playback was cancelled and cleanup completed.
    Cancelled,
    /// Playback failed and cleanup completed.
    Failed,
    /// Cleanup failed.
    CleanupIncomplete,
    /// No backend was admitted.
    Unsupported,
}

impl TerminalVideoStatus {
    const fn label(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::CleanupPending => "cleanup-pending",
            Self::Finished => "finished",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::CleanupIncomplete => "cleanup-incomplete",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Pure bounded playback state. It retains no frame body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoState {
    /// Playback identity.
    pub playback_id: TerminalVideoId,
    /// Current status.
    pub status: TerminalVideoStatus,
    /// Selected backend.
    pub backend: TerminalVideoBackendMode,
    /// Next expected frame index.
    pub next_frame_index: u32,
    /// Presented frame count.
    pub displayed_frames: u32,
    /// Dropped frame count.
    pub dropped_frames: u32,
    /// Last admitted monotonic tick.
    pub last_tick: u64,
    /// Playback start tick.
    pub start_tick: u64,
    /// Remaining exact cleanup obligations.
    pub cleanup_obligations: u16,
    /// Final status after successful cleanup.
    pub status_after_cleanup: Option<TerminalVideoStatus>,
    /// Whether the selected backend allocated terminal state.
    pub terminal_state_owned: bool,
}

impl TerminalVideoState {
    /// Creates an idle state for one admitted identity.
    #[must_use]
    pub fn idle(playback_id: TerminalVideoId) -> Self {
        Self {
            playback_id,
            status: TerminalVideoStatus::Idle,
            backend: TerminalVideoBackendMode::Unsupported,
            next_frame_index: 0,
            displayed_frames: 0,
            dropped_frames: 0,
            last_tick: 0,
            start_tick: 0,
            cleanup_obligations: 0,
            status_after_cleanup: None,
            terminal_state_owned: false,
        }
    }
}

/// Explicit lifecycle command supplied to the pure reducer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalVideoCommand {
    /// Start at one supplied monotonic tick.
    Start {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Admit one decoded frame metadata row and queue observation.
    FrameReady {
        /// Supplied frame metadata.
        frame: TerminalVideoFrameMetadata,
        /// Current supplied monotonic tick.
        tick: u64,
        /// Host-observed queued frames before admission.
        queued_frames: u16,
        /// Host-observed queued bytes before admission.
        queued_bytes: u64,
    },
    /// Advance with an explicit tick and no frame body.
    Tick {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Pause playback.
    Pause {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Resume playback.
    Resume {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Cancel playback.
    Cancel {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Report source completion.
    SourceFinished {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Report a typed source failure.
    SourceFailed {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Report terminal capability or writer loss.
    TerminalLost {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Report layout eviction.
    LayoutEvicted {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Report host shutdown.
    Shutdown {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Report one successful cleanup obligation.
    CleanupCompleted {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Report cleanup failure.
    CleanupFailed {
        /// Supplied monotonic tick.
        tick: u64,
    },
}

/// Host backend operation named by a pure action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalVideoBackendOperation {
    /// Allocate exact Kitty animation state.
    InitializeAnimation,
    /// Attach one transmitted frame to an admitted Kitty animation.
    PresentAnimationFrame,
    /// Pause Kitty animation.
    PauseAnimation,
    /// Resume Kitty animation.
    ResumeAnimation,
    /// Stop Kitty animation.
    StopAnimation,
    /// Replace one static terminal image.
    ReplaceStaticFrame,
    /// Delete allocated image state.
    DeleteImage,
    /// Delete allocated animation state.
    DeleteAnimation,
    /// Render one host-supplied thumbnail.
    RenderThumbnail,
    /// Render bounded text fallback.
    RenderTextFallback,
}

/// Pure effect intent returned by playback reduction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalVideoAction {
    /// Request exactly one frame under a deadline.
    RequestFrame {
        /// Requested frame index.
        frame_index: u32,
        /// Absolute supplied-tick deadline.
        deadline_tick: u64,
    },
    /// Wait until an explicit supplied tick.
    WaitUntil {
        /// Supplied monotonic tick.
        tick: u64,
    },
    /// Present one admitted frame through the selected mode.
    DisplayFrame {
        /// Frame metadata. No frame body is retained.
        frame: TerminalVideoFrameMetadata,
        /// Exact host backend operation after transfer.
        operation: TerminalVideoBackendOperation,
    },
    /// Drop one frame under explicit policy.
    DropFrame {
        /// Dropped frame index.
        frame_index: u32,
    },
    /// Pause frame-source delivery.
    PauseSource,
    /// Resume frame-source delivery.
    ResumeSource,
    /// Stop new frame requests.
    StopSource,
    /// Perform one backend lifecycle operation.
    Backend(TerminalVideoBackendOperation),
    /// Complete playback after cleanup.
    Complete(TerminalVideoStatus),
    /// Record an unsupported backend outcome.
    Unsupported,
}

/// Result of one pure reduction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoReduction {
    /// Updated bounded state.
    pub state: TerminalVideoState,
    /// Ordered bounded actions.
    pub actions: Vec<TerminalVideoAction>,
}

/// Inputs to one pure reduction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoReduceInput {
    /// Admitted descriptor.
    pub descriptor: TerminalVideoDescriptor,
    /// Exact terminal capability facts.
    pub capabilities: TerminalVideoCapabilities,
    /// Named policy.
    pub policy: TerminalVideoPolicy,
    /// Prior state.
    pub state: TerminalVideoState,
    /// Explicit lifecycle command.
    pub command: TerminalVideoCommand,
}

/// Closed playback and shell errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalVideoError {
    /// Safe identity validation failed.
    UnsafeIdentity,
    /// Descriptor fields were invalid or exceeded policy.
    InvalidDescriptor,
    /// Policy contained zero or inconsistent limits.
    InvalidPolicy,
    /// Command did not match the playback identity or current state.
    InvalidTransition,
    /// Supplied monotonic tick moved backward.
    StaleTick,
    /// Frame index was duplicate or out of order.
    FrameOrderMismatch,
    /// Frame timing did not match the admitted schedule.
    FrameTimingMismatch,
    /// One frame exceeded byte or geometry bounds.
    FrameBoundExceeded,
    /// Queue observations exceeded policy and dropping was denied.
    QueueBoundExceeded,
    /// Dropped-frame policy was exhausted.
    DropBoundExceeded,
    /// One reduction exceeded its action bound.
    ActionBoundExceeded,
    /// Required injected frame source was unavailable.
    MissingFrameSource,
    /// Injected frame source timed out.
    FrameSourceTimeout,
    /// Injected frame source returned a typed failure.
    FrameSourceFailed,
    /// Frame bytes did not match admitted metadata.
    FrameIdentityMismatch,
    /// Terminal image planning or writing failed.
    TerminalImageFailed,
    /// Injected backend operation failed.
    BackendOperationFailed,
    /// Cleanup failed.
    CleanupFailed,
    /// Receipt candidate leaked forbidden content or authority.
    EvidenceLeak,
}

/// Runs one deterministic bounded playback reduction.
///
/// # Errors
///
/// Returns a typed error for invalid policy, descriptor, ticks, frame order,
/// bounds, queue pressure, transition, or action count.
pub fn reduce_terminal_video(
    input: TerminalVideoReduceInput,
) -> Result<TerminalVideoReduction, TerminalVideoError> {
    validate_video_policy(input.policy)?;
    validate_descriptor(&input.descriptor, input.policy)?;
    if input.state.playback_id != input.descriptor.playback_id {
        return Err(TerminalVideoError::InvalidTransition);
    }
    let mut context = ReductionContext {
        descriptor: &input.descriptor,
        capabilities: input.capabilities,
        policy: input.policy,
        state: input.state,
        actions: Vec::new(),
    };
    apply_video_command(&mut context, input.command)?;
    enforce_action_bound(&context.actions, context.policy)?;
    Ok(TerminalVideoReduction {
        state: context.state,
        actions: context.actions,
    })
}

struct ReductionContext<'a> {
    descriptor: &'a TerminalVideoDescriptor,
    capabilities: TerminalVideoCapabilities,
    policy: TerminalVideoPolicy,
    state: TerminalVideoState,
    actions: Vec<TerminalVideoAction>,
}

struct ReadyFrameInput {
    frame: TerminalVideoFrameMetadata,
    tick: u64,
    queued_frames: u16,
    queued_bytes: u64,
}

fn apply_video_command(
    context: &mut ReductionContext<'_>,
    command: TerminalVideoCommand,
) -> Result<(), TerminalVideoError> {
    match command {
        TerminalVideoCommand::Start { tick } => start_playback(context, tick),
        TerminalVideoCommand::FrameReady {
            frame,
            tick,
            queued_frames,
            queued_bytes,
        } => admit_ready_frame(
            context,
            ReadyFrameInput {
                frame,
                tick,
                queued_frames,
                queued_bytes,
            },
        ),
        TerminalVideoCommand::Tick { tick } => tick_playback(context, tick),
        TerminalVideoCommand::Pause { tick } => pause_playback(context, tick),
        TerminalVideoCommand::Resume { tick } => resume_playback(context, tick),
        TerminalVideoCommand::Cancel { tick } => {
            start_terminal_cleanup(context, tick, TerminalVideoStatus::Cancelled)
        }
        TerminalVideoCommand::SourceFinished { tick } => finish_source(context, tick),
        TerminalVideoCommand::SourceFailed { tick }
        | TerminalVideoCommand::TerminalLost { tick }
        | TerminalVideoCommand::LayoutEvicted { tick }
        | TerminalVideoCommand::Shutdown { tick } => {
            start_terminal_cleanup(context, tick, TerminalVideoStatus::Failed)
        }
        TerminalVideoCommand::CleanupCompleted { tick } => complete_cleanup(context, tick),
        TerminalVideoCommand::CleanupFailed { tick } => fail_cleanup(context, tick),
    }
}

fn start_playback(context: &mut ReductionContext<'_>, tick: u64) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::Idle)?;
    context.state.start_tick = tick;
    context.state.last_tick = tick;
    context.state.backend = select_terminal_video_backend(context.capabilities, context.policy);
    match context.state.backend {
        TerminalVideoBackendMode::KittyAnimation => {
            context.state.status = TerminalVideoStatus::Running;
            context.state.terminal_state_owned = true;
            context.actions.push(TerminalVideoAction::Backend(
                TerminalVideoBackendOperation::InitializeAnimation,
            ));
            push_frame_request(&mut context.actions, &context.state, context.policy)
        }
        TerminalVideoBackendMode::StaticReplacement => {
            context.state.status = TerminalVideoStatus::Running;
            push_frame_request(&mut context.actions, &context.state, context.policy)
        }
        TerminalVideoBackendMode::ThumbnailFallback => {
            finish_with_fallback(context, TerminalVideoBackendOperation::RenderThumbnail);
            Ok(())
        }
        TerminalVideoBackendMode::TextFallback => {
            finish_with_fallback(context, TerminalVideoBackendOperation::RenderTextFallback);
            Ok(())
        }
        TerminalVideoBackendMode::Unsupported => {
            context.state.status = TerminalVideoStatus::Unsupported;
            context.actions.push(TerminalVideoAction::Unsupported);
            Ok(())
        }
    }
}

fn finish_with_fallback(
    context: &mut ReductionContext<'_>,
    operation: TerminalVideoBackendOperation,
) {
    context.state.status = TerminalVideoStatus::Finished;
    context
        .actions
        .push(TerminalVideoAction::Backend(operation));
    context
        .actions
        .push(TerminalVideoAction::Complete(TerminalVideoStatus::Finished));
}

fn admit_ready_frame(
    context: &mut ReductionContext<'_>,
    input: ReadyFrameInput,
) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::Running)?;
    admit_tick(&mut context.state, input.tick)?;
    validate_frame(
        &input.frame,
        context.descriptor,
        context.policy,
        context.state.next_frame_index,
    )?;
    let queue_exceeded = input.queued_frames >= context.policy.max_queue_frames
        || input
            .queued_bytes
            .checked_add(input.frame.image.byte_count)
            .is_none_or(|total| total > context.policy.max_buffered_bytes);
    let target_tick = context
        .state
        .start_tick
        .checked_add(input.frame.presentation_tick)
        .ok_or(TerminalVideoError::FrameTimingMismatch)?;
    let late = input.tick.saturating_sub(target_tick) > context.policy.max_lag_ticks;
    if queue_exceeded || late {
        drop_frame(
            &mut context.state,
            input.frame.frame_index,
            context.policy,
            &mut context.actions,
        )?;
        return request_next_if_present(
            &mut context.actions,
            &context.state,
            context.descriptor,
            context.policy,
        );
    }
    if input.tick < target_tick {
        context
            .actions
            .push(TerminalVideoAction::WaitUntil { tick: target_tick });
        return Ok(());
    }
    display_ready_frame(context, input.frame)
}

fn display_ready_frame(
    context: &mut ReductionContext<'_>,
    frame: TerminalVideoFrameMetadata,
) -> Result<(), TerminalVideoError> {
    let operation = match context.state.backend {
        TerminalVideoBackendMode::KittyAnimation => {
            TerminalVideoBackendOperation::PresentAnimationFrame
        }
        TerminalVideoBackendMode::StaticReplacement => {
            TerminalVideoBackendOperation::ReplaceStaticFrame
        }
        _ => return Err(TerminalVideoError::InvalidTransition),
    };
    context.state.displayed_frames = context
        .state
        .displayed_frames
        .checked_add(1)
        .ok_or(TerminalVideoError::FrameBoundExceeded)?;
    context.state.next_frame_index = context
        .state
        .next_frame_index
        .checked_add(1)
        .ok_or(TerminalVideoError::FrameOrderMismatch)?;
    context.state.terminal_state_owned = true;
    context
        .actions
        .push(TerminalVideoAction::DisplayFrame { frame, operation });
    request_next_if_present(
        &mut context.actions,
        &context.state,
        context.descriptor,
        context.policy,
    )
}

fn tick_playback(context: &mut ReductionContext<'_>, tick: u64) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::Running)?;
    admit_tick(&mut context.state, tick)?;
    context
        .actions
        .push(TerminalVideoAction::WaitUntil { tick });
    Ok(())
}

fn pause_playback(context: &mut ReductionContext<'_>, tick: u64) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::Running)?;
    admit_tick(&mut context.state, tick)?;
    context.state.status = TerminalVideoStatus::Paused;
    context.actions.push(TerminalVideoAction::PauseSource);
    if context.state.backend == TerminalVideoBackendMode::KittyAnimation {
        context.actions.push(TerminalVideoAction::Backend(
            TerminalVideoBackendOperation::PauseAnimation,
        ));
    }
    Ok(())
}

fn resume_playback(
    context: &mut ReductionContext<'_>,
    tick: u64,
) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::Paused)?;
    admit_tick(&mut context.state, tick)?;
    context.state.status = TerminalVideoStatus::Running;
    context.actions.push(TerminalVideoAction::ResumeSource);
    if context.state.backend == TerminalVideoBackendMode::KittyAnimation {
        context.actions.push(TerminalVideoAction::Backend(
            TerminalVideoBackendOperation::ResumeAnimation,
        ));
    }
    push_frame_request(&mut context.actions, &context.state, context.policy)
}

fn finish_source(context: &mut ReductionContext<'_>, tick: u64) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::Running)?;
    admit_tick(&mut context.state, tick)?;
    if context.state.next_frame_index != context.descriptor.total_frames {
        return Err(TerminalVideoError::FrameOrderMismatch);
    }
    begin_cleanup(
        &mut context.state,
        TerminalVideoStatus::Finished,
        context.policy,
        &mut context.actions,
    )
}

fn start_terminal_cleanup(
    context: &mut ReductionContext<'_>,
    tick: u64,
    completed_status: TerminalVideoStatus,
) -> Result<(), TerminalVideoError> {
    admit_tick(&mut context.state, tick)?;
    begin_cleanup(
        &mut context.state,
        completed_status,
        context.policy,
        &mut context.actions,
    )
}

fn complete_cleanup(
    context: &mut ReductionContext<'_>,
    tick: u64,
) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::CleanupPending)?;
    admit_tick(&mut context.state, tick)?;
    context.state.cleanup_obligations = context
        .state
        .cleanup_obligations
        .checked_sub(1)
        .ok_or(TerminalVideoError::InvalidTransition)?;
    if context.state.cleanup_obligations == 0 {
        let completed = context
            .state
            .status_after_cleanup
            .take()
            .ok_or(TerminalVideoError::InvalidTransition)?;
        context.state.status = completed;
        context.state.terminal_state_owned = false;
        context
            .actions
            .push(TerminalVideoAction::Complete(completed));
    }
    Ok(())
}

fn fail_cleanup(context: &mut ReductionContext<'_>, tick: u64) -> Result<(), TerminalVideoError> {
    require_status(context.state.status, TerminalVideoStatus::CleanupPending)?;
    admit_tick(&mut context.state, tick)?;
    context.state.status = TerminalVideoStatus::CleanupIncomplete;
    context.actions.push(TerminalVideoAction::Complete(
        TerminalVideoStatus::CleanupIncomplete,
    ));
    Ok(())
}

fn validate_video_policy(policy: TerminalVideoPolicy) -> Result<(), TerminalVideoError> {
    let valid = policy.max_frame_bytes >= MIN_POSITIVE
        && policy.max_buffered_bytes >= policy.max_frame_bytes
        && u64::from(policy.max_queue_frames) >= MIN_POSITIVE
        && u64::from(policy.max_frame_width) >= MIN_POSITIVE
        && u64::from(policy.max_frame_height) >= MIN_POSITIVE
        && policy.max_frame_pixels >= u64::from(policy.max_frame_width)
        && u64::from(policy.max_frames) >= MIN_POSITIVE
        && u64::from(policy.max_frame_rate_millihertz) >= MIN_POSITIVE
        && policy.max_duration_ticks >= MIN_POSITIVE
        && u64::from(policy.max_actions) >= MIN_POSITIVE
        && policy.max_frame_deadline_ticks >= MIN_POSITIVE
        && u64::from(policy.max_cleanup_obligations) >= MIN_POSITIVE;
    if valid {
        Ok(())
    } else {
        Err(TerminalVideoError::InvalidPolicy)
    }
}

fn validate_descriptor(
    descriptor: &TerminalVideoDescriptor,
    policy: TerminalVideoPolicy,
) -> Result<(), TerminalVideoError> {
    if descriptor.total_frames == 0
        || descriptor.total_frames > policy.max_frames
        || descriptor.frame_rate_millihertz == 0
        || descriptor.frame_rate_millihertz > policy.max_frame_rate_millihertz
        || descriptor.duration_ticks == 0
        || descriptor.duration_ticks > policy.max_duration_ticks
        || descriptor.geometry.width == 0
        || descriptor.geometry.height == 0
    {
        return Err(TerminalVideoError::InvalidDescriptor);
    }
    let interval = frame_interval_ticks(descriptor.frame_rate_millihertz)?;
    let last_frame_tick = u64::from(descriptor.total_frames.saturating_sub(1))
        .checked_mul(interval)
        .ok_or(TerminalVideoError::InvalidDescriptor)?;
    let terminal_cells = u64::from(descriptor.geometry.width)
        .checked_mul(u64::from(descriptor.geometry.height))
        .ok_or(TerminalVideoError::InvalidDescriptor)?;
    if last_frame_tick > descriptor.duration_ticks
        || u32::from(descriptor.geometry.width) > policy.max_frame_width
        || u32::from(descriptor.geometry.height) > policy.max_frame_height
        || terminal_cells > policy.max_frame_pixels
    {
        return Err(TerminalVideoError::InvalidDescriptor);
    }
    Ok(())
}

fn validate_frame(
    frame: &TerminalVideoFrameMetadata,
    descriptor: &TerminalVideoDescriptor,
    policy: TerminalVideoPolicy,
    expected_index: u32,
) -> Result<(), TerminalVideoError> {
    if frame.frame_index != expected_index || frame.frame_index >= descriptor.total_frames {
        return Err(TerminalVideoError::FrameOrderMismatch);
    }
    let interval = frame_interval_ticks(descriptor.frame_rate_millihertz)?;
    let expected_tick = u64::from(frame.frame_index)
        .checked_mul(interval)
        .ok_or(TerminalVideoError::FrameTimingMismatch)?;
    if frame.presentation_tick != expected_tick {
        return Err(TerminalVideoError::FrameTimingMismatch);
    }
    let pixels = u64::from(frame.image.source_width)
        .checked_mul(u64::from(frame.image.source_height))
        .ok_or(TerminalVideoError::FrameBoundExceeded)?;
    if frame.image.byte_count == 0
        || frame.image.byte_count > policy.max_frame_bytes
        || frame.image.source_width == 0
        || frame.image.source_width > policy.max_frame_width
        || frame.image.source_height == 0
        || frame.image.source_height > policy.max_frame_height
        || pixels > policy.max_frame_pixels
    {
        return Err(TerminalVideoError::FrameBoundExceeded);
    }
    Ok(())
}

fn frame_interval_ticks(frame_rate_millihertz: u32) -> Result<u64, TerminalVideoError> {
    TICKS_PER_SECOND
        .checked_mul(MILLIHERTZ_PER_HERTZ)
        .and_then(|ticks| ticks.checked_div(u64::from(frame_rate_millihertz)))
        .filter(|interval| *interval > 0)
        .ok_or(TerminalVideoError::InvalidDescriptor)
}

fn require_status(
    actual: TerminalVideoStatus,
    required: TerminalVideoStatus,
) -> Result<(), TerminalVideoError> {
    if actual == required {
        Ok(())
    } else {
        Err(TerminalVideoError::InvalidTransition)
    }
}

fn admit_tick(state: &mut TerminalVideoState, tick: u64) -> Result<(), TerminalVideoError> {
    if tick < state.last_tick {
        return Err(TerminalVideoError::StaleTick);
    }
    state.last_tick = tick;
    Ok(())
}

fn push_frame_request(
    actions: &mut Vec<TerminalVideoAction>,
    state: &TerminalVideoState,
    policy: TerminalVideoPolicy,
) -> Result<(), TerminalVideoError> {
    let deadline_tick = state
        .last_tick
        .checked_add(policy.max_frame_deadline_ticks)
        .ok_or(TerminalVideoError::ActionBoundExceeded)?;
    actions.push(TerminalVideoAction::RequestFrame {
        frame_index: state.next_frame_index,
        deadline_tick,
    });
    Ok(())
}

fn request_next_if_present(
    actions: &mut Vec<TerminalVideoAction>,
    state: &TerminalVideoState,
    descriptor: &TerminalVideoDescriptor,
    policy: TerminalVideoPolicy,
) -> Result<(), TerminalVideoError> {
    if state.next_frame_index < descriptor.total_frames {
        push_frame_request(actions, state, policy)?;
    }
    Ok(())
}

fn drop_frame(
    state: &mut TerminalVideoState,
    frame_index: u32,
    policy: TerminalVideoPolicy,
    actions: &mut Vec<TerminalVideoAction>,
) -> Result<(), TerminalVideoError> {
    if !policy.frame_drop.allowed() {
        return Err(TerminalVideoError::QueueBoundExceeded);
    }
    let next_drop = state
        .dropped_frames
        .checked_add(1)
        .ok_or(TerminalVideoError::DropBoundExceeded)?;
    if next_drop > policy.max_dropped_frames {
        return Err(TerminalVideoError::DropBoundExceeded);
    }
    state.dropped_frames = next_drop;
    state.next_frame_index = state
        .next_frame_index
        .checked_add(1)
        .ok_or(TerminalVideoError::FrameOrderMismatch)?;
    actions.push(TerminalVideoAction::DropFrame { frame_index });
    Ok(())
}

fn begin_cleanup(
    state: &mut TerminalVideoState,
    completed_status: TerminalVideoStatus,
    policy: TerminalVideoPolicy,
    actions: &mut Vec<TerminalVideoAction>,
) -> Result<(), TerminalVideoError> {
    if !matches!(
        state.status,
        TerminalVideoStatus::Running | TerminalVideoStatus::Paused
    ) {
        return Err(TerminalVideoError::InvalidTransition);
    }
    actions.push(TerminalVideoAction::StopSource);
    let mut obligations = 0_u16;
    if state.terminal_state_owned {
        match state.backend {
            TerminalVideoBackendMode::KittyAnimation => {
                actions.push(TerminalVideoAction::Backend(
                    TerminalVideoBackendOperation::StopAnimation,
                ));
                actions.push(TerminalVideoAction::Backend(
                    TerminalVideoBackendOperation::DeleteAnimation,
                ));
                actions.push(TerminalVideoAction::Backend(
                    TerminalVideoBackendOperation::DeleteImage,
                ));
                obligations = KITTY_CLEANUP_OBLIGATIONS;
            }
            TerminalVideoBackendMode::StaticReplacement => {
                actions.push(TerminalVideoAction::Backend(
                    TerminalVideoBackendOperation::DeleteImage,
                ));
                obligations = 1;
            }
            TerminalVideoBackendMode::ThumbnailFallback
            | TerminalVideoBackendMode::TextFallback
            | TerminalVideoBackendMode::Unsupported => {}
        }
    }
    if obligations > policy.max_cleanup_obligations {
        return Err(TerminalVideoError::ActionBoundExceeded);
    }
    if obligations == 0 {
        state.status = completed_status;
        state.terminal_state_owned = false;
        actions.push(TerminalVideoAction::Complete(completed_status));
    } else {
        state.status = TerminalVideoStatus::CleanupPending;
        state.cleanup_obligations = obligations;
        state.status_after_cleanup = Some(completed_status);
    }
    Ok(())
}

fn enforce_action_bound(
    actions: &[TerminalVideoAction],
    policy: TerminalVideoPolicy,
) -> Result<(), TerminalVideoError> {
    let count =
        u16::try_from(actions.len()).map_err(|_| TerminalVideoError::ActionBoundExceeded)?;
    if count > policy.max_actions {
        Err(TerminalVideoError::ActionBoundExceeded)
    } else {
        Ok(())
    }
}

fn validate_safe_token(value: &str) -> Result<(), TerminalVideoError> {
    if value.is_empty()
        || value.chars().count() > MAX_SAFE_LABEL_CHARS
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        Err(TerminalVideoError::UnsafeIdentity)
    } else {
        Ok(())
    }
}

/// Transient decoded-frame packet returned by an injected host source.
///
/// Callers must consume and release `bytes`; durable playback state stores only
/// the packet metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoFramePacket {
    /// Safe frame metadata.
    pub metadata: TerminalVideoFrameMetadata,
    /// One bounded decoded static-image body.
    pub bytes: Vec<u8>,
}

/// Bounded request passed to an injected frame source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoFrameSourceRequest {
    /// Approved opaque source handle.
    pub source_handle: TerminalImageHandle,
    /// Requested frame index.
    pub frame_index: u32,
    /// Maximum returned bytes.
    pub max_bytes: u64,
    /// Absolute supplied-tick deadline.
    pub deadline_tick: u64,
}

/// Frame index and deadline selected by the playback core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalVideoFrameFetch {
    /// Requested frame index.
    pub frame_index: u32,
    /// Absolute supplied-tick deadline.
    pub deadline_tick: u64,
}

/// Host-owned decoded-frame source.
pub trait TerminalVideoFrameSource {
    /// Fetches one exact frame under byte and deadline bounds.
    ///
    /// # Errors
    ///
    /// Returns a typed source error without exposing decoder output.
    fn fetch_frame(
        &mut self,
        request: &TerminalVideoFrameSourceRequest,
    ) -> Result<TerminalVideoFramePacket, TerminalVideoError>;
}

/// Fetches and rechecks one transient decoded frame through an injected source.
///
/// # Errors
///
/// Returns a typed error for a source failure, index mismatch, byte bound, or
/// content-identity mismatch.
pub fn fetch_terminal_video_frame<S: TerminalVideoFrameSource>(
    source: &mut S,
    descriptor: &TerminalVideoDescriptor,
    fetch: TerminalVideoFrameFetch,
    policy: TerminalVideoPolicy,
) -> Result<TerminalVideoFramePacket, TerminalVideoError> {
    let request = TerminalVideoFrameSourceRequest {
        source_handle: descriptor.source_handle.clone(),
        frame_index: fetch.frame_index,
        max_bytes: policy.max_frame_bytes,
        deadline_tick: fetch.deadline_tick,
    };
    let packet = source.fetch_frame(&request)?;
    if packet.metadata.frame_index != fetch.frame_index {
        return Err(TerminalVideoError::FrameOrderMismatch);
    }
    let byte_count =
        u64::try_from(packet.bytes.len()).map_err(|_| TerminalVideoError::FrameBoundExceeded)?;
    if byte_count == 0 || byte_count > policy.max_frame_bytes {
        return Err(TerminalVideoError::FrameBoundExceeded);
    }
    if packet.metadata.image.byte_count != byte_count
        || TerminalImageDigest::from_bytes(&packet.bytes) != packet.metadata.image.content_digest
    {
        return Err(TerminalVideoError::FrameIdentityMismatch);
    }
    validate_frame(&packet.metadata, descriptor, policy, fetch.frame_index)?;
    Ok(packet)
}

/// Metadata-only call passed to the injected playback backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoBackendCall {
    /// Exact backend operation.
    pub operation: TerminalVideoBackendOperation,
    /// Safe playback identity.
    pub playback_id: TerminalVideoId,
    /// Optional frame index.
    pub frame_index: Option<u32>,
    /// Host-supplied terminal geometry.
    pub geometry: TerminalImageGeometry,
}

/// Host-owned terminal-video lifecycle service.
pub trait TerminalVideoBackendService {
    /// Executes one metadata-only exact backend operation.
    ///
    /// # Errors
    ///
    /// Returns a typed failure when the operation is denied, unsupported, or
    /// incomplete.
    fn execute_backend(
        &mut self,
        call: &TerminalVideoBackendCall,
    ) -> Result<(), TerminalVideoError>;
}

/// Metadata-only result of one frame display shell call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoFrameExecution {
    /// Frame index attempted.
    pub frame_index: u32,
    /// Terminal-image execution result.
    pub image_execution: TerminalImageExecution,
    /// Whether the exact playback backend operation succeeded.
    pub backend_succeeded: bool,
}

struct PacketResolver {
    handle: TerminalImageHandle,
    bytes: Option<Vec<u8>>,
}

impl TerminalImageByteResolver for PacketResolver {
    fn resolve(
        &mut self,
        handle: &TerminalImageHandle,
        max_bytes: u64,
    ) -> Result<Vec<u8>, TerminalImageError> {
        if handle != &self.handle {
            return Err(TerminalImageError::ByteResolutionFailed);
        }
        let bytes = self
            .bytes
            .take()
            .ok_or(TerminalImageError::ByteResolutionFailed)?;
        let byte_count =
            u64::try_from(bytes.len()).map_err(|_| TerminalImageError::ImageBytesExceeded)?;
        if byte_count > max_bytes {
            return Err(TerminalImageError::ImageBytesExceeded);
        }
        Ok(bytes)
    }
}

/// Inputs to one transient frame-display shell call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoFrameExecutionInput {
    /// Safe playback identity.
    pub playback_id: TerminalVideoId,
    /// Exact selected playback mode.
    pub backend_mode: TerminalVideoBackendMode,
    /// Exact terminal-image capability row.
    pub protocol_capabilities: TerminalImageCapabilities,
    /// Terminal-image bounds.
    pub image_policy: TerminalImagePolicy,
    /// Current bounded lifecycle entry count.
    pub lifecycle_entries: u16,
    /// Host-supplied terminal geometry.
    pub geometry: TerminalImageGeometry,
    /// Transient frame packet.
    pub packet: TerminalVideoFramePacket,
}

/// Transmits one fetched static frame through `rat-image`, then performs the
/// exact injected playback operation. It opens no terminal and resolves no
/// path, URL, codec, process, network, credential, clock, or persistence state.
///
/// # Errors
///
/// Returns a typed terminal-image or backend error.
pub fn execute_terminal_video_frame<L, B, W>(
    input: TerminalVideoFrameExecutionInput,
    lifecycle: &mut L,
    backend: &mut B,
    writer: &mut W,
) -> Result<TerminalVideoFrameExecution, TerminalVideoError>
where
    L: TerminalImageLifecycleService,
    B: TerminalVideoBackendService,
    W: Write,
{
    let operation = match input.backend_mode {
        TerminalVideoBackendMode::KittyAnimation => {
            TerminalVideoBackendOperation::PresentAnimationFrame
        }
        TerminalVideoBackendMode::StaticReplacement => {
            TerminalVideoBackendOperation::ReplaceStaticFrame
        }
        _ => return Err(TerminalVideoError::InvalidTransition),
    };
    let frame_index = input.packet.metadata.frame_index;
    let handle = input.packet.metadata.image.handle.clone();
    let plan = plan_terminal_image(TerminalImagePlanInput {
        image: input.packet.metadata.image,
        capabilities: input.protocol_capabilities,
        geometry: input.geometry,
        policy: input.image_policy,
        intent: TerminalImageIntent::Render,
        lifecycle_entries: input.lifecycle_entries,
    })
    .map_err(|_| TerminalVideoError::TerminalImageFailed)?;
    let geometry = plan.geometry;
    let mut resolver = PacketResolver {
        handle,
        bytes: Some(input.packet.bytes),
    };
    let image_execution = execute_terminal_image_plan(&plan, &mut resolver, lifecycle, writer);
    if image_execution.final_status != TerminalImageOperationStatus::Succeeded {
        return Err(TerminalVideoError::TerminalImageFailed);
    }
    backend
        .execute_backend(&TerminalVideoBackendCall {
            operation,
            playback_id: input.playback_id,
            frame_index: Some(frame_index),
            geometry,
        })
        .map_err(|_| TerminalVideoError::BackendOperationFailed)?;
    Ok(TerminalVideoFrameExecution {
        frame_index,
        image_execution,
        backend_succeeded: true,
    })
}

/// Executes only metadata-level backend actions through an injected service.
/// Frame requests, waits, drops, and completion markers produce no host call.
///
/// # Errors
///
/// Returns [`TerminalVideoError::BackendOperationFailed`] on the first failed
/// injected backend operation.
pub fn execute_terminal_video_backend_actions<B: TerminalVideoBackendService>(
    playback_id: &TerminalVideoId,
    geometry: TerminalImageGeometry,
    actions: &[TerminalVideoAction],
    backend: &mut B,
) -> Result<u16, TerminalVideoError> {
    let mut completed = 0_u16;
    for action in actions {
        let (operation, frame_index) = match action {
            TerminalVideoAction::Backend(operation) => (*operation, None),
            TerminalVideoAction::DisplayFrame { frame, operation } => {
                (*operation, Some(frame.frame_index))
            }
            _ => continue,
        };
        backend
            .execute_backend(&TerminalVideoBackendCall {
                operation,
                playback_id: playback_id.clone(),
                frame_index,
                geometry,
            })
            .map_err(|_| TerminalVideoError::BackendOperationFailed)?;
        completed = completed
            .checked_add(1)
            .ok_or(TerminalVideoError::ActionBoundExceeded)?;
    }
    Ok(completed)
}

/// Deterministic metadata-only playback receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalVideoReceipt {
    /// Receipt schema.
    pub schema: &'static str,
    /// Selected terminal-image cohort.
    pub rats_revision: &'static str,
    /// Safe source class.
    pub source_class: &'static str,
    /// Exact backend mode.
    pub backend_mode: &'static str,
    /// Final status.
    pub status: &'static str,
    /// Source metadata identity.
    pub source_digest: String,
    /// Policy identity.
    pub policy_digest: String,
    /// Presented frame count.
    pub displayed_frames: u32,
    /// Dropped frame count.
    pub dropped_frames: u32,
    /// Remaining cleanup obligation count.
    pub cleanup_obligations: u16,
    /// Deterministic receipt identity.
    pub receipt_digest: String,
    /// Explicit non-claims.
    pub non_claims: [&'static str; RECEIPT_NON_CLAIM_COUNT],
}

/// Projects a deterministic metadata-only receipt.
///
/// # Errors
///
/// Returns [`TerminalVideoError::UnsafeIdentity`] for a malformed policy BLAKE3
/// identity.
pub fn project_terminal_video_receipt(
    descriptor: &TerminalVideoDescriptor,
    state: &TerminalVideoState,
    policy_digest: &str,
) -> Result<TerminalVideoReceipt, TerminalVideoError> {
    validate_blake3(policy_digest)?;
    let receipt_digest = terminal_video_receipt_identity(descriptor, state, policy_digest);
    Ok(TerminalVideoReceipt {
        schema: RECEIPT_SCHEMA,
        rats_revision: RATS_IMAGE_COHORT_REVISION,
        source_class: descriptor.source_class.label(),
        backend_mode: state.backend.label(),
        status: state.status.label(),
        source_digest: descriptor.source_digest.as_str().to_owned(),
        policy_digest: policy_digest.to_owned(),
        displayed_frames: state.displayed_frames,
        dropped_frames: state.dropped_frames,
        cleanup_obligations: state.cleanup_obligations,
        receipt_digest,
        non_claims: [
            "no decoder-safety or media-authenticity claim",
            "no exact-timing, smoothness, or frame-completeness claim",
            "no color-fidelity or terminal-correctness claim",
            "no audio or audio-video-synchronization claim",
            "no sandbox or process-isolation claim",
            "no production-readiness or release-eligibility claim",
        ],
    })
}

fn terminal_video_receipt_identity(
    descriptor: &TerminalVideoDescriptor,
    state: &TerminalVideoState,
    policy_digest: &str,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hash_field(&mut hasher, RECEIPT_DOMAIN);
    hash_field(&mut hasher, RATS_IMAGE_COHORT_REVISION.as_bytes());
    hash_field(&mut hasher, descriptor.playback_id.as_str().as_bytes());
    hash_field(&mut hasher, descriptor.source_class.label().as_bytes());
    hash_field(&mut hasher, descriptor.source_digest.as_str().as_bytes());
    hash_field(&mut hasher, policy_digest.as_bytes());
    hash_field(&mut hasher, state.backend.label().as_bytes());
    hash_field(&mut hasher, state.status.label().as_bytes());
    hash_field(&mut hasher, &state.displayed_frames.to_le_bytes());
    hash_field(&mut hasher, &state.dropped_frames.to_le_bytes());
    hash_field(&mut hasher, &state.cleanup_obligations.to_le_bytes());
    hasher.finalize().to_hex().to_string()
}

fn hash_field(hasher: &mut blake3::Hasher, field: &[u8]) {
    hasher.update(&u64::try_from(field.len()).unwrap_or(u64::MAX).to_le_bytes());
    hasher.update(field);
}

fn validate_blake3(value: &str) -> Result<(), TerminalVideoError> {
    if value.len() != BLAKE3_HEX_CHARS
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Err(TerminalVideoError::UnsafeIdentity)
    } else {
        Ok(())
    }
}

/// Rejects receipt candidates that contain media content, terminal payloads,
/// authority-bearing values, raw timing traces, or unsupported claims.
///
/// # Errors
///
/// Returns [`TerminalVideoError::EvidenceLeak`] for forbidden content.
pub fn validate_terminal_video_evidence(text: &str) -> Result<(), TerminalVideoError> {
    let normalized = text.to_ascii_lowercase();
    if FORBIDDEN_EVIDENCE_TOKENS
        .iter()
        .any(|token| normalized.contains(token))
    {
        Err(TerminalVideoError::EvidenceLeak)
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[path = "videotests.rs"]
mod tests;
