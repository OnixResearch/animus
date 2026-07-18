//! Animus consumer conformance for the pinned standalone host-control core.

use animus_adapters::{
    HostControlError, HostOutputReadDraft, HostOutputSource, HostOutputView, HostSelector,
    HostSelectorDraft, HostTopologySnapshot, HostTopologySnapshotDraft, HostTruncationStatus,
    SelectorLifetime, TopologyCommand, TopologyCommandDraft,
};

const HOST_ID: &str = "host:animus";
const OTHER_HOST_ID: &str = "host:other";
const PANE_ID: &str = "pane:main";
const KIND_PANE: &str = "pane";
const LIFETIME_LIVE: &str = "live";
const OP_FOCUS: &str = "focus";
const SOURCE_RECENT: &str = "recent";
const REDACTION_CLEAN: &str = "clean";
const OUTPUT: &str = "first\nsecond\nthird";
const LIVE_REVISION: u64 = 7;
const STALE_REVISION: u64 = 6;
const LINE_BUDGET: u32 = 2;

fn selector(host_id: &str, revision: u64) -> HostSelector {
    HostSelector::validate(HostSelectorDraft {
        host_id,
        resource_id: PANE_ID,
        kind: KIND_PANE,
        lifetime: LIFETIME_LIVE,
        revision,
    })
    .expect("fixture selector should validate")
}

#[test]
fn compatibility_facade_preserves_topology_and_bounded_output_behavior() {
    let selected = selector(HOST_ID, LIVE_REVISION);
    let snapshot = HostTopologySnapshot::validate(HostTopologySnapshotDraft {
        host_id: HOST_ID,
        revision: LIVE_REVISION,
        selectors: core::slice::from_ref(&selected),
        labels: &[],
        supported_operations: &[OP_FOCUS],
    })
    .expect("fixture snapshot should validate");
    let command = TopologyCommand::validate(
        &snapshot,
        TopologyCommandDraft {
            operation: OP_FOCUS,
            selector: Some(&selected),
            label: None,
        },
    )
    .expect("supported command should validate");
    let output = HostOutputView::new(HostOutputReadDraft {
        source: SOURCE_RECENT,
        body: OUTPUT,
        visible_line_budget: LINE_BUDGET,
        redaction_status: REDACTION_CLEAN,
        full_output_reference: None,
    })
    .expect("bounded clean output should validate");

    assert_eq!(command.selector(), Some(&selected));
    assert_eq!(selected.lifetime(), SelectorLifetime::Live);
    assert_eq!(output.source(), HostOutputSource::Recent);
    assert_eq!(output.truncation_status(), HostTruncationStatus::Truncated);
    assert!(output.content_hash().is_some());
}

#[test]
fn compatibility_facade_rejects_stale_cross_host_and_unbounded_inputs() {
    let selected = selector(HOST_ID, LIVE_REVISION);
    let stale = selector(HOST_ID, STALE_REVISION);
    let cross_host = selector(OTHER_HOST_ID, LIVE_REVISION);
    let snapshot = HostTopologySnapshot::validate(HostTopologySnapshotDraft {
        host_id: HOST_ID,
        revision: LIVE_REVISION,
        selectors: core::slice::from_ref(&selected),
        labels: &[],
        supported_operations: &[OP_FOCUS],
    })
    .expect("fixture snapshot should validate");

    for (candidate, expected) in [
        (&stale, HostControlError::StaleSelector),
        (&cross_host, HostControlError::CrossHostSelector),
    ] {
        let error = TopologyCommand::validate(
            &snapshot,
            TopologyCommandDraft {
                operation: OP_FOCUS,
                selector: Some(candidate),
                label: None,
            },
        )
        .expect_err("invalid selector should fail closed");
        assert_eq!(error, expected);
    }

    let unbounded = HostOutputView::new(HostOutputReadDraft {
        source: SOURCE_RECENT,
        body: OUTPUT,
        visible_line_budget: 0,
        redaction_status: REDACTION_CLEAN,
        full_output_reference: None,
    })
    .expect_err("zero output budget should fail closed");
    assert_eq!(unbounded, HostControlError::UnboundedRead);
}
