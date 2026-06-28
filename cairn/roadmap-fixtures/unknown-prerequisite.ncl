let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_UNKNOWN_PREREQUISITE_ID",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|runtime-change",
        "change|runtime-change|core-runtime|runtime|missing-prerequisite|",
      ],
} | Fixture
