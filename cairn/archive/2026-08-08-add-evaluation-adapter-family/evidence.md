# Evidence

## Baseline

- The prerequisite `add-native-evaluation-core` is accepted and archived at `ff64459ec5537b70df6b4991ba21e3f3270cfb74`.
- `cargo test -p animus-adapters`: PASS, 620 unit tests and 2 host-control tests.
- Default-free and Wasm checks for `animus-adapters` and `animus-eval-core`: PASS.
- Harbor runtime tests with `harbor-eval`: PASS, 2 positive and negative tests.
- Harbor std-shell tests with `harbor-eval`: PASS, 2 positive and negative tests.

## Current owner inventory

- Task, taskset, recipe, episode, reward, resume, policy, and evaluation receipt meaning: `animus-eval-core`.
- Generic model and tool calls: `animus-adapters::{ModelProviderService, ToolExecutionService}` and their host implementations.
- Harbor environment and verifier calls: `animus-adapters::harbor::{EvaluationEnvironmentService, VerifierService}`.
- Harbor host dispatch: `animus-std::harbor`.
- Harbor matrix, ATIF projection, reward compatibility, and Harbor receipts: `animus-runtime::harbor_eval`.
- Transcript and replay authority: `animus-adapters::ledger::SessionLedgerService` and the runtime ledger owners.
- Execution ordering and resume scheduling: `animus-runtime::batch`.
- Role identity, capability intersection, and separation: `animus-runtime::agent_roles`.
- Artifact allowlists and manifest meaning: existing artifact and Harbor owners. The new family carries only exact manifest identities.
- Cancellation decisions: existing injected cancellation contracts. The new family carries only cancellation-scope identities and typed observations.
- Model-call capture and result append, replace, and reopen: previously had no focused generic evaluation owner. `animus-adapter-eval` now owns only the implementation-free port contracts.
- Redaction and generic safe evidence: `animus-adapter-safe`, native evaluation receipt checks, and Cairn evidence-safety policy.

## Reviewed source cohort

- Repository: `PrimeIntellect-ai/verifiers`.
- Revision: `1a997814e85a2e4f409add960aa852737388dbd8`.
- License: MIT.
- Ordered source-cohort identity: `b3:facd1eab5ae08491dbab9ae51a85aac73b90415e61030b551d1f7f3f3ffc5918`.
- Dialects: `verifiers/v1/dialects/base.py`, `chat.py`, `responses.py`, and `anthropic.py`.
- Interception: `verifiers/v1/interception/base.py`, `pool.py`, and `server.py`.
- Shared-state reference: `verifiers/v1/state.py`.
- Adopted: separate model and state capability identity, dialect-specific metadata, terminal-after-commit ordering, and exact non-streaming retry coalescing.
- Rejected: Python runtime and server, sockets, HTTP, SSE transport, provider bodies, secrets, tunnels, dynamic pools, BLAKE2b identity, one-gibibyte bodies, task payload service, mutable shared state, and last-write-wins replacement.

## Current verification

- Proposal gate receipt `1561cffd8f348bb71211142b8357e3b0c48e466626f8c06d24c0a937778ef4cf`: PASS.
- Design gate receipt `fee4955bc179769445663af0c3be94b1fbb89cc748bcfb476f413bfa3d3e8d75`: PASS.
- `cargo test -p animus-adapter-eval`: PASS, 6 positive and negative contract tests.
- Strict `animus-adapter-eval` Clippy: PASS.
- Focused crate Wasm check: PASS.
- Root no-default Wasm check with `evaluation-adapters`: PASS.
- Harbor bridge tests: PASS, 2 positive and negative tests.
- Strict `animus-adapters` Clippy with `evaluation`: PASS.
- Component catalog, runtime facade inventory, composition conformance, Cairn validation, formatting, and `git diff --check`: PASS.
- `checks.x86_64-linux.evaluation-adapter-family`: PASS after local fallback from the unavailable SSH builder.
- Tasks gate receipt `d46222eb6801e5da1c001c6c56dff7dfb83d0eb405f0e9f5db6c2d43230e0431`: PASS with 23 of 23 tasks complete.
- Final sync dry-run receipt `05b4f40973ce749a7bb5098a2644fc8f6676282262907fba7bb4527061268631`: PASS with seven added requirements and expected accepted-spec identity `2ebb76f256e59e6463925fe213b84278ef18c29acc1ab3799eca382f87e3a491`.
- Archive dry-run receipt `2d360927607c8b1658eae1bda1caa0cb9bdbe6fd19ba220b5cd3aa85b71cf123`: PASS.
- Sync execute receipt `cdcaab18b833b1964e0894db5ba6cbdc8346f22e613d5aac55b72faf337774fb`: PASS; accepted spec changed from `97bddae750df3830487987049f157d585021460cfebc039f1bcc9525279a8d29` to `2ebb76f256e59e6463925fe213b84278ef18c29acc1ab3799eca382f87e3a491`.
- Archive execute receipt `fae87b08b6497d43c59ea0427af347a1e26bb0782792c6ded1a30ec961e76a37`: PASS; archived at `cairn/archive/2026-08-08-add-evaluation-adapter-family/`.
- Post-archive focused rail, evidence safety, Cairn validation, archive placement, and diff checks: PASS.
- Roadmap validation remains blocked only by the pre-existing unclassified active changes `add-optional-basalt-authority-adapter` and `add-pijul-vcs-adapter`.

## Non-claims

This evidence does not establish provider transport, tool execution, verifier soundness, artifact authenticity, persistence durability, exactly-once effects, complete capture, benchmark validity, authoritative completion, production readiness, or release eligibility.
