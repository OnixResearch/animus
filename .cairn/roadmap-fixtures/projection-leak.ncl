let Fixture = {
  expected_issue | String,
  roadmap_lines | String,
}
in
{
  expected_issue = "ROADMAP_PROJECTION_LEAK",
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|/home/example/private-change",
        "change|/home/example/private-change|core-runtime|runtime||",
      ],
} | Fixture
