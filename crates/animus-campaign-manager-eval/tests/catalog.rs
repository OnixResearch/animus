//! The task set: the checked Nickel projection is admitted, and every kind of drift is refused.

mod support;

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.tasks]
fn the_embedded_task_set_is_admitted() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    assert_eq!(set.schema, animus_campaign_manager_eval::TASK_SET_SCHEMA);
    assert_eq!(
        set.harbor.revision,
        animus_adapters::harbor::HARBOR_REVISION
    );
    assert_ne!(set.environment, set.verifier);
    for task in &set.tasks {
        let targets = task
            .candidates
            .iter()
            .chain(&task.offers)
            .filter(|candidate| candidate.is_target)
            .count();
        assert_eq!(targets, 1, "{} has one target", task.identity);
    }
    let terms = set
        .terms()
        .map_err(|_| support::Failure::Unexpected("the mandate terms were refused"))?;
    assert_eq!(terms.grants.len(), 6);
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.tasks]
fn drifted_task_sets_are_refused() -> Result<(), support::Failure> {
    let pristine = support::task_set_json()?;
    let cases = drift_cases(&pristine);
    assert_eq!(support::refusal(&pristine)?, None);
    for (pointer, replacement, expected) in cases {
        let value = changed(&pristine, pointer, replacement)?;
        assert_eq!(support::refusal(&value)?, Some(expected), "{pointer}");
    }
    Ok(())
}

/// One drifted field per case: its JSON pointer, its new value, and the expected refusal.
fn drift_cases(
    pristine: &serde_json::Value,
) -> Vec<(
    &'static str,
    serde_json::Value,
    animus_campaign_manager_eval::TaskError,
)> {
    vec![
        (
            "/extra",
            serde_json::json!(1),
            animus_campaign_manager_eval::TaskError::Malformed,
        ),
        (
            "/schema",
            serde_json::json!("other.v1"),
            animus_campaign_manager_eval::TaskError::Drift,
        ),
        (
            "/harbor/revision",
            serde_json::json!("0000"),
            animus_campaign_manager_eval::TaskError::Drift,
        ),
        (
            "/verifier",
            pristine["environment"].clone(),
            animus_campaign_manager_eval::TaskError::SharedVerifier,
        ),
        (
            "/tasks/0/identity",
            serde_json::json!("task with spaces"),
            animus_campaign_manager_eval::TaskError::Identity,
        ),
        (
            "/tasks/1/identity",
            pristine["tasks"][0]["identity"].clone(),
            animus_campaign_manager_eval::TaskError::Duplicate,
        ),
        (
            "/tasks/0/candidates/1/key",
            pristine["tasks"][0]["candidates"][0]["key"].clone(),
            animus_campaign_manager_eval::TaskError::Duplicate,
        ),
        (
            "/tasks/0/candidates/0/is_target",
            serde_json::json!(true),
            shaped(),
        ),
        (
            "/tasks/0/candidates/2/is_target",
            serde_json::json!(false),
            shaped(),
        ),
        (
            "/tasks/0/candidates/0/signal",
            serde_json::json!(10),
            shaped(),
        ),
        (
            "/tasks/0/candidates/0/label",
            serde_json::json!("x".repeat(81)),
            shaped(),
        ),
        (
            "/tasks/0/objective",
            serde_json::json!("two\nlines"),
            shaped(),
        ),
    ]
}

/// The refusal of a malformed first task.
fn shaped() -> animus_campaign_manager_eval::TaskError {
    animus_campaign_manager_eval::TaskError::Shape {
        task: String::from("task:json-decoder-hang"),
    }
}

/// `pristine` with the value at `pointer` replaced, or a new top-level key inserted.
fn changed(
    pristine: &serde_json::Value,
    pointer: &str,
    replacement: serde_json::Value,
) -> Result<serde_json::Value, support::Failure> {
    let mut value = pristine.clone();
    if let Some(slot) = value.pointer_mut(pointer) {
        *slot = replacement;
        return Ok(value);
    }
    let key = pointer.trim_start_matches('/');
    value
        .as_object_mut()
        .ok_or(support::Failure::Unexpected(
            "the task set is not an object",
        ))?
        .insert(String::from(key), replacement);
    Ok(value)
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.tasks]
fn bounds_on_turns_and_offers_are_enforced() -> Result<(), support::Failure> {
    let pristine = support::task_set_json()?;
    let mut long = pristine.clone();
    long["tasks"][0]["turns"] =
        serde_json::json!(animus_campaign_manager_eval::MAX_TURNS.saturating_add(1));
    let mut crowded = pristine.clone();
    let offer = crowded["tasks"][0]["offers"][0].clone();
    let offers: Vec<serde_json::Value> = (0..=animus_campaign_manager::MAX_OFFERS)
        .map(|index| {
            let mut copy = offer.clone();
            copy["key"] = serde_json::json!(format!("offer-{index}"));
            copy
        })
        .collect();
    crowded["tasks"][0]["offers"] = serde_json::Value::Array(offers);
    let expected = Some(animus_campaign_manager_eval::TaskError::Shape {
        task: String::from("task:json-decoder-hang"),
    });
    assert_eq!(support::refusal(&long)?, expected);
    assert_eq!(support::refusal(&crowded)?, expected);
    Ok(())
}
