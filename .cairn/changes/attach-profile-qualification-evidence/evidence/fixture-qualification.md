# Fixture qualification: attach-profile-qualification-evidence

- Subject: uncommitted review fixes on top of Animus `8d6f67d3` (Animus `4071c65e` plus the first #18 change) for [Animus #18](https://github.com/OnixResearch/animus/issues/18).
- Evidence label: `fixture`, with `live_qualification: false`. No model, provider, endpoint, or live evaluation ran. The attachments come from the deterministic scripted campaign-manager job.
- Toolchain: the flake-pinned dev shell (`nix develop`, rustc 1.96.1, cargo 1.96.2), with `CARGO_BUILD_JOBS=6`. The shell was built locally (`--option builders ''`) because the remote builder queue was saturated; its inputs are unchanged.

## Review fixes

The review of the first change found one P2 and five P3 issues. All are fixed (tasks 4.1–4.6):

- **Conflicts.** `qualify` now runs the claim checks (subject keys, task family, class) first, detects conflicts over every deduplicated attachment that passes them, and only then applies the currency checks (unusable, stale). `conflicts_survive_a_stale_unusable_or_changed_member` covers a member that expires over time, one that lacks a required component, and one whose environment changed. `conflicting_claims_over_one_run_are_not_qualified` covers the fixture with a required prompt.
- **Measures.** Coverage must satisfy `covered ≤ completed ≤ covered × repetitions`, and `time_to_acceptance_ms` must be `null` without an accepted completion. `denominators_and_costs_keep_failures_and_unknowns` covers both negatives.
- **Credential markers.** `_` is a boundary and matches the marker's `-`. `unsafe_material_cannot_be_represented` refuses `db_password:hunter2`, `sk_live_…`, `private_key:…`, `openai_api-key:…` and `auth_bearer-abc`, and accepts `task-family`, `risk-assessment` and `task_family`.
- **Producer.** `JobReport::task_set_identity` records the task set that ran. `task_family_attachment` returns `TaskSetMismatch` for another set and `TrialMismatch` unless the receipt planned exactly the replanned trials and each route trial appears once, as a failure or an `atif:` trajectory. `the_producer_refuses_task_sets_and_trials_the_job_did_not_run` covers an edited objective, renamed tasks, a shorter set, a relabelled report, and tampered planned counts, trajectories and failures. None of these panics.
- **Fixtures.** `redaction-host-path` now puts the path in the first `limitations` entry, so only the token rule rejects it. The rejected-completion fixture is renamed `rejected-dropped-from-denominator`. Every negative fixture test asserts its exact `Rejection.field`. Reordered root keys are refused at `attachment.canonical` and oversized bytes at `attachment.bytes`, in both the unit and fixture tests.
- **Provenance.** `Qualified` carries `provenance_label` (`EvidenceLabel::as_str()` gives `fixture` or `live`). The docs say `Qualified` is admissible evidence, not a favorable verdict, and list the remaining contract non-claims.

## Results

| Check | Result |
|---|---|
| `cargo check -p animus-runtime --no-default-features --features steel` | exit 0 |
| `cargo test -p animus-runtime --lib qualification` | exit 0: 11 passed, 0 failed |
| `cargo test -p animus-runtime --lib eval_feedback` | exit 0: 16 passed, 0 failed (11 qualification + 5 existing) |
| `cargo test -p animus-runtime` | exit 0: 599 passed, 0 failed, 0 ignored (597 unit + 2 `sessioncorefacade`) |
| `cargo test -p animus-campaign-manager-eval` | exit 0: 38 passed, 0 failed, 0 ignored across 14 test targets, including `qualification` (3) and `attachment` (5) |
| `cargo check --workspace --all-targets` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo fmt --all -- --check` | exit 0 |
| `nix build .#checks.x86_64-linux.campaign-manager-eval-octet` (Octet deny-all with `live`) | exit 0, `warning-only`: 0 errors, 3 warnings, all outside the touched files (`std::env::args` in the existing binary, twice, and the unknown `ambient_env` lint name in `animus-std`) |
| `scripts/cairn/check-runtime-facade-inventory --root .` | exit 0 |
| `scripts/cairn/check-evidence-safety --root .` | exit 0 |
| `scripts/cairn/check-component-catalog --root .` | exit 0 |
| `scripts/cairn/check-roadmap --root .` | exit 1 with one issue: the accepted pre-existing `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE add-acp-coding-agent`. This change is classified in `.cairn/roadmap.ncl`. |
| `cairn validate --root .` (the dev shell's `cairn`) | exit 0: `valid: true`, 0 issues |
| `cairn gate proposal\|design\|tasks attach-profile-qualification-evidence --root .` | exit 0 for each |

`check-nickel-configs` is not applicable: no file under `config/` changed. The two changed `.ncl` files are Cairn inventories, and their own rails cover them.

## Fixture identities

| Fixture | Identity |
|---|---|
| `fixtures/qualification-attachment/positive/signal-greedy.json` (3 of 3 accepted) | `blake3:46830a82f59932dbe48b167c23db8d999d93680edadbb037cc8cb309ee953658` |
| `fixtures/qualification-attachment/positive/explore-only.json` (1 of 3 accepted) | `blake3:f9cbbd02782db6fb1a6eee3cc40361fcb76eb2e60dd2bba18e2e287d7cfd4694` |

Each identity is `b3sum` over the file without its trailing newline. The review fixes left both files unchanged. `the_scripted_job_emits_the_checked_attachments` reproduces both byte for byte from a fresh scripted job. The negative fixtures are single-edit derivations of the positives, and each decodes as `malformed` at one asserted field: `noncanonical-whitespace` at `attachment.syntax`, `prefixed-profile-revision` at `subject.lattice_profile_revision`, `redaction-host-path` at `scope.limitations`, `authority-field` at `attachment.shape`, and `rejected-dropped-from-denominator` at `measures.task_coverage`.

## Criteria 66–75 (the issue's acceptance list, in order)

| # | Criterion | Status | Evidence |
|---|---|---|---|
| 66 | One existing native-evaluation fixture emits a complete attachment without a second engine | met (fixture) | `task_family_attachment` over the campaign-manager scripted job; `the_scripted_job_emits_the_checked_attachments` |
| 67 | Changing one component at a time prevents silent attribution | met | `stale_changed_and_unavailable_components_are_not_qualified` (prompt, route, tool implementation, permissions, context controls, environment, verifier, protocol); `applicability_requires_scoped_decisions_and_never_grants_authority` |
| 68 | Declared support, conformance, outcomes and admission stay separate | met (schema and validator) | Four independent classes; `cross_profile_scope_and_class_mismatches_are_not_qualified` shows no class substitutes for another. Kiln catalog presentation is not part of this change. |
| 69 | Equal-budget trials use unchanged independent checks and retain failures, cancellations, incomplete runs and unknown costs | met (fixture) | `compare_equal_budget` refuses changed controls; `denominators_and_costs_keep_failures_and_unknowns`; `produced_measures_keep_unknown_costs_and_compare_accepted_completions` |
| 70 | A faster route with fewer accepted changes cannot win on throughput or self-scores | met | No throughput or self-score field exists, and the `throughput_tokens_per_second` key is `malformed`; `equal_budget_comparison_counts_only_independently_accepted_completions` |
| 71 | Changed or unavailable verifiers and protocols make evidence unusable or historical | met | `changed_or_unavailable_evaluator_components_stop_qualification`; evaluator components cannot be waived |
| 72 | Malformed, cross-profile, stale, duplicate and conflicting attachments follow the rules; replay runs nothing | met | `duplicate_and_conflicting_sets_follow_replay_rules`, `conflicts_survive_a_stale_unusable_or_changed_member`, `checked_fixtures_qualify_only_their_exact_subject`, `conflicting_claims_over_one_run_are_not_qualified`, `reordered_and_oversized_fixture_bytes_are_malformed` |
| 73 | Favorable attachments cannot activate, widen authority, or bypass revocation or acceptance | met (contract) | `review_qualification` never activates; the authority field is `malformed`; reuse is never `Qualified` |
| 74 | Bounded exports keep redaction and separate provider-declared from verified identity | met | `unsafe_material_cannot_be_represented` (including `_`-separated credentials), `subject_keys_and_model_facts_are_strictly_spelled`, `checked_negative_fixtures_are_malformed` |
| 75 | Cairn requirements and rails with positive and negative fixtures; live experiments recorded separately | met for fixtures; live remains | Spec delta with 10 requirements, every one with `r[impl]` and `r[verify]`; live efficacy is task 3.3 |

## Remaining

- The NVIDIA skill-evidence follow-up in #18 (skill-set manifests, scanner coverage, activation, paired and incomplete arms) is not implemented (task 3.2).
- No live task-family efficacy experiment was run (task 3.3).
- Kiln #60 catalog presentation and Lattice #25 consumption are consumer work in other repositories.

## Non-claims

The fixture results establish no live model behavior, benchmark validity, production readiness, verifier soundness beyond Campaign history replay, or deployment admission. The fixture Lattice subject keys are synthetic test-binding keys, not values from a Lattice deployment. A `Qualified` answer over these fixtures says that admissible fixture evidence exists, not that the route performed well. The contract non-claims in `docs/qualification-attachments.md` still hold: hosted-model version drift is invisible to `validate_attachment`, the credential denylist can reject some capability names, lists cap at 32 entries, and attachments are unsigned.
