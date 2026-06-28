let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|unclassified-change",
      ],
} | Fixture
