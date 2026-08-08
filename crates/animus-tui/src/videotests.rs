use super::*;

const ONE_PIXEL_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0,
    0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 8, 215, 99, 248, 207, 192, 240, 31, 0, 5,
    0, 1, 255, 137, 153, 61, 29, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];
const MAX_FRAME_BYTES: u64 = 8_388_608;
const MAX_BUFFERED_BYTES: u64 = 16_777_216;
const MAX_QUEUE_FRAMES: u16 = 2;
const MAX_FRAME_DIMENSION: u32 = 8_192;
const MAX_FRAME_PIXELS: u64 = 16_777_216;
const MAX_FRAMES: u32 = 120;
const MAX_FRAME_RATE_MILLIHERTZ: u32 = 60_000;
const MAX_DURATION_TICKS: u64 = 60_000_000;
const MAX_DROPPED_FRAMES: u32 = 4;
const MAX_ACTIONS: u16 = 8;
const MAX_LAG_TICKS: u64 = 20_000;
const MAX_FRAME_DEADLINE_TICKS: u64 = 500_000;
const MAX_CLEANUP_OBLIGATIONS: u16 = 3;
const FRAME_RATE_MILLIHERTZ: u32 = 30_000;
const FRAME_INTERVAL_TICKS: u64 = 33_333;
const TOTAL_FRAMES: u32 = 2;
const VIDEO_DURATION_TICKS: u64 = 66_666;
const POLICY_DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MAX_IMAGE_BYTES: u64 = 8_388_608;
const MAX_PREPARED_BYTES: u64 = 33_554_432;
const MAX_CAPABILITY_RESPONSE_BYTES: u16 = 256;
const MAX_SOURCE_PIXELS: u64 = 16_777_216;
const MAX_DECODED_BYTES: u64 = 134_217_728;
const MAX_TERMINAL_WIDTH: u16 = 16;
const MAX_TERMINAL_HEIGHT: u16 = 8;
const MAX_TERMINAL_CELLS: u64 = 128;
const MAX_CHUNKS: u64 = 8_192;
const MAX_IMAGE_OPERATIONS: u16 = 8;
const MAX_LIFECYCLE_ENTRIES: u16 = 64;
const MAX_ALT_TEXT_CHARS: u16 = 256;

fn policy() -> TerminalVideoPolicy {
    TerminalVideoPolicy {
        max_frame_bytes: MAX_FRAME_BYTES,
        max_buffered_bytes: MAX_BUFFERED_BYTES,
        max_queue_frames: MAX_QUEUE_FRAMES,
        max_frame_width: MAX_FRAME_DIMENSION,
        max_frame_height: MAX_FRAME_DIMENSION,
        max_frame_pixels: MAX_FRAME_PIXELS,
        max_frames: MAX_FRAMES,
        max_frame_rate_millihertz: MAX_FRAME_RATE_MILLIHERTZ,
        max_duration_ticks: MAX_DURATION_TICKS,
        max_dropped_frames: MAX_DROPPED_FRAMES,
        max_actions: MAX_ACTIONS,
        max_lag_ticks: MAX_LAG_TICKS,
        max_frame_deadline_ticks: MAX_FRAME_DEADLINE_TICKS,
        max_cleanup_obligations: MAX_CLEANUP_OBLIGATIONS,
        frame_drop: TerminalVideoPolicyDecision::Allow,
        static_replacement: TerminalVideoPolicyDecision::Allow,
        thumbnail_fallback: TerminalVideoPolicyDecision::Allow,
        text_fallback: TerminalVideoPolicyDecision::Allow,
    }
}

fn image_policy() -> TerminalImagePolicy {
    TerminalImagePolicy {
        max_image_bytes: MAX_IMAGE_BYTES,
        max_prepared_bytes: MAX_PREPARED_BYTES,
        max_capability_response_bytes: MAX_CAPABILITY_RESPONSE_BYTES,
        max_source_width: MAX_FRAME_DIMENSION,
        max_source_height: MAX_FRAME_DIMENSION,
        max_source_pixels: MAX_SOURCE_PIXELS,
        max_decoded_bytes: MAX_DECODED_BYTES,
        max_terminal_width: MAX_TERMINAL_WIDTH,
        max_terminal_height: MAX_TERMINAL_HEIGHT,
        max_terminal_cells: MAX_TERMINAL_CELLS,
        max_chunks: MAX_CHUNKS,
        max_operations: MAX_IMAGE_OPERATIONS,
        max_lifecycle_entries: MAX_LIFECYCLE_ENTRIES,
        max_alt_text_chars: MAX_ALT_TEXT_CHARS,
        allow_text_fallback: true,
    }
}

