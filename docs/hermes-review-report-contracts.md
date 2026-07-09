# Hermes review report contracts

Animus adapts the Hermes review-report pattern as an Animus-owned, pure adapter contract. The parser operates over in-memory markdown only; hosts own file reads, storage, timestamps, and append-only persistence.

## Grammar

A report must contain a `VERDICT: PASS | WARN | FAIL` line. Optional `##` sections are parsed as bounded section metadata with line counts and BLAKE3 body hashes.

Metric-bearing findings are parsed only inside `## Findings` from bullets shaped like:

```md
- [class=lint scope=adapter route=test promoted=no] Short safe summary.
```

The required tags are `class`, `scope`, `route`, and `promoted`. `promoted` is limited to `yes` or `no`. Checklist bullets in other sections remain human-readable prose and do not become metric records.

## Metrics boundary

Parsing, summary, and promotion-candidate analysis are read-only. Durable writes require an explicit host append action using `append_review_metric_record`. The resulting record contains safe metadata, counts, tags, verdict, report hash, finding summary hashes, and a BLAKE3 record hash. It excludes raw prompts, provider payloads, credentials, full host paths, and unbounded tool/model output.

Repeated-finding promotion candidates group unpromoted findings with the same `class`, `scope`, and `route`. The grouping suggests a durable prevention route; it does not mark records promoted, edit tasks, mutate specs, or create lifecycle changes automatically.

## Corrective memory boundary

Corrective records are optional host-supplied DTOs. `validate_corrective_record` accepts safe labels, trust scope, freshness, prompt budget, redaction state, and BLAKE3 body identity only; raw napkin or memory bodies stay in the host store. `admit_corrective_record` requires explicit host policy for trusted source labels, allowed scopes, freshness, and prompt budget before a record can influence prompt assembly. Always-read records, untrusted sources, expired records, secret records, and over-budget records fail closed or are omitted with safe reasons.

`corrective_promotion_candidates` routes repeated review metrics into advisory targets such as deterministic checks, policy guidance, spec rules, memory guidance, or human checkpoints. Append and promote operations use explicit `validate_corrective_append_request` and `validate_corrective_promote_request` DTO validation against a host-owned store status. Safe receipts from `validate_corrective_host_action_receipt` contain labels, status, issue code, and BLAKE3 hashes only.

Non-claims: Animus does not load `.agent/napkin.md`, create global corrective memory, persist records by default, edit Cairn tasks/specs/policy automatically, or make review metrics a scoring authority.

## Done-review lifecycle boundary

Done-review lifecycle contracts are optional SDK DTOs. `validate_review_lifecycle_request` accepts safe review subject metadata, bounded evidence hashes, declarative completion rail descriptors, host-supplied rail result summaries, reviewer attempt descriptors, and an output contract. It does not collect diffs, read VCS state, run shell commands, call reviewers, write metrics, or persist lifecycle state.

`plan_review_lifecycle_dispatch` only decides whether a host may dispatch through an injected reviewer service; missing or unavailable services fail closed without selecting a default reviewer or provider. `reduce_review_lifecycle_outcome` maps parsed `PASS`, `WARN`, `FAIL`, missing-service, and malformed-output states to safe continuation intents. `validate_review_lifecycle_receipt` keeps durable review evidence to request/status labels, issue codes, and BLAKE3 hashes.

Non-claims: Animus does not provide a built-in `done-review` command, require Hermes, choose default reviewer models, execute completion rails, inspect files or VCS, persist metrics, mutate tasks/specs, retry automatically, or approve completion without host policy.

## Host asset reconciliation boundary

Host asset reconciliation contracts are optional SDK DTOs. `validate_host_asset_inventory` accepts host-supplied manifest entries and installed observations for skills, resources, plugins, tool catalogs, policy profiles, prompt profiles, and other optional assets. Inputs are limited to safe asset IDs, store labels, trust/enablement state, observation epochs, provenance labels, and BLAKE3 content identities.

`classify_host_asset_drift` compares validated manifests and observations into store-neutral classes such as installed-matches-manifest, installed-disabled, identity-mismatch, manifest-only, host-only, stale-observation, unsupported-store, duplicate-asset, and untrusted-asset. `plan_host_asset_reconciliation` emits dry-run advisory actions only. `admit_host_asset_mutation` requires a separate host service, matching dry-run action, capability label, and plan identity before a host may mutate anything. `validate_host_asset_receipt` keeps durable evidence to safe status labels, issue codes, and hashes.

Non-claims: Animus does not scan `~/.hermes`, `~/.pi`, home directories, repositories, or global stores; it does not resolve symlinks, copy/link/unlink files, enable/disable assets, install plugins, write manifests, or decide which host store is canonical.

## Multi-lens focus deck boundary

Multi-lens contracts are optional SDK DTOs. `validate_focus_deck` accepts host-defined deck families, focus items, retrieval budget references, evidence expectations, output contracts, and review criteria. `validate_lens_task_packet` admits only bounded context labels and BLAKE3 evidence references for a focus item.

`plan_lens_execution` returns typed task packets for host-owned worker, external-service, or sequential-checklist modes, and missing worker services fail closed for modes that require them. `validate_lens_report` and `validate_lens_synthesis` preserve lens IDs, evidence hashes, assumptions, skipped lenses, contradictions, confidence, and unresolved gaps. Unresolved contradictions cannot be collapsed into a high-confidence final claim.

Non-claims: Animus does not ship Hermes `multi-lens`, spawn subagents, call models, open networks, read repositories, create threads/processes, require property-discovery decks, enforce parallelism, or provide a live analysis service by default.

## Plan/loop mode FSM boundary

Plan/loop mode contracts are optional SDK DTOs. `validate_mode_descriptor` accepts trusted host descriptors with mode identity, posture, capability ceiling labels, explicit plan markers, selected-step or loop output contracts, and bounded loop budgets. These descriptors are policy data, not permission enforcement.

`extract_plan_steps` parses only configured marker-delimited step records such as `- [step-id] label`; ordinary numbered prose is not captured as a plan. `apply_selected_step_update`, `start_loop_work`, and `validate_breakout_acknowledgement` keep selected-step and loop-continuation state typed, epoch-checked, and tied to safe work IDs. `plan_mode_controller` reports missing host controllers without installing commands, shortcuts, tools, timers, or UI behavior. `validate_mode_receipt` keeps durable state to safe labels and BLAKE3 hashes.

Non-claims: Animus does not provide `/mode`, `/plan`, `/loop`, shortcuts, UI pickers, tool allowlists, automatic loop turns, provider probes, delegation strategy, session persistence, or read-only enforcement by prompt text. Hosts must enforce capabilities, sandbox policy, confirmation, and mutation serialization separately.

## Validation

Focused validation:

```sh
nix develop -c cargo test -p animus-adapters --locked review::tests
nix develop -c cargo test -p animus-adapters --locked
nix run .#cairn -- validate --root .
```

Full workspace clippy/flake closeout remains subject to the repo-wide `animus-adapters` clippy debt tracked in Cairn evidence.
