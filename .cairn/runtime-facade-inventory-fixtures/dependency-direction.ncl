let Fixture = {
  expected_issue | String,
  public_export_lines | String,
  inventory_lines | String,
}
in
{
  expected_issue = "RUNTIME_FACADE_DEPENDENCY_DIRECTION",
  public_export_lines =
    std.string.join
      "\n"
      [
        "animus-core|Intent",
        "animus-std|run",
      ],
  inventory_lines =
    std.string.join
      "\n"
      [
        "entry|animus-core::Intent|animus-core|crates/animus-core/src/lib.rs|reusable-contract|none|stable|validated intent DTO|compat|none|animus-std::run||compatibility",
        "entry|animus-std::run|animus-std|crates/animus-std/src/lib.rs|shell-adapter|shell-owned|stable|std shell runner|compat|shell-owned|||compatibility",
      ],
} | Fixture