fn descriptor() -> TerminalVideoDescriptor {
    TerminalVideoDescriptor {
        playback_id: TerminalVideoId::try_new("playback_1").expect("playback id"),
        source_handle: TerminalImageHandle::try_new("source_1").expect("source handle"),
        source_class: TerminalVideoSourceClass::ApprovedDecodedFrames,
        source_digest: TerminalImageDigest::from_bytes(b"safe source metadata"),
        total_frames: TOTAL_FRAMES,
        frame_rate_millihertz: FRAME_RATE_MILLIHERTZ,
        duration_ticks: VIDEO_DURATION_TICKS,
        geometry: TerminalImageGeometry {
            column: 1,
            row: 2,
            width: 8,
            height: 4,
        },
    }
}

fn capability(enabled: bool) -> TerminalVideoCapabilityState {
    if enabled {
        TerminalVideoCapabilityState::Supported
    } else {
        TerminalVideoCapabilityState::Unsupported
    }
}

fn capabilities(mode: TerminalVideoBackendMode) -> TerminalVideoCapabilities {
    let animation = mode == TerminalVideoBackendMode::KittyAnimation;
    let replacement = mode == TerminalVideoBackendMode::StaticReplacement || animation;
    let thumbnail = mode == TerminalVideoBackendMode::ThumbnailFallback;
    let text = mode == TerminalVideoBackendMode::TextFallback;
    TerminalVideoCapabilities {
        protocol: if replacement {
            TerminalImageProtocol::Kitty
        } else {
            TerminalImageProtocol::Text
        },
        frame_transfer: capability(replacement),
        kitty_animation_control: capability(animation),
        kitty_animation_placement: capability(animation),
        kitty_animation_deletion: capability(animation),
        static_replacement: capability(replacement),
        thumbnail_fallback: capability(thumbnail),
        text_fallback: capability(text),
    }
}

fn frame(frame_index: u32) -> TerminalVideoFrameMetadata {
    let presentation_tick = u64::from(frame_index) * FRAME_INTERVAL_TICKS;
    TerminalVideoFrameMetadata {
        frame_index,
        presentation_tick,
        image: TerminalImageMetadata {
            handle: TerminalImageHandle::try_new(&format!("frame_{frame_index}"))
                .expect("frame handle"),
            content_digest: TerminalImageDigest::from_bytes(ONE_PIXEL_PNG),
            byte_count: u64::try_from(ONE_PIXEL_PNG.len()).expect("fixture size"),
            source_width: 1,
            source_height: 1,
            format: super::super::pixel::TerminalImageFormat::Png,
            alt_text: "video frame".to_owned(),
        },
    }
}

fn reduce(
    state: TerminalVideoState,
    command: TerminalVideoCommand,
    mode: TerminalVideoBackendMode,
) -> Result<TerminalVideoReduction, TerminalVideoError> {
    reduce_terminal_video(TerminalVideoReduceInput {
        descriptor: descriptor(),
        capabilities: capabilities(mode),
        policy: policy(),
        state,
        command,
    })
}

fn started(mode: TerminalVideoBackendMode) -> TerminalVideoReduction {
    reduce(
        TerminalVideoState::idle(descriptor().playback_id),
        TerminalVideoCommand::Start { tick: 0 },
        mode,
    )
    .expect("start")
}

