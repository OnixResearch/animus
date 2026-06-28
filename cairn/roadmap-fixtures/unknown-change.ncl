let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_UNKNOWN_CHANGE_ID",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|known-change",
        "change|ghost-change|core-runtime|runtime||",
      ],
} | Fixture
