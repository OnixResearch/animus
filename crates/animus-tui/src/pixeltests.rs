use super::*;

const ONE_PIXEL_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0,
    0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 8, 215, 99, 248, 207, 192, 240, 31, 0, 5,
    0, 1, 255, 137, 153, 61, 29, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];
const MAX_IMAGE_BYTES: u64 = 8_388_608;
const MAX_PREPARED_BYTES: u64 = 33_554_432;
const MAX_CAPABILITY_RESPONSE_BYTES: u16 = 256;
const MAX_SOURCE_DIMENSION: u32 = 8_192;
const MAX_SOURCE_PIXELS: u64 = 16_777_216;
const MAX_DECODED_BYTES: u64 = 134_217_728;
const MAX_TERMINAL_WIDTH: u16 = 256;
const MAX_TERMINAL_HEIGHT: u16 = 128;
const MAX_TERMINAL_CELLS: u64 = 32_768;
const MAX_CHUNKS: u64 = 8_192;
const MAX_OPERATIONS: u16 = 8;
const MAX_LIFECYCLE_ENTRIES: u16 = 64;
const MAX_ALT_TEXT_CHARS: u16 = 256;
const KITTY_MULTICHUNK_EXTRA_BYTES: usize = 5_000;
const KITTY_FRAME_PREFIX: &[u8] = b"\x1b_G";

fn policy() -> TerminalImagePolicy {
    TerminalImagePolicy {
        max_image_bytes: MAX_IMAGE_BYTES,
        max_prepared_bytes: MAX_PREPARED_BYTES,
        max_capability_response_bytes: MAX_CAPABILITY_RESPONSE_BYTES,
        max_source_width: MAX_SOURCE_DIMENSION,
        max_source_height: MAX_SOURCE_DIMENSION,
        max_source_pixels: MAX_SOURCE_PIXELS,
        max_decoded_bytes: MAX_DECODED_BYTES,
        max_terminal_width: MAX_TERMINAL_WIDTH,
        max_terminal_height: MAX_TERMINAL_HEIGHT,
        max_terminal_cells: MAX_TERMINAL_CELLS,
        max_chunks: MAX_CHUNKS,
        max_operations: MAX_OPERATIONS,
        max_lifecycle_entries: MAX_LIFECYCLE_ENTRIES,
        max_alt_text_chars: MAX_ALT_TEXT_CHARS,
        allow_text_fallback: true,
    }
}

fn image() -> TerminalImageMetadata {
    TerminalImageMetadata {
        handle: TerminalImageHandle::try_new("image_1").expect("safe handle"),
        content_digest: TerminalImageDigest::from_bytes(ONE_PIXEL_PNG),
        byte_count: u64::try_from(ONE_PIXEL_PNG.len()).expect("fixture length"),
        source_width: 1,
        source_height: 1,
        format: TerminalImageFormat::Png,
        alt_text: "one pixel image".to_owned(),
    }
}

const fn geometry() -> TerminalImageGeometry {
    TerminalImageGeometry {
        column: 2,
        row: 3,
        width: 8,
        height: 4,
    }
}

fn support(enabled: bool) -> TerminalImageCapabilityState {
    if enabled {
        TerminalImageCapabilityState::Supported
    } else {
        TerminalImageCapabilityState::Unsupported
    }
}

fn capabilities(protocol: TerminalImageProtocol) -> TerminalImageCapabilities {
    TerminalImageCapabilities {
        protocol,
        query: support(protocol == TerminalImageProtocol::Kitty),
        transmit: support(protocol != TerminalImageProtocol::Text),
        place: support(protocol == TerminalImageProtocol::Kitty),
        replace: support(protocol == TerminalImageProtocol::Kitty),
        delete: support(protocol == TerminalImageProtocol::Kitty),
        cleanup: support(protocol == TerminalImageProtocol::Kitty),
    }
}

fn input(protocol: TerminalImageProtocol, intent: TerminalImageIntent) -> TerminalImagePlanInput {
    TerminalImagePlanInput {
        image: image(),
        capabilities: capabilities(protocol),
        geometry: geometry(),
        policy: policy(),
        intent,
        lifecycle_entries: 0,
    }
}