#[test]
fn backend_selection_requires_exact_animation_row_and_preserves_fallback_order() {
    assert_eq!(
        select_terminal_video_backend(
            capabilities(TerminalVideoBackendMode::KittyAnimation),
            policy()
        ),
        TerminalVideoBackendMode::KittyAnimation
    );
    assert_eq!(
        select_terminal_video_backend(
            capabilities(TerminalVideoBackendMode::StaticReplacement),
            policy()
        ),
        TerminalVideoBackendMode::StaticReplacement
    );
    assert_eq!(
        select_terminal_video_backend(
            capabilities(TerminalVideoBackendMode::ThumbnailFallback),
            policy()
        ),
        TerminalVideoBackendMode::ThumbnailFallback
    );
    assert_eq!(
        select_terminal_video_backend(
            capabilities(TerminalVideoBackendMode::TextFallback),
            policy()
        ),
        TerminalVideoBackendMode::TextFallback
    );
    assert_eq!(
        select_terminal_video_backend(
            capabilities(TerminalVideoBackendMode::Unsupported),
            policy()
        ),
        TerminalVideoBackendMode::Unsupported
    );
}

#[test]
fn ordered_frames_advance_with_bounded_requests_and_static_replacement() {
    let start = started(TerminalVideoBackendMode::StaticReplacement);
    assert_eq!(start.state.status, TerminalVideoStatus::Running);
    assert!(matches!(
        start.actions.as_slice(),
        [TerminalVideoAction::RequestFrame { frame_index: 0, .. }]
    ));
    let first = reduce(
        start.state,
        TerminalVideoCommand::FrameReady {
            frame: frame(0),
            tick: 0,
            queued_frames: 0,
            queued_bytes: 0,
        },
        TerminalVideoBackendMode::StaticReplacement,
    )
    .expect("first frame");
    assert_eq!(first.state.displayed_frames, 1);
    assert_eq!(first.state.next_frame_index, 1);
    assert!(matches!(
        first.actions.as_slice(),
        [
            TerminalVideoAction::DisplayFrame {
                operation: TerminalVideoBackendOperation::ReplaceStaticFrame,
                ..
            },
            TerminalVideoAction::RequestFrame { frame_index: 1, .. }
        ]
    ));
    let second = reduce(
        first.state,
        TerminalVideoCommand::FrameReady {
            frame: frame(1),
            tick: FRAME_INTERVAL_TICKS,
            queued_frames: 0,
            queued_bytes: 0,
        },
        TerminalVideoBackendMode::StaticReplacement,
    )
    .expect("second frame");
    assert_eq!(second.state.displayed_frames, TOTAL_FRAMES);
    assert_eq!(second.actions.len(), 1);
}

#[test]
fn pause_resume_and_tick_are_explicit_and_monotonic() {
    let start = started(TerminalVideoBackendMode::KittyAnimation);
    let paused = reduce(
        start.state,
        TerminalVideoCommand::Pause { tick: 10 },
        TerminalVideoBackendMode::KittyAnimation,
    )
    .expect("pause");
    assert_eq!(paused.state.status, TerminalVideoStatus::Paused);
    assert_eq!(paused.actions.len(), 2);
    let resumed = reduce(
        paused.state,
        TerminalVideoCommand::Resume { tick: 20 },
        TerminalVideoBackendMode::KittyAnimation,
    )
    .expect("resume");
    assert_eq!(resumed.state.status, TerminalVideoStatus::Running);
    assert_eq!(resumed.actions.len(), 3);
    assert_eq!(
        reduce(
            resumed.state,
            TerminalVideoCommand::Tick { tick: 19 },
            TerminalVideoBackendMode::KittyAnimation,
        ),
        Err(TerminalVideoError::StaleTick)
    );
}

#[test]
fn duplicate_out_of_order_and_invalid_timing_fail_closed() {
    let start = started(TerminalVideoBackendMode::StaticReplacement);
    assert_eq!(
        reduce(
            start.state.clone(),
            TerminalVideoCommand::FrameReady {
                frame: frame(1),
                tick: FRAME_INTERVAL_TICKS,
                queued_frames: 0,
                queued_bytes: 0,
            },
            TerminalVideoBackendMode::StaticReplacement,
        ),
        Err(TerminalVideoError::FrameOrderMismatch)
    );
    let mut bad_timing = frame(0);
    bad_timing.presentation_tick = 1;
    assert_eq!(
        reduce(
            start.state,
            TerminalVideoCommand::FrameReady {
                frame: bad_timing,
                tick: 0,
                queued_frames: 0,
                queued_bytes: 0,
            },
            TerminalVideoBackendMode::StaticReplacement,
        ),
        Err(TerminalVideoError::FrameTimingMismatch)
    );
}

