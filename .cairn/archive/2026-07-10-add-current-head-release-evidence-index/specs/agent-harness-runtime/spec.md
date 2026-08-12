# agent-harness-runtime Change Spec: Current-head release evidence index

## Requirements

### Requirement: Current-head release evidence generator is shell-owned

r[agent_harness_runtime.current_head_release_evidence_index.generator] Animus MUST provide a shell-owned current-head release evidence index generator that renders safe index lines from explicit check statuses and VCS head metadata without running hidden validation suites.

#### Scenario: Generator records supplied evidence only

r[agent_harness_runtime.current_head_release_evidence_index.generator.supplied_checks]
- GIVEN a maintainer supplies check modes, statuses, and missing-mode reasons
- WHEN the generator renders an evidence index
- THEN the output contains head, check, missing-mode, claim, and non-claim records derived from those supplied values
- AND it does not run workspace tests, Clippy, flake checks, live providers, UI dogfood, plugin runtime, VM checks, release tagging, or hidden harnesses.

### Requirement: Current-head claims are explicit and reject dirty evidence

r[agent_harness_runtime.current_head_release_evidence_index.claims] The generator MUST emit `current-head-pure-evidence` only when explicitly requested, and the focused rail MUST prove that dirty heads, stale receipts, and missing required pure modes are rejected by the release evidence validator.

#### Scenario: Dirty head cannot become current-head proof

r[agent_harness_runtime.current_head_release_evidence_index.claims.dirty]
- GIVEN generated evidence records a dirty worktree and a current-head claim
- WHEN release evidence validation runs
- THEN validation rejects the index with a dirty-head diagnostic
- AND the evidence may remain draft or historical evidence only.

### Requirement: Packaged dogfood smoke is a pure selected mode

r[agent_harness_runtime.current_head_release_evidence_index.packaged_smoke] The release evidence schema and checker MUST recognize `packaged-headless-dogfood-smoke` as a pure selected evidence mode that can be tied to the recorded head without making live-provider, TUI, daemon, plugin, persistence, production, or Clankers-parity claims.

#### Scenario: Packaged smoke receipt is tied to head

r[agent_harness_runtime.current_head_release_evidence_index.packaged_smoke.head_match]
- GIVEN an index includes `packaged-headless-dogfood-smoke` as a passing pure check
- WHEN the index is rendered for a recorded head
- THEN the check is classified as pure and records whether it matches that head
- AND optional host-dependent surfaces remain non-claims unless separately indexed.

### Requirement: Actual-head drafts stay local and safe

r[agent_harness_runtime.current_head_release_evidence_index.draft] The current-head rail SHOULD write an actual-head draft index under ignored local output when a Git worktree is available, and MUST omit raw prompts, raw model outputs, tool bodies, provider payloads, credentials, private paths, unbounded logs, and full check transcripts.

#### Scenario: Local draft records missing modes without proof claim

r[agent_harness_runtime.current_head_release_evidence_index.draft.partial]
- GIVEN only focused rails and packaged dogfood smoke are supplied as passing evidence
- WHEN the actual-head draft is generated
- THEN missing required pure modes are recorded as `missing_mode` entries
- AND no `current-head-pure-evidence` claim is emitted.

### Requirement: Current-head release evidence verification is focused

r[agent_harness_runtime.current_head_release_evidence_index.verification] The current-head generator change MUST include positive and negative generated fixtures for passing current-head evidence, dirty-head rejection, missing-mode rejection, stale-receipt rejection, actual-head draft generation, docs, Nix check wiring, Cairn validation, and Cairn gates before archive.

#### Scenario: Focused rail validates generated indexes

r[agent_harness_runtime.current_head_release_evidence_index.verification.focused_rail]
- GIVEN maintainers run the current-head release evidence focused rail
- WHEN generated synthetic and actual-head draft cases complete
- THEN the rail names positive, dirty, missing-mode, stale, and draft evidence results
- AND it records skipped live-provider, TUI, daemon, plugin, VM, production, and release-tagging checks as non-claims or future rungs.

### Requirement: Current-head release evidence docs preserve non-claims

r[agent_harness_runtime.current_head_release_evidence_index.docs] Release evidence documentation MUST explain how to generate draft and current-head indexes, when `--claim-current-head` is allowed, which pure modes are required, which optional surfaces remain non-claims, and why the generator is not a hidden test runner.

#### Scenario: Maintainer reads claim guidance

r[agent_harness_runtime.current_head_release_evidence_index.docs.claim_guidance]
- GIVEN a maintainer reads release evidence documentation
- WHEN they prepare a current-head index
- THEN the docs instruct them to run or supply every required pure mode before adding `--claim-current-head`
- AND omitted optional host-dependent surfaces remain listed as non-claims.
