struct IdentitySet {
    root: String,
    resource: String,
    revision: String,
    metadata_policy: String,
    file_metadata: String,
}

fn collect_tags(draft: &crate::model::SnapshotDraft<'_>) -> Vec<(String, u32)> {
    draft
        .base
        .tagged_lines()
        .iter()
        .map(|line| (String::from(line.tag()), line.line_number()))
        .collect()
}

fn line_for_tag(tags: &[(String, u32)], tag: &str) -> Result<u32, crate::model::HostIssue> {
    tags.iter()
        .find_map(|(candidate, line)| (candidate == tag).then_some(*line))
        .ok_or(crate::model::HostIssue::UnknownTag)
}

fn collect_visible_ranges(
    draft: &crate::model::SnapshotDraft<'_>,
    tags: &[(String, u32)],
) -> Result<Vec<crate::model::text::VisibleRange>, crate::model::HostIssue> {
    if draft.visible_spans.is_empty()
        || draft.visible_spans.len() > crate::model::MAX_VISIBILITY_ITEMS
    {
        return Err(crate::model::HostIssue::InvalidVisibility);
    }
    let mut ranges = Vec::with_capacity(draft.visible_spans.len());
    let mut previous_end = 0_u32;
    for span in draft.visible_spans {
        let start = line_for_tag(tags, span.start_tag)?;
        let end = line_for_tag(tags, span.end_tag)?;
        if start > end || start <= previous_end {
            return Err(crate::model::HostIssue::InvalidVisibility);
        }
        let end_exclusive = end
            .checked_add(1)
            .ok_or(crate::model::HostIssue::InvalidVisibility)?;
        ranges.push(crate::model::text::VisibleRange {
            lines: start..end_exclusive,
        });
        previous_end = end;
    }
    debug_assert_eq!(ranges.len(), draft.visible_spans.len());
    Ok(ranges)
}

