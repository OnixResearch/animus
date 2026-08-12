# Evidence: Remora-derived maintenance adapter families

## Scope

This evidence covers two optional authority-free adapter families:

- `animus-adapter-change-request` for forge-neutral change-request and review meaning;
- `animus-adapter-maintenance` for dependency inventory, candidate, advisory, grouping, and update-plan meaning.

The reviewed Remora source is `git@github.com:OnixResearch/remora.git` at revision `e3c857c857f4aa170d8a5b6286aad5855db541be`. Its declared license is `MIT OR Apache-2.0`.

The reviewed source cohort identity is `b3:a084a8a06727b1fae59377d34754f224065351e550caadf4e12c0bbcdcc26e42`.

## Ownership result

The two reusable crates own deterministic DTO validation, exact request and observation binding, pure maintenance and review planning, typed failure reduction, and metadata-only receipt validation.

Nine separate ports describe dependency inventory, candidate discovery, advisory lookup, workspace mutation, validation, change-request source observation, review analysis, change-request publication, and review publication.

The host retains registry, advisory, filesystem, workspace, VCS, forge, scheduler, process, credential, network, clock, and persistence authority. Existing Animus schedule, queue, orchestration, model, quality, validation, workspace, VCS, authority, and receipt owners retain their current meaning.

## Implemented evidence

Positive and negative tests cover:

- routine grouping and isolated security or major updates;
- ecosystem and manifest compatibility;
- complete candidate-material binding and tampered groups;
- exact repository, branch, commit, generation, implementation, and correlation identities;
- diff totals, observed sides, range anchors, and publication bounds;
- unavailable, denied, unsupported, cancelled, timed-out, malformed, stale, mismatch, overflow, and infrastructure outcomes;
- safe validation profiles and unsafe command rejection;
- metadata-only receipts, content-leak rejection, and unsupported-claim rejection;
- lowercase canonical BLAKE3 identities and uppercase rejection; and
- direct-crate, optional-facade, no-default, and Wasm isolation.

The component catalog contains separate `adapters.change-request-contracts` and `adapters.dependency-maintenance` entries. The runtime facade inventory contains the two optional feature families without default dependency leakage.

## Deterministic checks

The focused rail is `scripts/cairn/check-remora-adapter-families --root .`. Its local bounded receipt is `target/cairn/focused-rails/remora-adapter-families.md`.

Final results before lifecycle mutation:

- Change-request unit tests: 8 passed.
- Maintenance unit tests: 8 passed.
- Focused Rust, formatting, strict Clippy, no-default, Wasm, facade, dependency-isolation, static-authority, component-catalog, and runtime-facade checks: PASS.
- Complete focused rail: PASS.
- `nix build .#checks.x86_64-linux.remora-adapter-families --no-link -L`: PASS.
- `nix flake check -L --builders ''`: PASS for all 117 flake checks. This local rerun reused 116 completed store results and completed the last derivation locally.
- Cairn validation: PASS with one active change and no issues.

The first broad flake run used the configured remote builder. It made no failing observation but stalled for more than five hours in the final derivation after a cache timeout. The run was stopped. The same broad gate then passed locally with remote builders disabled.

Lifecycle planning evidence before final task closure:

- Sync dry-run receipt: `e02b865a1e84c5925dfb3a2ddf034948deeac60be8437f92c30646dc0d4045c5`.
- Sync plan hash: `3f0e670596e174f53b97c00da72fd99f7abb4525a70c6333562e53d351d4b7a8`.
- The sync plan adds all eight `agent_harness_runtime.remora_adapters.*` requirement IDs.
- Expected accepted-spec identity after sync: `2b439def78c10d0bee2af5c1498301de84e09023232e8f6ccf028a10710d44b5`.
- Initial archive dry-run receipt: `213de9113e118865fcaba147cff595ce6d603471a05e329c4aee0c770e866212`.
- The initial archive review correctly blocked only because the two closeout tasks were open.

Final lifecycle evidence:

- Final tasks gate receipt: `379fc2fc0b51dd5eb37e71061d8bf1fcea0ccff0b95ee8659d249102ccfffe10`, with 16 of 16 tasks complete.
- Final sync dry-run receipt: `7d7e434165798130d2cf7d1d823463371f79e75256c365feaa302f9bc437ea91`.
- Final archive dry-run receipt: `bd2e84d4a4745c95cacd5e2517d7a29609c299d1e7cf5496d500736af7e2e749`.
- Accepted-spec sync execution receipt: `6be37392d28648c0ca26a277b5a01aac7ecb1c9801e7fba7ccbd761c3aaccd45`.
- Sync mutation manifest: `45cd2458d660d86bb80a748c3c4eb6c9719c753b318a8a98ac1780e4d85ff1f6`.
- The accepted specification contains all eight root `agent_harness_runtime.remora_adapters.*` requirement IDs.
- Accepted-spec identity after sync: `2b439def78c10d0bee2af5c1498301de84e09023232e8f6ccf028a10710d44b5`.
- Archive execution receipt: `2a638de012d7369bc38498e0f2c5189439eddfee89a93c0c7189ce3b7b616db8`.
- Archive mutation manifest: `efaa6c1df7b5d60b9f2969747dfa57c6eee7cdd4a4fbfb317ee535c250b86ad0`.
- Archive path: `.cairn/archive/2026-08-12-adapt-remora-maintenance-adapters/`.
- Archived evidence identity at mutation: `61c3e5f1637166a433fb01d0a5882f6af2f346c2b7d8de7c6f6420d468781dde`.

## Non-claims

This evidence does not establish registry freshness, advisory completeness, dependency graph completeness, candidate correctness, update correctness, validation sufficiency, provider delivery, review quality, merge safety, production readiness, or release eligibility.

It does not establish Dependabot parity. Vulnerability alerts, automatic security remediation, broad ecosystem support, transitive dependency graphs, private registries, cooldowns, and release intelligence remain outside this adaptation.
