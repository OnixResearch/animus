# Fixture qualification: attach-profile-qualification-evidence

- Subject: uncommitted work on top of Animus `4071c65e` for [Animus #18](https://github.com/OnixResearch/animus/issues/18).
- Evidence label: `fixture`, with `live_qualification: false`. No model, provider, endpoint, or live evaluation ran. The attachments come from the deterministic scripted campaign-manager job.
- Toolchain: the flake-pinned dev shell (`nix develop`, rustc 1.96.1, cargo 1.96.2), with `CARGO_BUILD_JOBS=6`.

## Results

| Check | Result |
|---|---|
| `cargo check -p animus-runtime --no-default-features --features steel` | exit 0 |
| `cargo test -p animus-runtime --lib qualification` | exit 0: 10 passed, 0 failed |
| `cargo test -p animus-runtime --lib eval_feedback` | exit 0: 15 passed, 0 failed (10 qualification + 5 existing) |
| `cargo test -p animus-runtime` | exit 0: 598 passed, 0 failed, 0 ignored |
| `cargo test -p animus-campaign-manager-eval` | exit 0: 36 passed, 0 failed across 10 integration suites, including `qualification` (2) and `attachment` (4) |
| `cargo check --workspace --all-targets` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo clippy -p animus-runtime --all-targets -- -D warnings`, after the credential-marker boundary fix | exit 0 |
| `cargo fmt --all -- --check` | exit 0 |
| `nix build .#checks.x86_64-linux.campaign-manager-eval-octet` (Octet deny-all with `live`) | exit 0, `warning-only`: 0 errors, 3 warnings, all outside the touched files (`std::env::args` in the existing binary, and the unknown `ambient_env` lint name in `animus-std`) |
| `scripts/cairn/check-runtime-facade-inventory --root .` | exit 0 |
| `scripts/cairn/check-evidence-safety --root .` | exit 0 |
| `scripts/cairn/check-component-catalog --root .` | exit 0 |
| `scripts/cairn/check-roadmap --root .` | exit 1 with one issue: the accepted pre-existing `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE add-acp-coding-agent`. This change is classified in `.cairn/roadmap.ncl`. |
| `cairn validate --root .` (packaged `/nix/store/4wwkqy9mwkd09lzb35w0s5gcng5yczmx-cairn/bin/cairn`) | exit 0: `valid: true`, 0 issues |
| `cairn gate proposal\|design\|tasks attach-profile-qualification-evidence --root .` | exit 0 for each |

`check-nickel-configs` is not applicable: no file under `config/` changed. The two changed `.ncl` files are Cairn inventories, and their own rails cover them.

## Fixture identities

| Fixture | Identity |
|---|---|
| `fixtures/qualification-attachment/positive/signal-greedy.json` (3 of 3 accepted) | `blake3:46830a82f59932dbe48b167c23db8d999d93680edadbb037cc8cb309ee953658` |
| `fixtures/qualification-attachment/positive/explore-only.json` (1 of 3 accepted) | `blake3:f9cbbd02782db6fb1a6eee3cc40361fcb76eb2e60dd2bba18e2e287d7cfd4694` |

Each identity is `b3sum` over the file without its trailing newline. `the_scripted_job_emits_the_checked_attachments` reproduces both files byte for byte from a fresh scripted job. The negative fixtures (`noncanonical-whitespace`, `prefixed-profile-revision`, `redaction-host-path`, `authority-field`, `failures-dropped-from-denominator`) are single-edit derivations of the positives, and every one decodes as `malformed`.

## Criteria 66–75 (the issue's acceptance list, in order)

| # | Criterion | Status | Evidence |
|---|---|---|---|
| 66 | One existing native-evaluation fixture emits a complete attachment without a second engine | met (fixture) | `task_family_attachment` over the campaign-manager scripted job; `the_scripted_job_emits_the_checked_attachments` |
| 67 | Changing one component at a time prevents silent attribution | met | `stale_changed_and_unavailable_components_are_not_qualified` (prompt, route, tool implementation, permissions, context controls, environment, verifier, protocol); `applicability_requires_scoped_decisions_and_never_grants_authority` |
| 68 | Declared support, conformance, outcomes and admission stay separate | met (schema and validator) | Four independent classes; `cross_profile_scope_and_class_mismatches_are_not_qualified` shows no class substitutes for another. Kiln catalog presentation is not part of this change. |
| 69 | Equal-budget trials use unchanged independent checks and retain failures, cancellations, incomplete runs and unknown costs | met (fixture) | `compare_equal_budget` refuses changed controls; `denominators_and_costs_keep_failures_and_unknowns`; `produced_measures_keep_unknown_costs_and_compare_accepted_completions` |
| 70 | A faster route with fewer accepted changes cannot win on throughput or self-scores | met | No throughput or self-score field exists, and the `throughput_tokens_per_second` key is `malformed`; `equal_budget_comparison_counts_only_independently_accepted_completions` |
| 71 | Changed or unavailable verifiers and protocols make evidence unusable or historical | met | `changed_or_unavailable_evaluator_components_stop_qualification`; evaluator components cannot be waived |
| 72 | Malformed, cross-profile, stale, duplicate and conflicting attachments follow the rules; replay runs nothing | met | `duplicate_and_conflicting_sets_follow_replay_rules`, `checked_fixtures_qualify_only_their_exact_subject`, `conflicting_claims_over_one_run_are_not_qualified` |
| 73 | Favorable attachments cannot activate, widen authority, or bypass revocation or acceptance | met (contract) | `review_qualification` never activates; the authority field is `malformed`; reuse is never `Qualified` |
| 74 | Bounded exports keep redaction and separate provider-declared from verified identity | met | `unsafe_material_cannot_be_represented`, `subject_keys_and_model_facts_are_strictly_spelled`, `checked_negative_fixtures_are_malformed` |
| 75 | Cairn requirements and rails with positive and negative fixtures; live experiments recorded separately | met for fixtures; live remains | Spec delta with 10 requirements, every one with `r[impl]` and `r[verify]`; live efficacy is task 3.3 |

## Remaining

- The NVIDIA skill-evidence follow-up in #18 (skill-set manifests, scanner coverage, activation, paired and incomplete arms) is not implemented (task 3.2).
- No live task-family efficacy experiment was run (task 3.3).
- Kiln #60 catalog presentation and Lattice #25 consumption are consumer work in other repositories.

## Non-claims

The fixture results establish no live model behavior, benchmark validity, production readiness, verifier soundness beyond Campaign history replay, or deployment admission. The fixture Lattice subject keys are synthetic test-binding keys, not values from a Lattice deployment.