fn collect_visible_boundaries(
    draft: &crate::model::SnapshotDraft<'_>,
    tags: &[(String, u32)],
) -> Result<Vec<crate::model::text::VisibleBoundary>, crate::model::HostIssue> {
    if draft.visible_boundaries.len() > crate::model::MAX_VISIBILITY_ITEMS {
        return Err(crate::model::HostIssue::InvalidVisibility);
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut boundaries = Vec::with_capacity(draft.visible_boundaries.len());
    for boundary in draft.visible_boundaries {
        let line = line_for_tag(tags, boundary.line_tag)?;
        if !seen.insert((line, boundary.side)) {
            return Err(crate::model::HostIssue::InvalidVisibility);
        }
        boundaries.push(crate::model::text::VisibleBoundary {
            line,
            side: boundary.side,
        });
    }
    boundaries.sort_by_key(|boundary| (boundary.line, boundary.side));
    debug_assert_eq!(boundaries.len(), draft.visible_boundaries.len());
    Ok(boundaries)
}

fn visibility_identity(
    ranges: &[crate::model::text::VisibleRange],
    boundaries: &[crate::model::text::VisibleBoundary],
    is_end_boundary_visible: bool,
) -> String {
    let mut hasher = blake3::Hasher::new();
    crate::planning::common::frame_part(
        &mut hasher,
        crate::planning::common::FRAME_DOMAIN_VISIBILITY,
    );
    for range in ranges {
        crate::planning::common::frame_part(&mut hasher, &range.lines.start.to_le_bytes());
        crate::planning::common::frame_part(&mut hasher, &range.lines.end.to_le_bytes());
    }
    for boundary in boundaries {
        crate::planning::common::frame_part(&mut hasher, &boundary.line.to_le_bytes());
        crate::planning::common::frame_part(&mut hasher, boundary.side.as_str().as_bytes());
    }
    crate::planning::common::frame_part(&mut hasher, &[u8::from(is_end_boundary_visible)]);
    hasher.finalize().to_hex().to_string()
}

fn admit_identities(
    draft: &crate::model::SnapshotDraft<'_>,
) -> Result<IdentitySet, crate::model::HostIssue> {
    Ok(IdentitySet {
        root: crate::planning::common::require_identity(crate::planning::common::IdentityInput {
            field: "root_identity",
            value: draft.root_identity,
        })?,
        resource: crate::planning::common::require_identity(
            crate::planning::common::IdentityInput {
                field: "resource_identity",
                value: draft.resource_identity,
            },
        )?,
        revision: crate::planning::common::require_identity(
            crate::planning::common::IdentityInput {
                field: "revision_identity",
                value: draft.revision_identity,
            },
        )?,
        metadata_policy: crate::planning::common::require_identity(
            crate::planning::common::IdentityInput {
                field: "metadata_policy_identity",
                value: draft.metadata_policy_identity,
            },
        )?,
        file_metadata: crate::planning::common::require_identity(
            crate::planning::common::IdentityInput {
                field: "file_metadata_identity",
                value: draft.file_metadata_identity,
            },
        )?,
    })
}

fn validate_base(
    draft: &crate::model::SnapshotDraft<'_>,
    identities: &IdentitySet,
    content_identity: &str,
) -> Result<(), crate::model::HostIssue> {
    if draft.writer_generation == 0 {
        return Err(crate::model::HostIssue::InvalidIdentity(
            "writer_generation",
        ));
    }
    if draft.base.resource_class() != crate::planning::common::FILE_RESOURCE_CLASS {
        return Err(crate::model::HostIssue::SnapshotShapeMismatch);
    }
    if draft.base.resource_identity() != identities.resource {
        return Err(crate::model::HostIssue::SnapshotShapeMismatch);
    }
    if draft.base.snapshot_hash() != content_identity {
        return Err(crate::model::HostIssue::SnapshotHashMismatch);
    }
    debug_assert_eq!(draft.base.resource_identity(), identities.resource);
    Ok(())
}

fn validate_shape(
    draft: &crate::model::SnapshotDraft<'_>,
    lines: &[crate::model::text::SourceLine],
    observed: crate::planning::syntax::ParsedFacts,
) -> Result<(), crate::model::HostIssue> {
    let line_count =
        u32::try_from(lines.len()).map_err(|_| crate::model::HostIssue::SnapshotShapeMismatch)?;
    if draft.base.line_count() != line_count {
        return Err(crate::model::HostIssue::SnapshotShapeMismatch);
    }
    if draft.base.normalization() != crate::planning::syntax::expected_normalization(observed.mode)
    {
        return Err(crate::model::HostIssue::SnapshotShapeMismatch);
    }
    if observed.mode != draft.line_ending_mode {
        return Err(crate::model::HostIssue::LineEndingMismatch);
    }
    if observed.mode == crate::model::LineEndingMode::Mixed && !draft.mixed_endings_admitted {
        return Err(crate::model::HostIssue::MixedLineEndingsDenied);
    }
    if observed.final_newline != draft.final_newline {
        return Err(crate::model::HostIssue::FinalNewlineMismatch);
    }
    debug_assert_eq!(draft.base.line_count(), line_count);
    Ok(())
}

/// Validate exact source bytes, tagged snapshot facts, and visibility metadata.
///
/// # Errors
///
/// Returns a bounded [`crate::model::HostIssue`] before any file mutation when
/// identities, source shape, tags, visibility, or policy facts conflict.
// r[impl agent_harness_runtime.hashed_edit_host.snapshot]
// r[impl agent_harness_runtime.hashed_edit_host.visibility]
pub fn admit_snapshot(
    draft: &crate::model::SnapshotDraft<'_>,
) -> Result<crate::model::AdmittedSnapshot, crate::model::HostIssue> {
    if draft.source_bytes.len() > crate::model::MAX_SOURCE_BYTES {
        return Err(crate::model::HostIssue::SourceTooLarge);
    }
    let identities = admit_identities(draft)?;
    let content_identity = crate::planning::common::hash_bytes(draft.source_bytes);
    validate_base(draft, &identities, &content_identity)?;
    let (lines, observed_shape) = crate::planning::syntax::parse_lines(draft.source_bytes)?;
    validate_shape(draft, &lines, observed_shape)?;
    let tags = collect_tags(draft);
    let visible_ranges = collect_visible_ranges(draft, &tags)?;
    let visible_boundaries = collect_visible_boundaries(draft, &tags)?;
    let visibility_identity = visibility_identity(
        &visible_ranges,
        &visible_boundaries,
        draft.end_boundary_visible,
    );
    let identity = crate::planning::common::hash_parts(
        crate::planning::common::FRAME_DOMAIN_SNAPSHOT,
        &[
            identities.root.as_bytes(),
            identities.resource.as_bytes(),
            identities.revision.as_bytes(),
            identities.metadata_policy.as_bytes(),
            identities.file_metadata.as_bytes(),
            &draft.writer_generation.to_le_bytes(),
            content_identity.as_bytes(),
            draft.base.identity_hash().as_bytes(),
            draft.line_ending_mode.as_str().as_bytes(),
            draft.final_newline.as_str().as_bytes(),
            draft.replacement_ending.as_str().as_bytes(),
            visibility_identity.as_bytes(),
        ],
    );
    debug_assert_eq!(
        crate::planning::common::hash_bytes(draft.source_bytes),
        content_identity
    );
    Ok(crate::model::AdmittedSnapshot {
        root_identity: identities.root,
        resource_identity: identities.resource,
        revision_identity: identities.revision,
        metadata_policy_identity: identities.metadata_policy,
        file_metadata_identity: identities.file_metadata,
        writer_generation: draft.writer_generation,
        source_bytes: draft.source_bytes.to_vec(),
        source_byte_count: draft.source_bytes.len() as u64,
        content_identity,
        base_identity: String::from(draft.base.identity_hash()),
        line_ending_mode: draft.line_ending_mode,
        final_newline: draft.final_newline,
        replacement_ending: draft.replacement_ending,
        lines,
        tags,
        visible_ranges,
        visible_boundaries,
        end_boundary_visible: draft.end_boundary_visible,
        visibility_identity,
        identity,
    })
}
