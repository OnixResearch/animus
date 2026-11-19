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

#[test]
// r[verify agent_harness_runtime.frozen_policy.evaluation]
fn the_frozen_policy_report_states_its_decision_and_holds_metadata_only() -> Result<(), String> {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_animus-campaign-manager-eval"))
        .arg("--frozen-policy")
        .output()
        .map_err(|error| error.to_string())?;
    let printed = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    assert!(output.status.success(), "{printed}");
    assert!(printed.starts_with("FROZEN_POLICY_COMPARISON status=completed\n"));
    assert!(printed.contains("protocol=animus.campaign-manager-frozen-policy-eval.v1\n"));
    assert!(printed.contains("provenance=fixture\n"));
    assert!(printed.contains("planned=33 completed=33 failed=0\n"));
    assert!(printed.contains("arm role=scorer route=model:frozen-policy:"));
    assert!(printed.contains("\ndecision="));
    for leaked in ["Objective", "includes/", "signal 9", "Reply with"] {
        assert!(!printed.contains(leaked), "the report leaks {leaked}");
    }
    Ok(())
}

#[cfg(not(feature = "live"))]
#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.boundary]
fn live_mode_needs_a_build_with_the_live_feature() -> Result<(), String> {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_animus-campaign-manager-eval"))
        .args([
            "--live",
            "openrouter",
            "--model",
            "model",
            "--credential-stdin",
        ])
        .output()
        .map_err(|error| error.to_string())?;
    assert!(!output.status.success());
    assert!(
        output.stdout.is_empty(),
        "a refused invocation prints no report"
    );
    let error = String::from_utf8(output.stderr).map_err(|error| error.to_string())?;
    assert!(
        error.contains("live mode needs a build with `--features live`"),
        "{error}"
    );
    Ok(())
}
