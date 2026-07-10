let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_MALFORMED_RECORD",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|malformed-change",
        "change|malformed-change|core-runtime",
      ],
} | Fixture
