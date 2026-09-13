//! Portable counts and refusal at the model-call sequence boundary.

#[test]
fn fixture_counts_retain_empty_and_populated_inputs() -> Result<(), Box<dyn std::error::Error>> {
    const RESPONSE_BOUND_BYTES: u64 = 1;
    let empty = super::FixtureStreamProvider::new([], RESPONSE_BOUND_BYTES);
    assert_eq!(empty.remaining()?, 0);
    let populated = super::FixtureStreamProvider::new([Vec::new()], RESPONSE_BOUND_BYTES);
    assert_eq!(populated.remaining()?, 1);
    Ok(())
}

#[test]
fn model_call_sequence_advances_without_reusing_an_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = super::continuation::SharedProviderState::default();
    let first = state.next_model_call_id()?;
    let next = state.next_model_call_id()?;
    assert_ne!(first, next);
    assert!(first.starts_with(super::MODEL_CALL_PREFIX));
    Ok(())
}
