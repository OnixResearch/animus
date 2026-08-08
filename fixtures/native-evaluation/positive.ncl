{
  schema = "animus.native-evaluation-positive-fixtures.v1",
  cases = [
    { name = "exact-taskset", expected = "planned" },
    { name = "bounded-lazy-taskset", selection_bound = 2, expected = "planned" },
    { name = "single-agent", expected = "planned" },
    { name = "best-of-n", candidates = 2, expected = "planned" },
    { name = "user-simulation", user_trainable = false, expected = "planned" },
    { name = "isolated-agentic-judge", separate_environment = true, expected = "planned" },
    { name = "shared-advisory-judge", advisory = true, expected = "planned" },
    { name = "finite-reward", value = 1.0, expected = "reduced" },
    { name = "exact-resume", terminal = true, safe = true, expected = "keep" },
    { name = "metadata-receipt", raw_content = false, expected = "projected" },
  ],
}
