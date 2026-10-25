//! The evaluation binary's scripted report is metadata only.

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.report]
fn the_scripted_report_holds_metadata_only() -> Result<(), String> {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_animus-campaign-manager-eval"))
        .output()
        .map_err(|error| error.to_string())?;
    let printed = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    assert!(output.status.success(), "{printed}");
    assert!(printed.starts_with("CAMPAIGN_MANAGER_EVAL status=completed\n"));
    assert!(printed.contains("planned=9 completed=9 failed=0\n"));
    assert!(printed.contains("route=model:scripted:signal-greedy completed=3 found=1.000"));
    for leaked in ["Objective", "includes/", "signal 9", "scores", "Reply with"] {
        assert!(!printed.contains(leaked), "the report leaks {leaked}");
    }
    let refused = std::process::Command::new(env!("CARGO_BIN_EXE_animus-campaign-manager-eval"))
        .arg("--unknown")
        .output()
        .map_err(|error| error.to_string())?;
    assert!(!refused.status.success());
    assert!(refused.stdout.is_empty());
    Ok(())
}