#[test]
fn queue_pressure_drops_before_retention_and_enforces_drop_bound() {
    let start = started(TerminalVideoBackendMode::StaticReplacement);
    let dropped = reduce(
        start.state,
        TerminalVideoCommand::FrameReady {
            frame: frame(0),
            tick: 0,
            queued_frames: MAX_QUEUE_FRAMES,
            queued_bytes: MAX_BUFFERED_BYTES,
        },
        TerminalVideoBackendMode::StaticReplacement,
    )
    .expect("bounded drop");
    assert_eq!(dropped.state.dropped_frames, 1);
    assert!(matches!(
        dropped.actions.first(),
        Some(TerminalVideoAction::DropFrame { frame_index: 0 })
    ));

    let mut strict = policy();
    strict.max_dropped_frames = 0;
    assert_eq!(
        reduce_terminal_video(TerminalVideoReduceInput {
            descriptor: descriptor(),
            capabilities: capabilities(TerminalVideoBackendMode::StaticReplacement),
            policy: strict,
            state: started(TerminalVideoBackendMode::StaticReplacement).state,
            command: TerminalVideoCommand::FrameReady {
                frame: frame(0),
                tick: 0,
                queued_frames: MAX_QUEUE_FRAMES,
                queued_bytes: 0,
            },
        }),
        Err(TerminalVideoError::DropBoundExceeded)
    );
}

#[test]
fn early_frame_waits_without_retaining_or_reordering() {
    let mut start = started(TerminalVideoBackendMode::StaticReplacement).state;
    start.next_frame_index = 1;
    let waiting = reduce(
        start,
        TerminalVideoCommand::FrameReady {
            frame: frame(1),
            tick: 0,
            queued_frames: 0,
            queued_bytes: 0,
        },
        TerminalVideoBackendMode::StaticReplacement,
    )
    .expect("wait");
    assert_eq!(waiting.state.next_frame_index, 1);
    assert_eq!(
        waiting.actions,
        [TerminalVideoAction::WaitUntil {
            tick: FRAME_INTERVAL_TICKS,
        }]
    );
}

#[test]
fn cancellation_creates_exact_cleanup_and_cleanup_failure_stays_visible() {
    let start = started(TerminalVideoBackendMode::KittyAnimation);
    let cancelling = reduce(
        start.state,
        TerminalVideoCommand::Cancel { tick: 5 },
        TerminalVideoBackendMode::KittyAnimation,
    )
    .expect("cancel");
    assert_eq!(cancelling.state.status, TerminalVideoStatus::CleanupPending);
    assert_eq!(cancelling.state.cleanup_obligations, 2);
    let one = reduce(
        cancelling.state,
        TerminalVideoCommand::CleanupCompleted { tick: 6 },
        TerminalVideoBackendMode::KittyAnimation,
    )
    .expect("first cleanup");
    assert_eq!(one.state.cleanup_obligations, 1);
    let failed = reduce(
        one.state,
        TerminalVideoCommand::CleanupFailed { tick: 7 },
        TerminalVideoBackendMode::KittyAnimation,
    )
    .expect("cleanup failure state");
    assert_eq!(failed.state.status, TerminalVideoStatus::CleanupIncomplete);
}

#[test]
fn terminal_loss_source_failure_layout_eviction_and_shutdown_stop_requests() {
    let terminal_events = [
        TerminalVideoCommand::SourceFailed { tick: 1 },
        TerminalVideoCommand::TerminalLost { tick: 1 },
        TerminalVideoCommand::LayoutEvicted { tick: 1 },
        TerminalVideoCommand::Shutdown { tick: 1 },
    ];
    for command in terminal_events {
        let result = reduce(
            started(TerminalVideoBackendMode::StaticReplacement).state,
            command,
            TerminalVideoBackendMode::StaticReplacement,
        )
        .expect("failure cleanup");
        assert_eq!(result.state.status, TerminalVideoStatus::Failed);
        assert_eq!(
            result.actions.first(),
            Some(&TerminalVideoAction::StopSource)
        );
        assert!(
            !result
                .actions
                .iter()
                .any(|action| matches!(action, TerminalVideoAction::RequestFrame { .. }))
        );
    }
}

