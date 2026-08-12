let Fixture = {
  expected_issue | String,
  public_export_lines | String,
  inventory_lines | String,
}
in
{
  expected_issue = "RUNTIME_FACADE_UNCLASSIFIED_EXPORT",
  public_export_lines =
    std.string.join
      "\n"
      [
        "animus-core|Intent",
        "animus-adapters|ModelAdapter",
      ],
  inventory_lines =
    std.string.join
      "\n"
      [
        "entry|animus-core::Intent|animus-core|crates/animus-core/src/lib.rs|reusable-contract|none|stable|validated intent DTO|compat|none|||compatibility",
      ],
} | Fixture
