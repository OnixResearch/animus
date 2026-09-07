//! Current repository-policy and whole-report compatibility controls.
#![cfg(test)]

type TestResult = Result<(), Box<dyn std::error::Error>>;
const OLD_POLICY: &str = "2c0b44345b9d2e2b3eff5fa7db7a14b9681c6bfcf7a4bb651f13bcadc1f91757";
const SUBJECT: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const STALE_SUBJECT: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const FIXTURES: &str = "fixtures/authoritative-completion";

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &std::path::Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_slice(&std::fs::read(path)?)?)
}

fn policy() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    read(&root().join("cairn-policy/generated/cairn-policy.json"))
}

fn report(subject: &str, is_old: bool) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let directory = root().join(FIXTURES);
    let directory = if is_old {
        directory.join("previous-policy")
    } else {
        directory
    };
    let contract = read(&directory.join("cairn-contract.json"))?;
    let results = read(&directory.join("cairn-results.json"))?;
    Ok(serde_json::to_value(
        cairn_core::evaluate_authoritative_agent_completion(
            &contract,
            &results,
            subject,
            &cairn_core::canonical_json_hash(&policy()?),
        ),
    )?)
}

fn is_matching(receipt: &serde_json::Value, expected: &serde_json::Value) -> bool {
    let current_policy =
        serde_json::json!(animus_runtime::authoritative_completion::PINNED_CAIRN_POLICY_HASH);
    let pairs = [
        (&receipt["completion"], expected),
        (
            &receipt["completion_receipt_hash"],
            &expected["receipt_hash"],
        ),
        (&receipt["valid"], &expected["valid"]),
        (&receipt["verdict"], &expected["verdict"]),
        (&receipt["current_policy_hash"], &current_policy),
    ];
    pairs.iter().all(|(actual, expected)| actual == expected) && receipt["checks_executed"] == false
}

#[test]
fn current_policy_matches_runtime_and_generated_registry() -> TestResult {
    // r[verify agent_harness_runtime.completion_policy.coherence]
    let identity = cairn_core::canonical_json_hash(&policy()?);
    assert_eq!(
        identity,
        animus_runtime::authoritative_completion::PINNED_CAIRN_POLICY_HASH
    );
    let registry = read(&root().join("config/generated/agent-check-registry.json"))?;
    assert_eq!(registry["cairn_policy_hash"], identity);
    assert_ne!(identity, OLD_POLICY);
    Ok(())
}

#[test]
fn cli_completion_reports_match_the_pinned_core() -> TestResult {
    // r[verify agent_harness_runtime.completion_policy.parity]
    // The maintained rail supplies fresh responses. Ordinary Cargo runs use saved fixtures.
    let directory = match std::env::var_os("ANIMUS_COMPLETION_RECEIPT_DIR") {
        Some(path) => std::path::PathBuf::from(path),
        None => root().join(FIXTURES),
    };
    for (file, subject, is_old, is_valid) in [
        ("cairn-positive-receipt.json", SUBJECT, false, true),
        (
            "cairn-stale-subject-receipt.json",
            STALE_SUBJECT,
            false,
            false,
        ),
        ("cairn-stale-policy-receipt.json", SUBJECT, true, false),
    ] {
        let expected = report(subject, is_old)?;
        assert_eq!(expected["valid"], is_valid);
        let receipt = read(&directory.join(file))?;
        assert!(
            is_matching(&receipt, &expected),
            "{file}: whole report mismatch"
        );
    }
    Ok(())
}

#[test]
fn internally_matching_old_policy_inputs_are_rejected() -> TestResult {
    let expected = report(SUBJECT, true)?;
    assert_eq!(expected["valid"], false);
    let diagnostics = expected["diagnostics"]
        .as_array()
        .ok_or("diagnostic array")?;
    assert!(
        diagnostics
            .iter()
            .any(|row| row["code"] == "verification_contract.policy_drift")
    );
    Ok(())
}

#[test]
fn a_changed_policy_cannot_borrow_current_completion() -> TestResult {
    let mut changed = policy()?;
    changed["agent_completion_policy"]["passing_status"] = serde_json::json!("failed");
    let hash = cairn_core::canonical_json_hash(&changed);
    assert_ne!(
        hash,
        animus_runtime::authoritative_completion::PINNED_CAIRN_POLICY_HASH
    );
    let contract = read(&root().join(FIXTURES).join("cairn-contract.json"))?;
    let results = read(&root().join(FIXTURES).join("cairn-results.json"))?;
    let rejected =
        cairn_core::evaluate_authoritative_agent_completion(&contract, &results, SUBJECT, &hash);
    assert!(!rejected.valid);
    assert!(
        rejected
            .diagnostics
            .iter()
            .any(|row| row.code == "verification_contract.policy_drift")
    );
    Ok(())
}

#[test]
fn an_altered_receipt_cannot_pass_the_comparator() -> TestResult {
    let expected = report(SUBJECT, false)?;
    let original = read(&root().join(FIXTURES).join("cairn-positive-receipt.json"))?;
    assert!(is_matching(&original, &expected));
    for field in [
        "valid",
        "verdict",
        "completion_receipt_hash",
        "current_policy_hash",
        "checks_executed",
    ] {
        let mut altered = original.clone();
        altered[field] = serde_json::Value::Null;
        assert!(!is_matching(&altered, &expected), "{field}");
    }
    let mut altered = original;
    altered["completion"]["non_claims"] = serde_json::json!([]);
    assert!(!is_matching(&altered, &expected));
    Ok(())
}
