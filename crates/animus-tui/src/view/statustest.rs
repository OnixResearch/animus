// Status rail tests — separated to reduce file length.
// Included from view.rs inside `mod status_rail_tests { use super::*; include!(...); }`

fn idle_rail() -> StatusRailState {
    StatusRailState::idle()
}

#[test]
fn idle_rail_has_sensible_defaults() {
    let rail = idle_rail();
    assert_eq!(rail.session_label(), UNNAMED_SESSION);
    assert_eq!(rail.model_label(), NO_MODEL_SELECTED);
    assert_eq!(rail.status_text(), "idle");
    assert!((rail.context_ratio() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn session_label_updates() {
    let mut rail = idle_rail();
    rail.set_session("workspace-session-1");
    assert_eq!(rail.session_label(), "workspace-session-1");
}

#[test]
fn available_sessions_listed() {
    let mut rail = idle_rail();
    rail.set_available_sessions(vec![
        String::from("session-a"),
        String::from("session-b"),
    ]);
    assert_eq!(rail.available_sessions().len(), 2);
}

#[test]
fn model_label_bounded() {
    let mut rail = idle_rail();
    let long = "x".repeat(200);
    rail.set_model(&long);
    assert!(rail.model_label().len() <= MAX_MODEL_LABEL_CHARS);
}

#[test]
fn context_ratio_clamped() {
    let mut rail = idle_rail();
    rail.set_context(1.5, "over budget");
    assert!((rail.context_ratio() - 1.0).abs() < f64::EPSILON);
    rail.set_context(-0.5, "negative");
    assert!((rail.context_ratio() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn status_text_updates() {
    let mut rail = idle_rail();
    rail.set_status("streaming");
    assert_eq!(rail.status_text(), "streaming");
}

#[test]
fn cost_and_git_are_optional() {
    let rail = idle_rail();
    assert!(rail.cost_text().is_empty());
    assert!(rail.git_branch_text().is_empty());
}

#[test]
fn idle_rail_snapshot() {
    let rail = idle_rail();
    let snapshot = render_status_rail_snapshot(&rail);
    assert!(snapshot.contains("session: (unnamed session)"));
    assert!(snapshot.contains("status: idle"));
}

#[test]
fn populated_rail_snapshot() {
    let mut rail = idle_rail();
    rail.set_session("work");
    rail.set_model("gpt-5.5");
    rail.set_context(0.75, "high");
    rail.set_status("streaming");
    rail.set_cost("~$0.02/turn");
    rail.set_git_branch("main");
    let snapshot = render_status_rail_snapshot(&rail);
    assert!(snapshot.contains("session: work"));
    assert!(snapshot.contains("model: gpt-5.5"));
    assert!(snapshot.contains("status: streaming"));
    assert!(snapshot.contains("git: main"));
}

#[test]
fn missing_session_does_not_panic() {
    let mut rail = idle_rail();
    rail.set_session("");
    assert_eq!(rail.session_label(), "");
}

#[test]
fn oversize_context_does_not_panic() {
    let mut rail = idle_rail();
    let long = "x".repeat(200);
    rail.set_context(0.5, &long);
    assert!(rail.context_label().len() <= MAX_GAUGE_LABEL_WIDTH);
}

#[test]
fn empty_available_sessions() {
    let rail = idle_rail();
    assert!(rail.available_sessions().is_empty());
}

#[test]
fn model_label_does_not_grow_unbounded() {
    let mut rail = idle_rail();
    let long = "x".repeat(200);
    rail.set_model(&long);
    assert!(rail.model_label().len() <= MAX_MODEL_LABEL_CHARS);
}