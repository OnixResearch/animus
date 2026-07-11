//! Compatibility checks between direct family boundaries and the broad facade.

const MODEL_PROMPT: &str = "intent: preserve adapter compatibility";
const MALFORMED_RECORD: &str = "<BoundaryRecord";

#[test]
fn direct_adapter_boundary_and_compatibility_facade_preserve_behavior() {
    let request =
        animus_adapters::ModelRequest::new(MODEL_PROMPT).expect("model request should validate");
    let direct = animus_preserves::adapter::AdapterMessage::from_model_request(&request);
    let direct_text =
        animus_preserves::adapter::to_text(&direct).expect("direct message should encode");
    let direct_request = animus_preserves::adapter::from_text(&direct_text)
        .expect("direct message should decode")
        .try_into_model_request()
        .expect("direct message should revalidate");

    let legacy = animus_preserves::BoundaryMessage::from_model_request(&request);
    let legacy_text =
        animus_preserves::to_versioned_text(&legacy).expect("facade message should encode");
    let legacy_request = animus_preserves::from_versioned_text(&legacy_text)
        .expect("facade message should decode")
        .try_into_model_request()
        .expect("facade message should revalidate");

    assert_eq!(direct_request, request);
    assert_eq!(legacy_request, request);
}

#[test]
fn direct_and_compatibility_paths_both_fail_closed_on_malformed_records() {
    assert!(animus_preserves::adapter::from_text(MALFORMED_RECORD).is_err());
    assert!(animus_preserves::from_versioned_text(MALFORMED_RECORD).is_err());
}
