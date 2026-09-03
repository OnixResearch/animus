# Post-archive validation

Animus pins Cairn revision `5e1fe08d62149b286ff583eb056e655fe92e3cf1` and uses the checked repository policy.

The policy drift check passed against the immutable Cairn source. Cairn validation passed with no active change.

The profile rejected planned sync before completion evidence existed. The retained receipt has BLAKE3 `82f4062cbc8363df926b9b42267b0b10c51e4c617f9888a4fc447ba23b833474`.

The fresh-state task-gate receipt has BLAKE3 `665a264e59159a5c111a39316534ee62a8a89c6093b9223ce55ad346d929d607`.

The authoritative agent-completion check receipt has BLAKE3 `1f607700285c521deda3520ef0f15b4a8685d4318776c9cacc51cd79c497546a`.

Sync mutation manifest `4c06b6a4c2af2f763faa72332c6dff6057ab0242eec92a97475513b21d3b318f` accepted the requirement.

Archive mutation manifest `5ad70532dae5c6aa4e889661f095937c6736d2ec3b15b56d2070624d338385a4` retained all referenced evidence.

Positive and negative runner tests pass. Focused Clippy passes with warnings denied.

The final authoritative agent-completion Nix check passes. It includes positive and negative completion paths.

Repository traceability still reports existing unrelated debt. The new `animus.lifecycle.outcome_machine_profile` requirement is not missing or dangling.

These checks do not establish agent correctness or Animus release eligibility.
