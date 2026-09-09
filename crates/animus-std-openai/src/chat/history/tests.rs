//! Exact history retention limits: never silently trim a conversation.

#[test]
fn message_count_accepts_the_bound_and_rejects_empty_or_excess_history() {
    let mut messages = vec![
        serde_json::json!({"role": "user", "content": "synthetic"});
        super::MAX_HISTORY_MESSAGES
    ];
    assert!(super::check_messages(&messages).is_ok());
    messages.push(serde_json::json!({"role": "user", "content": "excess"}));
    assert!(super::check_messages(&messages).is_err());
    assert!(super::check_messages(&[]).is_err());
}

#[test]
fn serialized_bytes_accept_the_bound_and_reject_one_more_byte() {
    let empty = vec![serde_json::Value::String(String::new())];
    let framing_bytes = serde_json::to_vec(&empty).expect("JSON framing").len();
    let text_bytes = super::MAX_HISTORY_BYTES - framing_bytes;
    let accepted = vec![serde_json::Value::String("x".repeat(text_bytes))];
    assert_eq!(
        serde_json::to_vec(&accepted).expect("JSON").len(),
        super::MAX_HISTORY_BYTES
    );
    assert!(super::check_messages(&accepted).is_ok());
    let rejected = vec![serde_json::Value::String("x".repeat(text_bytes + 1))];
    assert!(super::check_messages(&rejected).is_err());
}
