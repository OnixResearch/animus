let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_UNKNOWN_OVERLAP_OWNER",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|known-change",
        "change|known-change|core-runtime|runtime||",
        "overlap|unknown-change-reference|ghost-change|known-change",
      ],
} | Fixture
