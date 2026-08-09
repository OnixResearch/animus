//! Cross-crate ATIF validation and Harbor projection conformance.
#![cfg(all(feature = "harbor-eval", feature = "harbor-atif-json"))]

use animus::{harbor_atif, harbor_evaluation};

const FIRST_DOCUMENT: &[u8] = br#"{
  "schema_version":"ATIF-v1.7",
  "trajectory_id":"integration-a",
  "agent":{"name":"animus","version":"1"},
  "steps":[{"step_id":1,"source":"user","message":"input"}]
}"#;
const SECOND_DOCUMENT: &[u8] = br#"{
  "schema_version":"ATIF-v1.7",
  "trajectory_id":"integration-b",
  "agent":{"name":"animus","version":"1"},
  "steps":[{"step_id":1,"source":"user","message":"other"}]
}"#;

#[test]
fn validated_atif_projection_reaches_harbor_without_terminal_inference() {
    let policy_bytes = include_bytes!("../config/generated/harbor-atif-profile.json");
    let policy = harbor_atif::decode_atif_policy_json(policy_bytes)
        .expect("generated policy must revalidate");
    let decoded =
        harbor_atif::decode_atif_json(FIRST_DOCUMENT, &policy).expect("valid document must decode");
    let imported = harbor_atif::import_validated_trajectory(&decoded.validated, &policy)
        .expect("validated document must import");

    let projected = harbor_evaluation::project_validated_atif(&decoded.validated, &imported)
        .expect("Harbor must consume the validated projection");
    assert_eq!(projected.steps, 1);
    assert_eq!(projected.identity, decoded.validated.semantic_identity());
}

#[test]
fn harbor_projection_rejects_a_mismatched_import_identity() {
    let policy_bytes = include_bytes!("../config/generated/harbor-atif-profile.json");
    let policy = harbor_atif::decode_atif_policy_json(policy_bytes)
        .expect("generated policy must revalidate");
    let first =
        harbor_atif::decode_atif_json(FIRST_DOCUMENT, &policy).expect("first document must decode");
    let second = harbor_atif::decode_atif_json(SECOND_DOCUMENT, &policy)
        .expect("second document must decode");
    let imported = harbor_atif::import_validated_trajectory(&second.validated, &policy)
        .expect("second document must import");

    assert_eq!(
        harbor_evaluation::project_validated_atif(&first.validated, &imported),
        Err(harbor_evaluation::AtifError::Correlation)
    );
}