#[test]
fn kitty_capability_response_parsing_is_bounded_and_content_free() {
    let response = b"\x1b_Gi=31;OK\x1b\\";
    assert_eq!(
        parse_kitty_capability_response(response, MAX_CAPABILITY_RESPONSE_BYTES),
        Ok(TerminalImageCapabilityState::Supported)
    );
    assert_eq!(
        parse_kitty_capability_response(b"\x1b_Gi=31;EINVAL\x1b\\", MAX_CAPABILITY_RESPONSE_BYTES),
        Err(TerminalImageError::MalformedCapabilityResponse)
    );
    assert_eq!(
        parse_kitty_capability_response(response, 1),
        Err(TerminalImageError::CapabilityResponseExceeded)
    );
}

#[test]
fn kitty_plan_preserves_the_static_lifecycle_order() {
    let plan = plan_terminal_image(input(
        TerminalImageProtocol::Kitty,
        TerminalImageIntent::Render,
    ))
    .expect("kitty plan");
    assert_eq!(
        plan.operations,
        [
            TerminalImageOperation::Query,
            TerminalImageOperation::Transmit,
            TerminalImageOperation::Place,
        ]
    );
    assert!(plan.cleanup_required);
}

#[test]
fn alternate_protocols_admit_only_their_exact_transmit_row() {
    for protocol in [TerminalImageProtocol::Iterm2, TerminalImageProtocol::Sixel] {
        let plan = plan_terminal_image(input(protocol, TerminalImageIntent::Render))
            .expect("alternate protocol");
        assert_eq!(plan.operations, [TerminalImageOperation::Transmit]);
        assert!(plan.cleanup_required);
    }
}

#[test]
fn replace_delete_and_cleanup_remain_distinct_operations() {
    let replacements = [
        (
            TerminalImageIntent::Replace {
                prior_image_id: "old_image".to_owned(),
            },
            TerminalImageOperation::Replace,
        ),
        (TerminalImageIntent::Delete, TerminalImageOperation::Delete),
        (
            TerminalImageIntent::Cleanup,
            TerminalImageOperation::Cleanup,
        ),
    ];
    for (intent, expected) in replacements {
        let plan = plan_terminal_image(input(TerminalImageProtocol::Kitty, intent))
            .expect("lifecycle plan");
        assert_eq!(plan.operations, [expected]);
    }
}

#[test]
fn missing_render_capability_produces_visible_bounded_text_fallback() {
    let mut request = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    request.capabilities.place = TerminalImageCapabilityState::Unsupported;
    let plan = plan_terminal_image(request).expect("fallback");
    assert_eq!(plan.protocol, TerminalImageProtocol::Text);
    assert_eq!(plan.operations, [TerminalImageOperation::TextFallback]);
    assert!(!plan.cleanup_required);
}

#[derive(Default)]
struct Resolver {
    bytes: Vec<u8>,
    fail: bool,
}

impl TerminalImageByteResolver for Resolver {
    fn resolve(
        &mut self,
        _handle: &TerminalImageHandle,
        max_bytes: u64,
    ) -> Result<Vec<u8>, TerminalImageError> {
        if self.fail || u64::try_from(self.bytes.len()).unwrap_or(u64::MAX) > max_bytes {
            Err(TerminalImageError::ByteResolutionFailed)
        } else {
            Ok(self.bytes.clone())
        }
    }
}

#[derive(Default)]
struct Lifecycle {
    operations: Vec<TerminalImageOperation>,
    fail_on: Option<TerminalImageOperation>,
}

impl TerminalImageLifecycleService for Lifecycle {
    fn execute(
        &mut self,
        operation: TerminalImageOperation,
        _image_digest: &TerminalImageDigest,
        _geometry: TerminalImageGeometry,
    ) -> Result<(), TerminalImageError> {
        self.operations.push(operation);
        if self.fail_on == Some(operation) {
            Err(TerminalImageError::LifecycleOperationFailed)
        } else {
            Ok(())
        }
    }
}

