let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_DEPENDENCY_CYCLE",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|cycle-a",
        "active|cycle-b",
        "change|cycle-a|core-runtime|runtime|cycle-b|",
        "change|cycle-b|core-runtime|runtime|cycle-a|",
      ],
} | Fixture
