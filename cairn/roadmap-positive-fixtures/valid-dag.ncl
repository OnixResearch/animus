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
        "DAG_BATCH 0 foundation-change",
        "DAG_BATCH 1 runtime-change",
        "DAG_BATCH 2 ui-surface",
        "SAFETY_PATH ui-surface foundational agent_harness_runtime.cairn_policy.closeout runtime agent_harness_runtime.validation",
        "ROADMAP_REVIEW projection=markdown fields=change-id,role,tier,prerequisites,issue-codes,counts redaction=applied",
      ],
  roadmap_lines =
    std.string.join
      "\n"
      [
        "tier|base||Base",
        "tier|runtime|base|Runtime",
        "tier|ui|runtime|UI",
        "active|foundation-change",
        "active|runtime-change",
        "active|ui-surface",
        "accepted|agent_harness_runtime.cairn_policy.closeout",
        "accepted|agent_harness_runtime.validation",
        "change|foundation-change|foundational|base|agent_harness_runtime.cairn_policy.closeout|",
        "change|runtime-change|core-runtime|runtime|foundation-change,agent_harness_runtime.validation|",
        "change|ui-surface|product-surface|ui|runtime-change|",
      ],
} | Fixture