#[test]
fn source_finish_requires_all_frames_and_completes_cleanup() {
    let mut state = started(TerminalVideoBackendMode::StaticReplacement).state;
    assert_eq!(
        reduce(
            state.clone(),
            TerminalVideoCommand::SourceFinished { tick: 1 },
            TerminalVideoBackendMode::StaticReplacement,
        ),
        Err(TerminalVideoError::FrameOrderMismatch)
    );
    state.next_frame_index = TOTAL_FRAMES;
    state.terminal_state_owned = true;
    let finishing = reduce(
        state,
        TerminalVideoCommand::SourceFinished {
            tick: VIDEO_DURATION_TICKS,
        },
        TerminalVideoBackendMode::StaticReplacement,
    )
    .expect("finish cleanup");
    assert_eq!(finishing.state.status, TerminalVideoStatus::CleanupPending);
    let complete = reduce(
        finishing.state,
        TerminalVideoCommand::CleanupCompleted {
            tick: VIDEO_DURATION_TICKS,
        },
        TerminalVideoBackendMode::StaticReplacement,
    )
    .expect("finish");
    assert_eq!(complete.state.status, TerminalVideoStatus::Finished);
}

struct FakeSource {
    packet: Option<TerminalVideoFramePacket>,
    error: Option<TerminalVideoError>,
}

impl TerminalVideoFrameSource for FakeSource {
    fn fetch_frame(
        &mut self,
        _request: &TerminalVideoFrameSourceRequest,
    ) -> Result<TerminalVideoFramePacket, TerminalVideoError> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }
        self.packet
            .take()
            .ok_or(TerminalVideoError::FrameSourceFailed)
    }
}

fn packet(frame_index: u32) -> TerminalVideoFramePacket {
    TerminalVideoFramePacket {
        metadata: frame(frame_index),
        bytes: ONE_PIXEL_PNG.to_vec(),
    }
}

#[test]
fn injected_frame_source_rechecks_bounds_identity_order_and_timeout() {
    let fetch = TerminalVideoFrameFetch {
        frame_index: 0,
        deadline_tick: 10,
    };
    let mut source = FakeSource {
        packet: Some(packet(0)),
        error: None,
    };
    let admitted = fetch_terminal_video_frame(&mut source, &descriptor(), fetch, policy())
        .expect("admitted packet");
    assert_eq!(admitted.metadata.frame_index, 0);

    let mut drifted = packet(0);
    drifted.bytes.push(0);
    let mut source = FakeSource {
        packet: Some(drifted),
        error: None,
    };
    assert_eq!(
        fetch_terminal_video_frame(&mut source, &descriptor(), fetch, policy()),
        Err(TerminalVideoError::FrameIdentityMismatch)
    );

    let mut source = FakeSource {
        packet: None,
        error: Some(TerminalVideoError::FrameSourceTimeout),
    };
    assert_eq!(
        fetch_terminal_video_frame(&mut source, &descriptor(), fetch, policy()),
        Err(TerminalVideoError::FrameSourceTimeout)
    );
}

#[derive(Default)]
struct FakeLifecycle {
    operations: Vec<super::super::pixel::TerminalImageOperation>,
    fail_cleanup: bool,
}

impl TerminalImageLifecycleService for FakeLifecycle {
    fn execute(
        &mut self,
        operation: super::super::pixel::TerminalImageOperation,
        _image_digest: &TerminalImageDigest,
        _geometry: TerminalImageGeometry,
    ) -> Result<(), TerminalImageError> {
        self.operations.push(operation);
        if self.fail_cleanup && operation == super::super::pixel::TerminalImageOperation::Cleanup {
            Err(TerminalImageError::LifecycleOperationFailed)
        } else {
            Ok(())
        }
    }
}

#[derive(Default)]
struct FakeBackend {
    operations: Vec<(TerminalVideoBackendOperation, Option<u32>)>,
    fail: bool,
}

