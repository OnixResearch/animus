let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_DUPLICATE_CHANGE_ID",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|duplicate-change",
        "change|duplicate-change|core-runtime|runtime||",
        "change|duplicate-change|core-runtime|runtime||",
      ],
} | Fixture
