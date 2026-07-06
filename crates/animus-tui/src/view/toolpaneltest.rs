// Tool panel tests — separated to reduce excessive_file_length.
// Included from view.rs inside `mod tool_panel_tests { use super::*; include!(...); }`

// -----------------------------------------------------------------------
// Tool call card tests
// -----------------------------------------------------------------------

#[test]
fn card_from_request_has_tool_name_and_input() {
    let request = animus_adapters::ToolRequest::new("search", "query: hello")
        .expect("request should validate");
    let card = ToolCallCard::from_request(&request);
    assert_eq!(card.tool_name(), "search");
    assert!(card.input_summary().contains("query: hello"));
    assert!(card.output().is_none());
}

#[test]
fn card_output_updates() {
    let request = animus_adapters::ToolRequest::new("read", "path: Cargo.toml")
        .expect("request should validate");
    let mut card = ToolCallCard::from_request(&request);
    card.set_output("[package]\nname = \"animus\"");
    assert_eq!(card.output(), Some("[package]\nname = \"animus\""));
}

#[test]
fn card_progress_updates() {
    let request = animus_adapters::ToolRequest::new("build", ".")
        .expect("request should validate");
    let mut card = ToolCallCard::from_request(&request);
    card.set_progress(5, Some(10));
    assert_eq!(card.progress(), Some(ToolCardProgress { completed: 5, total: Some(10) }));
}

#[test]
fn card_toggle_collapsed() {
    let request = animus_adapters::ToolRequest::new("test", "all")
        .expect("request should validate");
    let mut card = ToolCallCard::from_request(&request);
    assert!(!card.is_collapsed());
    card.toggle_collapsed();
    assert!(card.is_collapsed());
    card.toggle_collapsed();
    assert!(!card.is_collapsed());
}

#[test]
fn long_input_is_truncated() {
    let long_input = "a".repeat(500);
    let request = animus_adapters::ToolRequest::new("tool", &long_input)
        .expect("request should validate");
    let card = ToolCallCard::from_request(&request);
    assert!(card.input_summary().len() <= 120);
}

// -----------------------------------------------------------------------
// Diff view tests
// -----------------------------------------------------------------------

#[test]
fn diff_view_renders_lines() {
    let diff = DiffView::new(
        "editor",
        vec![
            DiffLine::Context(String::from("line 1")),
            DiffLine::Added(String::from("new line")),
            DiffLine::Removed(String::from("old line")),
        ],
    );
    assert_eq!(diff.source_label(), "editor");
    assert_eq!(diff.lines().len(), 3);
}

#[test]
fn diff_view_toggle_collapsed() {
    let mut diff = DiffView::new("editor", vec![]);
    assert!(!diff.is_collapsed());
    diff.toggle_collapsed();
    assert!(diff.is_collapsed());
}

// -----------------------------------------------------------------------
// Tool panel state tests
// -----------------------------------------------------------------------

#[test]
fn empty_panel_shows_no_tools() {
    let panel = ToolPanelState::new();
    assert!(panel.active_cards().is_empty());
    assert!(!panel.has_pending_approval());
}

#[test]
fn add_card_to_panel() {
    let mut panel = ToolPanelState::new();
    let request = animus_adapters::ToolRequest::new("build", ".")
        .expect("request should validate");
    panel.add_card(ToolCallCard::from_request(&request));
    assert_eq!(panel.active_cards().len(), 1);
}

#[test]
fn replacing_card_updates_in_place() {
    let mut panel = ToolPanelState::new();
    let req1 = animus_adapters::ToolRequest::new("tool", "first")
        .expect("request should validate");
    panel.add_card(ToolCallCard::from_request(&req1));
    let req2 = animus_adapters::ToolRequest::new("tool", "second")
        .expect("request should validate");
    panel.add_card(ToolCallCard::from_request(&req2));
    assert_eq!(panel.active_cards().len(), 1);
    assert_eq!(panel.active_cards()[0].input_summary(), "second");
}

#[test]
fn approval_prompt_tracking() {
    let mut panel = ToolPanelState::new();
    assert!(!panel.has_pending_approval());
    panel.set_approval("Allow write to /tmp?", &[String::from("approve"), String::from("deny")]);
    assert!(panel.has_pending_approval());
    assert_eq!(panel.approval_choices().len(), 2);
}

#[test]
fn clear_resets_panel() {
    let mut panel = ToolPanelState::new();
    let request = animus_adapters::ToolRequest::new("tool", "test")
        .expect("request should validate");
    panel.add_card(ToolCallCard::from_request(&request));
    panel.set_approval("Proceed?", &[String::from("yes")]);
    panel.clear();
    assert!(panel.active_cards().is_empty());
    assert!(!panel.has_pending_approval());
}

// -----------------------------------------------------------------------
// Snapshot tests
// -----------------------------------------------------------------------

#[test]
fn empty_panel_snapshot() {
    let panel = ToolPanelState::new();
    let snapshot = render_tool_panel_snapshot(&panel);
    assert_eq!(snapshot, "(no active tools)");
}

#[test]
fn panel_snapshot_shows_tools_and_approval() {
    let mut panel = ToolPanelState::new();
    let request = animus_adapters::ToolRequest::new("search", "query: hello")
        .expect("request should validate");
    panel.add_card(ToolCallCard::from_request(&request));
    panel.set_approval("Allow?", &[String::from("approve"), String::from("deny")]);
    let snapshot = render_tool_panel_snapshot(&panel);
    assert!(snapshot.contains("tool: search"));
    assert!(snapshot.contains("-- approval needed --"));
    assert!(snapshot.contains("choice: approve"));
}

// -----------------------------------------------------------------------
// Negative tests
// -----------------------------------------------------------------------

#[test]
fn denied_approval_clears_prompt() {
    let mut panel = ToolPanelState::new();
    panel.set_approval("Allow?", &[String::from("approve"), String::from("deny")]);
    assert!(panel.has_pending_approval());
    panel.clear();
    assert!(!panel.has_pending_approval());
}

#[test]
fn mismatched_correlation_does_not_panic() {
    let mut panel = ToolPanelState::new();
    let req1 = animus_adapters::ToolRequest::new("tool_a", "first")
        .expect("request should validate");
    let req2 = animus_adapters::ToolRequest::new("tool_b", "second")
        .expect("request should validate");
    panel.add_card(ToolCallCard::from_request(&req1));
    panel.add_card(ToolCallCard::from_request(&req2));
    assert_eq!(panel.active_cards().len(), 2);
}

#[test]
fn oversized_output_does_not_panic() {
    let mut panel = ToolPanelState::new();
    let request = animus_adapters::ToolRequest::new("tool", "input")
        .expect("request should validate");
    let mut card = ToolCallCard::from_request(&request);
    let huge_output = "x".repeat(100_000);
    card.set_output(&huge_output);
    panel.add_card(card);
    assert!(panel.active_cards()[0].output().is_some());
}