#[test]
fn explicit_resolver_and_writer_execute_rats_transmit_without_ambient_terminal_access() {
    let plan = plan_terminal_image(input(
        TerminalImageProtocol::Iterm2,
        TerminalImageIntent::Render,
    ))
    .expect("plan");
    let mut resolver = Resolver {
        bytes: ONE_PIXEL_PNG.to_vec(),
        fail: false,
    };
    let mut lifecycle = Lifecycle::default();
    let mut writer = Vec::new();
    let execution = execute_terminal_image_plan(&plan, &mut resolver, &mut lifecycle, &mut writer);
    assert_eq!(
        execution.final_status,
        TerminalImageOperationStatus::Succeeded
    );
    assert_eq!(execution.outcomes.len(), 1);
    assert!(execution.prepared_bytes_written > 0);
    assert!(writer.starts_with(b"\x1b]1337;"));
    assert!(lifecycle.operations.is_empty());
}

#[test]
fn kitty_transmission_uses_bounded_multiple_upstream_chunks() {
    let mut bytes = ONE_PIXEL_PNG.to_vec();
    bytes.extend(std::iter::repeat_n(0_u8, KITTY_MULTICHUNK_EXTRA_BYTES));
    let mut request = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    request.image.byte_count = u64::try_from(bytes.len()).expect("fixture byte count");
    request.image.content_digest = TerminalImageDigest::from_bytes(&bytes);
    let plan = plan_terminal_image(request).expect("kitty plan");
    let mut resolver = Resolver { bytes, fail: false };
    let mut lifecycle = Lifecycle::default();
    let mut writer = Vec::new();
    let execution = execute_terminal_image_plan(&plan, &mut resolver, &mut lifecycle, &mut writer);
    let frame_count = writer
        .windows(KITTY_FRAME_PREFIX.len())
        .filter(|window| *window == KITTY_FRAME_PREFIX)
        .count();
    assert!(frame_count > 1);
    assert_eq!(
        execution.final_status,
        TerminalImageOperationStatus::Succeeded
    );
}

#[test]
fn receipt_is_deterministic_and_omits_handle_alt_text_and_protocol_payload() {
    let plan = plan_terminal_image(input(
        TerminalImageProtocol::Iterm2,
        TerminalImageIntent::Render,
    ))
    .expect("plan");
    let execution = TerminalImageExecution {
        outcomes: vec![(
            TerminalImageOperation::Transmit,
            TerminalImageOperationStatus::Succeeded,
        )],
        final_status: TerminalImageOperationStatus::Succeeded,
        prepared_bytes_written: 120,
    };
    let first = project_terminal_image_receipt(&plan, &execution);
    let second = project_terminal_image_receipt(&plan, &execution);
    assert_eq!(first, second);
    let rendered = format!("{first:?}");
    assert!(!rendered.contains(plan.image.handle.as_str()));
    assert!(!rendered.contains(plan.image.alt_text.as_str()));
    assert!(!rendered.contains("1337;"));
    assert_eq!(first.rats_revision, RATS_IMAGE_COHORT_REVISION);
}

#[test]
fn malformed_safe_identities_and_metadata_fail_closed() {
    assert_eq!(
        TerminalImageHandle::try_new("/private/image.png"),
        Err(TerminalImageError::UnsafeHandle)
    );
    assert_eq!(
        TerminalImageDigest::try_new("abcd"),
        Err(TerminalImageError::MalformedDigest)
    );
    let mut request = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    request.image.byte_count = 0;
    assert_eq!(
        plan_terminal_image(request),
        Err(TerminalImageError::InvalidMetadata)
    );
}

