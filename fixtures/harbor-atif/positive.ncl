let profile = (import "../../config/harbor-atif-profile.ncl").harbor_atif_profile in
{
  schema = "animus.harbor-atif-positive-fixtures.v1",
  provenance = {
    repository = profile.source_repository,
    release = profile.harbor_release,
    revision = profile.harbor_revision,
    license = profile.source_license,
    schema = profile.atif_schema,
    derivation = "minimized-synthetic-from-reviewed-rfc-model-validator-and-golden-cohort",
  },
  cases = [
    { name = "text-steps", entrypoints = ["decode_atif_json", "validate_trajectory"], expected = "accepted" },
    { name = "multimodal-image-descriptor", entrypoints = ["validate_trajectory"], expected = "unresolved_image" },
    { name = "tool-observation-correlation", entrypoints = ["validate_trajectory"], expected = "accepted" },
    { name = "finite-metrics", entrypoints = ["validate_trajectory"], expected = "accepted" },
    { name = "copied-context", entrypoints = ["import_validated_trajectory"], expected = "dataset_ineligible" },
    { name = "no-llm-dispatch", entrypoints = ["validate_trajectory"], expected = "accepted_without_metrics_or_reasoning" },
    { name = "embedded-subagent", entrypoints = ["validate_trajectory", "import_validated_trajectory"], expected = "trajectory_id_lineage" },
    { name = "external-subagent", entrypoints = ["validate_trajectory"], expected = "unresolved_subagent" },
    { name = "continuation-descriptor", entrypoints = ["validate_trajectory"], expected = "unresolved_continuation" },
    { name = "bounded-extra", entrypoints = ["validate_trajectory"], expected = "accepted_then_omitted" },
    { name = "safe-import", entrypoints = ["import_validated_trajectory", "build_atif_receipt"], expected = "loss_reported" },
    { name = "safe-export", entrypoints = ["export_trajectory_snapshot", "encode_atif_json"], expected = "revalidated_without_invention" },
    { name = "semantic-round-trip", entrypoints = ["decode_atif_json", "encode_atif_json"], expected = "same_typed_semantic_identity" },
  ],
}