impl TerminalVideoBackendService for FakeBackend {
    fn execute_backend(
        &mut self,
        call: &TerminalVideoBackendCall,
    ) -> Result<(), TerminalVideoError> {
        self.operations.push((call.operation, call.frame_index));
        if self.fail {
            Err(TerminalVideoError::BackendOperationFailed)
        } else {
            Ok(())
        }
    }
}

fn image_capabilities() -> TerminalImageCapabilities {
    TerminalImageCapabilities {
        protocol: TerminalImageProtocol::Kitty,
        query: super::super::pixel::TerminalImageCapabilityState::Supported,
        transmit: super::super::pixel::TerminalImageCapabilityState::Supported,
        place: super::super::pixel::TerminalImageCapabilityState::Supported,
        replace: super::super::pixel::TerminalImageCapabilityState::Supported,
        delete: super::super::pixel::TerminalImageCapabilityState::Supported,
        cleanup: super::super::pixel::TerminalImageCapabilityState::Supported,
    }
}

#[test]
fn injected_shell_transmits_static_frame_then_calls_exact_animation_operation() {
    let mut lifecycle = FakeLifecycle::default();
    let mut backend = FakeBackend::default();
    let mut writer = Vec::new();
    let execution = execute_terminal_video_frame(
        TerminalVideoFrameExecutionInput {
            playback_id: descriptor().playback_id,
            backend_mode: TerminalVideoBackendMode::KittyAnimation,
            protocol_capabilities: image_capabilities(),
            image_policy: image_policy(),
            lifecycle_entries: 0,
            geometry: descriptor().geometry,
            packet: packet(0),
        },
        &mut lifecycle,
        &mut backend,
        &mut writer,
    )
    .expect("frame execution");
    assert!(execution.image_execution.prepared_bytes_written > 0);
    assert_eq!(
        backend.operations,
        [(
            TerminalVideoBackendOperation::PresentAnimationFrame,
            Some(0)
        )]
    );
    assert!(writer.starts_with(b"\x1b_G"));
}

struct PartialWriter {
    remaining: usize,
}

impl Write for PartialWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.remaining == 0 {
            return Ok(0);
        }
        let count = bytes.len().min(self.remaining);
        self.remaining -= count;
        Ok(count)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn partial_terminal_write_fails_before_video_backend_and_attempts_image_cleanup() {
    let mut lifecycle = FakeLifecycle::default();
    let mut backend = FakeBackend::default();
    let mut writer = PartialWriter { remaining: 1 };
    assert_eq!(
        execute_terminal_video_frame(
            TerminalVideoFrameExecutionInput {
                playback_id: descriptor().playback_id,
                backend_mode: TerminalVideoBackendMode::StaticReplacement,
                protocol_capabilities: image_capabilities(),
                image_policy: image_policy(),
                lifecycle_entries: 0,
                geometry: descriptor().geometry,
                packet: packet(0),
            },
            &mut lifecycle,
            &mut backend,
            &mut writer,
        ),
        Err(TerminalVideoError::TerminalImageFailed)
    );
    assert!(backend.operations.is_empty());
    assert!(
        lifecycle
            .operations
            .contains(&super::super::pixel::TerminalImageOperation::Cleanup)
    );
}

#[test]
fn backend_action_shell_is_injected_bounded_and_fail_closed() {
    let start = started(TerminalVideoBackendMode::KittyAnimation);
    let mut backend = FakeBackend::default();
    assert_eq!(
        execute_terminal_video_backend_actions(
            &descriptor().playback_id,
            descriptor().geometry,
            &start.actions,
            &mut backend,
        ),
        Ok(1)
    );
    backend.fail = true;
    assert_eq!(
        execute_terminal_video_backend_actions(
            &descriptor().playback_id,
            descriptor().geometry,
            &[TerminalVideoAction::Backend(
                TerminalVideoBackendOperation::PauseAnimation,
            )],
            &mut backend,
        ),
        Err(TerminalVideoError::BackendOperationFailed)
    );
}

