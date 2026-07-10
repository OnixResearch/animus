let Fixture = {
  expected_contains | String,
  roadmap_lines | String,
}
in
{
  expected_contains =
    std.string.join
      "\n"
      [
        "blocked dependents: 1",
        "BLOCKED child-change waits_for parent-change status active-prerequisite",
        "DAG_BATCH 0 parent-change",
        "DAG_BATCH 1 child-change",
      ],
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|runtime||Runtime",
        "active|parent-change",
        "active|child-change",
        "change|parent-change|core-runtime|runtime||",
        "change|child-change|core-runtime|runtime|parent-change|",
      ],
} | Fixture
