let Fixture = {
  expected_issue | String,
  public_export_lines | String,
  inventory_lines | String,
}
in
{
  expected_issue = "RUNTIME_FACADE_UNSAFE_EVIDENCE_FIELD",
  public_export_lines =
    std.string.join
      "\n"
      [
        "animus-core|Intent",
      ],
  inventory_lines =
    std.string.join
      "\n"
      [
        "entry|animus-core::Intent|animus-core|crates/animus-core/src/lib.rs|reusable-contract|none|stable|RAW_PROMPT: unsafe note|compat|none|||compatibility",
      ],
} | Fixture