#[test]
fn fallback_and_cancellation_race_are_explicit() {
    let fallback_rows = [
        (
            TerminalVideoBackendMode::ThumbnailFallback,
            TerminalVideoBackendOperation::RenderThumbnail,
        ),
        (
            TerminalVideoBackendMode::TextFallback,
            TerminalVideoBackendOperation::RenderTextFallback,
        ),
    ];
    for (mode, operation) in fallback_rows {
        let fallback = started(mode);
        assert_eq!(fallback.state.status, TerminalVideoStatus::Finished);
        assert_eq!(
            fallback.actions,
            [
                TerminalVideoAction::Backend(operation),
                TerminalVideoAction::Complete(TerminalVideoStatus::Finished),
            ]
        );
    }
    let unsupported = started(TerminalVideoBackendMode::Unsupported);
    assert_eq!(unsupported.state.status, TerminalVideoStatus::Unsupported);
    assert_eq!(unsupported.actions, [TerminalVideoAction::Unsupported]);

    let cancelling = reduce(
        started(TerminalVideoBackendMode::KittyAnimation).state,
        TerminalVideoCommand::Cancel { tick: 1 },
        TerminalVideoBackendMode::KittyAnimation,
    )
    .expect("first cancellation");
    assert_eq!(
        reduce(
            cancelling.state,
            TerminalVideoCommand::Cancel { tick: 2 },
            TerminalVideoBackendMode::KittyAnimation,
        ),
        Err(TerminalVideoError::InvalidTransition)
    );
}

#[test]
fn malformed_descriptor_policy_frame_and_identity_fail_closed() {
    assert_eq!(
        TerminalVideoId::try_new("https://unsafe"),
        Err(TerminalVideoError::UnsafeIdentity)
    );
    let mut bad_descriptor = descriptor();
    bad_descriptor.frame_rate_millihertz = 0;
    assert_eq!(
        reduce_terminal_video(TerminalVideoReduceInput {
            descriptor: bad_descriptor,
            capabilities: capabilities(TerminalVideoBackendMode::StaticReplacement),
            policy: policy(),
            state: TerminalVideoState::idle(descriptor().playback_id),
            command: TerminalVideoCommand::Start { tick: 0 },
        }),
        Err(TerminalVideoError::InvalidDescriptor)
    );
    let mut bad_policy = policy();
    bad_policy.max_queue_frames = 0;
    assert_eq!(
        reduce_terminal_video(TerminalVideoReduceInput {
            descriptor: descriptor(),
            capabilities: capabilities(TerminalVideoBackendMode::StaticReplacement),
            policy: bad_policy,
            state: TerminalVideoState::idle(descriptor().playback_id),
            command: TerminalVideoCommand::Start { tick: 0 },
        }),
        Err(TerminalVideoError::InvalidPolicy)
    );
    let start = started(TerminalVideoBackendMode::StaticReplacement);
    let mut oversized = frame(0);
    oversized.image.byte_count = MAX_FRAME_BYTES + 1;
    assert_eq!(
        reduce(
            start.state,
            TerminalVideoCommand::FrameReady {
                frame: oversized,
                tick: 0,
                queued_frames: 0,
                queued_bytes: 0,
            },
            TerminalVideoBackendMode::StaticReplacement,
        ),
        Err(TerminalVideoError::FrameBoundExceeded)
    );
}

#[test]
fn receipt_is_deterministic_metadata_only_and_leaks_fail_closed() {
    let state = started(TerminalVideoBackendMode::KittyAnimation).state;
    let first = project_terminal_video_receipt(&descriptor(), &state, POLICY_DIGEST)
        .expect("first receipt");
    let second = project_terminal_video_receipt(&descriptor(), &state, POLICY_DIGEST)
        .expect("second receipt");
    assert_eq!(first, second);
    assert_eq!(first.rats_revision, RATS_IMAGE_COHORT_REVISION);
    assert!(!format!("{first:?}").contains(descriptor().source_handle.as_str()));
    for unsafe_text in [
        "frame_bytes=secret",
        "base64 payload",
        "https://media.invalid/video",
        "/home/private/movie",
        "decoder_output=raw",
        "timing_trace=all_ticks",
        "smooth_playback=true",
    ] {
        assert_eq!(
            validate_terminal_video_evidence(unsafe_text),
            Err(TerminalVideoError::EvidenceLeak)
        );
    }
    assert_eq!(
        validate_terminal_video_evidence("status=failed frames=2 dropped=1"),
        Ok(())
    );
}
