let Fixture = {
  expected_issue | String,
  public_export_lines | String,
  inventory_lines | String,
}
in
{
  expected_issue = "RUNTIME_FACADE_REUSABLE_AMBIENT_AUTHORITY",
  public_export_lines =
    std.string.join
      "\n"
      [
        "animus-core|ReadCredential",
      ],
  inventory_lines =
    std.string.join
      "\n"
      [
        "entry|animus-core::ReadCredential|animus-core|crates/animus-core/src/lib.rs|reusable-contract|none|stable|credential reader example",
      ],
} | Fixture
