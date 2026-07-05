let Fixture = {
  expected_issue | String,
  public_export_lines | String,
  inventory_lines | String,
}
in
{
  expected_issue = "OK",
  public_export_lines =
    std.string.join
      "\n"
      [
        "animus-core|Intent",
        "animus-adapters|ModelAdapter",
        "animus-std|run",
      ],
  inventory_lines =
    std.string.join
      "\n"
      [
        "entry|animus-core::Intent|animus-core|crates/animus-core/src/lib.rs|reusable-contract|none|stable|validated intent DTO|compat|none|||compatibility",
        "entry|animus-adapters::ModelAdapter|animus-adapters|crates/animus-adapters/src/lib.rs|host-injection-surface|injected-only|stable|model port trait|compat|injected-only|||tool",
        "entry|animus-std::run|animus-std|crates/animus-std/src/lib.rs|shell-adapter|shell-owned|stable|std shell runner|compat|shell-owned|||compatibility",
      ],
} | Fixture
