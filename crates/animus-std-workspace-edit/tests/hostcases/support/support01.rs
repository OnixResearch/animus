const ROOT_ID: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const REVISION_ID: &str = "3333333333333333333333333333333333333333333333333333333333333333";
const POLICY_ID: &str = "4444444444444444444444444444444444444444444444444444444444444444";
const OTHER_ID: &str = "6666666666666666666666666666666666666666666666666666666666666666";
const WRITER_GENERATION: u64 = 7;
const NEXT_WRITER_GENERATION: u64 = 8;
const TAG_FIRST: &str = "tag-first";
const TAG_SECOND: &str = "tag-second";
const TAG_THIRD: &str = "tag-third";
const RESOURCE: &str = "nested/file.txt";
const ROOT_RESOURCE: &str = "root.txt";
const SOURCE: &[u8] = b"alpha\nbeta\ngamma\n";
const FINAL_MODE: u32 = 0o600;
const COLLISION_LIMIT: usize = 3;
const DRIFTED_WRITER_GENERATION: u64 = NEXT_WRITER_GENERATION.saturating_add(1);
const NONEXISTENT_OWNER: u32 = u32::MAX;
const MAX_WRITER_GENERATION: u64 = u64::MAX;
const FIRST_LINE: u32 = 1;
const SECOND_LINE: u32 = 2;
const THIRD_LINE: u32 = 3;
const LINE_COUNT: u32 = THIRD_LINE;

fn hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn base_snapshot(
    resource_identity: &str,
    source: &[u8],
) -> Result<animus_adapters::tool::hashed::Snapshot, String> {
    assert!(!resource_identity.is_empty());
    assert!(!source.is_empty());
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
    let snapshot_hash = hash(source);
    let snapshot = animus_adapters::tool::hashed::Snapshot::new(&animus_adapters::tool::hashed::SnapshotDraft {
        resource_class: "file",
        resource_identity,
        snapshot_hash: &snapshot_hash,
        normalization: "lf",
        line_count: LINE_COUNT,
        tag_generation: "fixture-tags-v1",
        tagged_lines: &lines,
    })
    .map_err(|error| format!("base snapshot: {error:?}"))?;
    Ok(snapshot)
}

fn gates() -> animus_std_workspace_edit::GateObservations {
    animus_std_workspace_edit::GateObservations {
        executor: animus_std_workspace_edit::GateDecision::Allowed,
        capability: animus_std_workspace_edit::GateDecision::Allowed,
        authority: animus_std_workspace_edit::GateDecision::Allowed,
        approval: animus_std_workspace_edit::GateDecision::Allowed,
        mutation: animus_std_workspace_edit::GateDecision::Allowed,
        lifecycle: animus_std_workspace_edit::LifecycleGate::Current,
        output: animus_std_workspace_edit::GateDecision::Allowed,
    }
}

#[derive(Clone, Copy)]
struct RequestInput<'a> {
    base: &'a animus_adapters::tool::hashed::Snapshot,
    resource_identity: &'a str,
    logical_resource: &'a std::path::Path,
    visible_spans: &'a [animus_std_workspace_edit::VisibleSpanDraft<'a>],
    visible_boundaries: &'a [animus_std_workspace_edit::VisibleBoundaryDraft<'a>],
    operations: &'a [animus_std_workspace_edit::EditOperationDraft<'a>],
}

fn build_request(input: RequestInput<'_>) -> animus_std_workspace_edit::HostRequest<'_> {
    animus_std_workspace_edit::HostRequest {
        base_snapshot: input.base,
        logical_resource: input.logical_resource,
        root_identity: ROOT_ID,
        resource_identity: input.resource_identity,
        revision_identity: REVISION_ID,
        metadata_policy_identity: POLICY_ID,
        writer_generation: WRITER_GENERATION,
        gates: gates(),
        line_ending_mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        replacement_ending: animus_std_workspace_edit::ReplacementEnding::Lf,
        mixed_endings_admitted: false,
        visible_spans: input.visible_spans,
        visible_boundaries: input.visible_boundaries,
        end_boundary_visible: true,
        operations: input.operations,
        expected_owner: None,
        expected_mode: None,
        final_mode: FINAL_MODE,
        collision_attempt_limit: COLLISION_LIMIT,
        require_durability: true,
    }
}