#[test]
fn image_source_terminal_alt_text_and_lifecycle_bounds_fail_closed() {
    let mut bytes = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    bytes.image.byte_count = bytes.policy.max_image_bytes + 1;
    assert_eq!(
        plan_terminal_image(bytes),
        Err(TerminalImageError::ImageBytesExceeded)
    );

    let mut source = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    source.image.source_width = source.policy.max_source_width + 1;
    assert_eq!(
        plan_terminal_image(source),
        Err(TerminalImageError::SourceGeometryExceeded)
    );

    let mut cells = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    cells.geometry.width = cells.policy.max_terminal_width + 1;
    assert_eq!(
        plan_terminal_image(cells),
        Err(TerminalImageError::TerminalGeometryExceeded)
    );

    let mut alt = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    alt.image.alt_text = "x".repeat(usize::from(alt.policy.max_alt_text_chars) + 1);
    assert_eq!(
        plan_terminal_image(alt),
        Err(TerminalImageError::AltTextExceeded)
    );

    let mut lifecycle = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    lifecycle.lifecycle_entries = lifecycle.policy.max_lifecycle_entries;
    assert_eq!(
        plan_terminal_image(lifecycle),
        Err(TerminalImageError::LifecycleExceeded)
    );
}

#[test]
fn unsupported_destructive_operations_and_denied_fallback_fail_closed() {
    let request = input(TerminalImageProtocol::Iterm2, TerminalImageIntent::Delete);
    assert_eq!(
        plan_terminal_image(request),
        Err(TerminalImageError::UnsupportedOperation)
    );

    let mut request = input(TerminalImageProtocol::Kitty, TerminalImageIntent::Render);
    request.capabilities.place = TerminalImageCapabilityState::Unsupported;
    request.policy.allow_text_fallback = false;
    assert_eq!(
        plan_terminal_image(request),
        Err(TerminalImageError::UnsupportedOperation)
    );
}

#[test]
fn resolver_failure_and_identity_drift_do_not_write() {
    let plan = plan_terminal_image(input(
        TerminalImageProtocol::Iterm2,
        TerminalImageIntent::Render,
    ))
    .expect("plan");
    let mut lifecycle = Lifecycle::default();

    let mut missing = Resolver {
        bytes: Vec::new(),
        fail: true,
    };
    let mut missing_writer = Vec::new();
    let missing_result =
        execute_terminal_image_plan(&plan, &mut missing, &mut lifecycle, &mut missing_writer);
    assert_eq!(
        missing_result.final_status,
        TerminalImageOperationStatus::Failed
    );
    assert!(missing_writer.is_empty());

    let mut drift = Resolver {
        bytes: vec![1, 2, 3],
        fail: false,
    };
    let mut drift_writer = Vec::new();
    let drift_result =
        execute_terminal_image_plan(&plan, &mut drift, &mut lifecycle, &mut drift_writer);
    assert_eq!(
        drift_result.final_status,
        TerminalImageOperationStatus::Failed
    );
    assert!(drift_writer.is_empty());
}

struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("fixture failure"))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Err(std::io::Error::other("fixture failure"))
    }
}

#[test]
fn failed_write_keeps_cleanup_failure_visible() {
    let plan = plan_terminal_image(input(
        TerminalImageProtocol::Iterm2,
        TerminalImageIntent::Render,
    ))
    .expect("plan");
    let mut resolver = Resolver {
        bytes: ONE_PIXEL_PNG.to_vec(),
        fail: false,
    };
    let mut lifecycle = Lifecycle {
        operations: Vec::new(),
        fail_on: Some(TerminalImageOperation::Cleanup),
    };
    let execution =
        execute_terminal_image_plan(&plan, &mut resolver, &mut lifecycle, &mut FailingWriter);
    assert_eq!(
        execution.final_status,
        TerminalImageOperationStatus::CleanupIncomplete
    );
    assert_eq!(
        execution.outcomes.last(),
        Some(&(
            TerminalImageOperation::Cleanup,
            TerminalImageOperationStatus::CleanupIncomplete,
        ))
    );
}

#[test]
fn evidence_validator_rejects_terminal_and_host_content() {
    for candidate in [
        "payload=base64",
        "data:image/png",
        "bytes=\\x1b_G",
        "/home/user/private.png",
        "TERM_PROGRAM=kitty",
        "credential=secret",
        "provider_payload={}",
    ] {
        assert_eq!(
            validate_terminal_image_evidence(candidate),
            Err(TerminalImageError::EvidenceLeak)
        );
    }
    assert_eq!(
        validate_terminal_image_evidence(
            "schema=animus.terminal-image-receipt.v1 status=succeeded image_bytes=67"
        ),
        Ok(())
    );
}
