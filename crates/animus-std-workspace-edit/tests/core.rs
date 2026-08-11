//! Pure workspace-edit admission, apply, freshness, publication, and receipt fixtures.

const ROOT_ID: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const RESOURCE_ID: &str = "2222222222222222222222222222222222222222222222222222222222222222";
const REVISION_ID: &str = "3333333333333333333333333333333333333333333333333333333333333333";
const POLICY_ID: &str = "4444444444444444444444444444444444444444444444444444444444444444";
const METADATA_ID: &str = "5555555555555555555555555555555555555555555555555555555555555555";
const OTHER_ID: &str = "6666666666666666666666666666666666666666666666666666666666666666";
const WRITER_GENERATION: u64 = 7;
const RELEASED_GENERATION: u64 = 8;
const FUTURE_RELEASED_GENERATION: u64 = RELEASED_GENERATION.saturating_add(1);
const TAG_FIRST: &str = "tag-first";
const TAG_SECOND: &str = "tag-second";
const TAG_THIRD: &str = "tag-third";
const FINAL_MODE: u32 = 0o600;
const COLLISION_LIMIT: usize = 3;
const FIRST_LINE: u32 = 1;
const SECOND_LINE: u32 = 2;
const THIRD_LINE: u32 = 3;
const LINE_COUNT: u32 = THIRD_LINE;
const PAYLOAD_BYTES: usize = 4;
const MAXIMUM_PAYLOAD_BYTES: usize = 16;

fn hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

#[derive(Clone, Copy)]
struct BaseCase<'a> {
    source: &'a [u8],
    normalization: &'a str,
    resource: &'a str,
}

fn base_snapshot(input: BaseCase<'_>) -> Result<animus_adapters::tool::hashed::Snapshot, String> {
    assert!(!input.source.is_empty());
    assert!(!input.normalization.is_empty());
    let first_hash = hash(b"alpha");
    let second_hash = hash(b"beta");
    let third_hash = hash(b"gamma");
    let lines = [
        animus_adapters::tool::hashed::TaggedLineDraft {
            line_number: FIRST_LINE,
            tag: TAG_FIRST,
            line_hash: &first_hash,
        },
        animus_adapters::tool::hashed::TaggedLineDraft {
            line_number: SECOND_LINE,
            tag: TAG_SECOND,
            line_hash: &second_hash,
        },
        animus_adapters::tool::hashed::TaggedLineDraft {
            line_number: THIRD_LINE,
            tag: TAG_THIRD,
            line_hash: &third_hash,
        },
    ];
    let snapshot_hash = hash(input.source);
    let snapshot = animus_adapters::tool::hashed::Snapshot::new(
        &animus_adapters::tool::hashed::SnapshotDraft {
            resource_class: "file",
            resource_identity: input.resource,
            snapshot_hash: &snapshot_hash,
            normalization: input.normalization,
            line_count: LINE_COUNT,
            tag_generation: "fixture-tags-v1",
            tagged_lines: &lines,
        },
    )
    .map_err(|error| format!("base snapshot: {error:?}"))?;
    Ok(snapshot)
}

#[derive(Clone, Copy)]
struct SourceCase<'a> {
    source: &'a [u8],
    normalization: &'a str,
    mode: animus_std_workspace_edit::LineEndingMode,
    final_newline: animus_std_workspace_edit::FinalNewlineState,
    mixed: bool,
}

fn admitted_for(
    case: SourceCase<'_>,
) -> Result<animus_std_workspace_edit::AdmittedSnapshot, String> {
    assert!(!case.source.is_empty());
    assert!(!case.normalization.is_empty());
    let base = base_snapshot(BaseCase {
        source: case.source,
        normalization: case.normalization,
        resource: RESOURCE_ID,
    })?;
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let boundaries = [
        animus_std_workspace_edit::VisibleBoundaryDraft {
            line_tag: TAG_SECOND,
            side: animus_std_workspace_edit::BoundarySide::Before,
        },
        animus_std_workspace_edit::VisibleBoundaryDraft {
            line_tag: TAG_SECOND,
            side: animus_std_workspace_edit::BoundarySide::After,
        },
    ];
    let snapshot =
        animus_std_workspace_edit::admit_snapshot(&animus_std_workspace_edit::SnapshotDraft {
            base: &base,
            root_identity: ROOT_ID,
            resource_identity: RESOURCE_ID,
            revision_identity: REVISION_ID,
            metadata_policy_identity: POLICY_ID,
            file_metadata_identity: METADATA_ID,
            writer_generation: WRITER_GENERATION,
            source_bytes: case.source,
            line_ending_mode: case.mode,
            final_newline: case.final_newline,
            replacement_ending: animus_std_workspace_edit::ReplacementEnding::Lf,
            mixed_endings_admitted: case.mixed,
            visible_spans: &spans,
            visible_boundaries: &boundaries,
            end_boundary_visible: true,
        })
        .map_err(|error| format!("admitted snapshot: {error:?}"))?;
    Ok(snapshot)
}

fn complete_classification() -> Result<durable_file_publication::core::Classification, String> {
    let request = durable_file_publication::core::PublicationRequest {
        destination_leaf: String::from("file.txt"),
        payload_bytes: PAYLOAD_BYTES,
        maximum_payload_bytes: MAXIMUM_PAYLOAD_BYTES,
        collision_attempt_limit: COLLISION_LIMIT,
        final_mode: FINAL_MODE,
        replacement: durable_file_publication::core::ReplacementMode::Replace,
        durability: durable_file_publication::core::DurabilityMode::DurabilityRequired,
    };
    assert!(request.payload_bytes > 0);
    assert!(request.maximum_payload_bytes >= request.payload_bytes);
    let classification = durable_file_publication::core::classify_publication(
        &request,
        &[
            durable_file_publication::core::PhaseObservation::StageCreated {
                leaf: String::from(".stage"),
            },
            durable_file_publication::core::PhaseObservation::PayloadWritten {
                bytes: PAYLOAD_BYTES,
            },
            durable_file_publication::core::PhaseObservation::PermissionsApplied,
            durable_file_publication::core::PhaseObservation::PayloadSynchronized,
            durable_file_publication::core::PhaseObservation::Renamed,
            durable_file_publication::core::PhaseObservation::ParentSynchronized,
        ],
    )
    .map_err(|error| format!("complete publication: {error:?}"))?;
    Ok(classification)
}

include!("corecases/semantics/case01.rs");
include!("corecases/semantics/case02.rs");
include!("corecases/semantics/case03.rs");
include!("corecases/semantics/case04.rs");
include!("corecases/semantics/case05.rs");
include!("corecases/evidence/case06.rs");
include!("corecases/evidence/case07.rs");
include!("corecases/evidence/case08.rs");
include!("corecases/evidence/case09.rs");
include!("corecases/evidence/case10.rs");
