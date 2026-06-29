use alloc::string::ToString;

const USER_ID: &str = "user-1";
const ASSISTANT_ID: &str = "assistant-1";
const TOOL_ID: &str = "tool-1";
const SUMMARY_ID: &str = "summary-1";
const USAGE_ID: &str = "usage-1";
const RECEIPT_ID: &str = "receipt-1";
const UNSUPPORTED_ID: &str = "unsupported-1";
const ADAPTER_ID: &str = "adapter-1";
const SESSION_LABEL: &str = "session-a";
const TOOL_LABEL: &str = "search";
const USER_TEXT: &str = "user asks a question";
const ASSISTANT_TEXT: &str = "assistant drafts an answer";
const TOOL_TEXT: &str = "tool result summary";
const SUMMARY_TEXT: &str = "bounded prior context";
const UNSUPPORTED_KIND: &str = "opaque-entry";
const ADAPTER_KIND: &str = "adapter-owned-entry";
const INPUT_TOKENS: u32 = 5;
const OUTPUT_TOKENS: u32 = 8;
const HASH_TEXT: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const EXPECTED_CONTEXT_ITEMS: usize = 4;
const EXPECTED_ENTRY_COUNT: u32 = 6;
const EXPECTED_CONTEXT_COUNT: u32 = 4;
const EXPECTED_ONE: u32 = 1;

#[test]
fn replay_orders_visible_entries_and_counts_metadata() {
    let receipt = sample_receipt();
    let entries = [
        super::Entry::user(USER_ID, USER_TEXT).expect("user entry should build"),
        super::Entry::assistant(ASSISTANT_ID, ASSISTANT_TEXT)
            .expect("assistant entry should build"),
        super::Entry::tool(TOOL_ID, TOOL_LABEL, TOOL_TEXT).expect("tool entry should build"),
        super::Entry::summary(SUMMARY_ID, SUMMARY_TEXT).expect("summary entry should build"),
        super::Entry::usage(USAGE_ID, INPUT_TOKENS, OUTPUT_TOKENS)
            .expect("usage entry should build"),
        super::Entry::safe_receipt(RECEIPT_ID, receipt).expect("receipt entry should build"),
    ];

    let replayed = super::replay(&entries).expect("ledger replay should succeed");
    let context = replayed.context();
    let counts = replayed.counts();

    assert_eq!(context.len(), EXPECTED_CONTEXT_ITEMS);
    assert_eq!(context[0].role(), super::ContextRole::User);
    assert_eq!(context[0].content(), USER_TEXT);
    assert_eq!(context[1].role(), super::ContextRole::Assistant);
    assert_eq!(context[2].role(), super::ContextRole::Tool);
    assert_eq!(context[3].role(), super::ContextRole::System);
    assert_eq!(counts.entries(), EXPECTED_ENTRY_COUNT);
    assert_eq!(counts.model_visible_items(), EXPECTED_CONTEXT_COUNT);
    assert_eq!(counts.summaries(), EXPECTED_ONE);
    assert_eq!(counts.usage_observations(), EXPECTED_ONE);
    assert_eq!(counts.safe_receipts(), EXPECTED_ONE);
}

#[test]
fn rejects_unsupported_entry_without_payload() {
    let entries = [super::Entry::unsupported(UNSUPPORTED_ID, UNSUPPORTED_KIND)
        .expect("unsupported marker should build")];

    let error = super::replay(&entries).expect_err("unsupported entry should fail closed");

    assert_eq!(
        error,
        super::Error::UnsupportedEntry {
            entry_id: alloc::string::String::from(UNSUPPORTED_ID),
            kind_label: alloc::string::String::from(UNSUPPORTED_KIND),
            redaction: crate::ReceiptRedaction::OutputBounded,
        }
    );
    assert!(!error.to_string().contains("payload"));
}

#[test]
fn rejects_adapter_owned_entry_without_payload() {
    let entries =
        [super::Entry::adapter_owned(ADAPTER_ID, ADAPTER_KIND)
            .expect("adapter marker should build")];

    let error = super::replay(&entries).expect_err("adapter-owned entry should fail closed");

    assert_eq!(
        error,
        super::Error::UnsupportedEntry {
            entry_id: alloc::string::String::from(ADAPTER_ID),
            kind_label: alloc::string::String::from(ADAPTER_KIND),
            redaction: crate::ReceiptRedaction::OutputBounded,
        }
    );
    assert!(!error.to_string().contains("raw"));
}

#[test]
fn empty_input_fails_closed() {
    let error = super::replay(&[]).expect_err("empty ledger should fail closed");

    assert_eq!(error, super::Error::EmptyLedger);
    assert_eq!(
        error.to_string(),
        "ledger replay requires at least one entry"
    );
}

#[test]
fn missing_service_returns_absent_outcome() {
    let request = super::Request::new(SESSION_LABEL).expect("request should build");
    let outcome = super::missing_service(&request);

    assert_eq!(outcome.kind(), crate::OutcomeKind::Absent);
    assert_eq!(
        outcome.receipt().service_kind(),
        crate::ServiceKind::SessionLedger
    );
    assert_eq!(
        outcome.receipt().capability(),
        crate::CapabilityClass::SessionPersistence
    );
    assert!(!outcome.is_completed());
}

fn sample_receipt() -> crate::SafeReceipt {
    let hash = crate::ArtifactHash::new(HASH_TEXT).expect("hash should parse");
    let labels = ["ledger-replay"];
    let hashes = [hash];
    let redactions = [crate::ReceiptRedaction::PayloadHashOnly];
    crate::SafeReceipt::new(
        crate::SafeReceiptDraft::new(
            crate::ReceiptStatus::Completed,
            crate::ReceiptIssueCode::ProvenanceObservation,
            crate::ReceiptOperation::ProvenanceObservation,
        )
        .with_labels(&labels)
        .with_artifact_hashes(&hashes)
        .with_redactions(&redactions),
    )
    .expect("sample receipt should build")
}
