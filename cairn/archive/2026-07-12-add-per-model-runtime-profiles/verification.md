# Verification evidence

## Baseline

- Before implementation, focused existing prompt-assembly, skill-manifest, model-capability/dispatch, router-execution, and tool-executor-admission tests passed.
- Before implementation, `nix run path:$PWD#cairn -- validate --root .` reported `valid: true` for seven active changes with no change or spec issues.
- The baseline established that the existing contracts were healthy; it did not establish per-model composition or fallback isolation, which did not yet exist.

## Implemented boundary

- `config/model-runtime-profiles.ncl` owns the typed authoring contract and deterministic generated JSON. Eleven positive/negative fixtures cover exact bindings, explicit defaults, reference integrity, memory and tool modes, budget references, duplicate identities, project admission, and schema drift.
- `animus-runtime::modelprofiles` is a pure deterministic core for validation, BLAKE3 identity, exact-tier resolution, prompt/skill/memory/tool composition, dispatch and transition binding, returned tool-call binding, and metadata-only receipt projection.
- Profiles are independent ceilings, never grants. Composition requires an exact existing validated `OptInMemoryPolicy`, existing model capability decisions, every authority ceiling, trusted prompt/skill/tool inputs, and ready executor bindings.
- Cross-target fallback requires fresh resolution and composition. Same-target retry, resume, and replay require every target, profile, prompt, skill, memory, capability, tool-catalog, policy, and plan identity to remain exact.
- `animus-std` owns the feature-gated explicit-path generated-JSON loader. Reusable crates neither read profile files nor evaluate Nickel.

## Focused evidence

- `scripts/cairn/check-model-runtime-profiles --root .`: pass; deterministic Nickel export, one positive and ten negative Nickel fixtures, runtime and std tests, pure-core ambient-authority scan, and focused receipt generation all passed.
- `cargo test -p animus-runtime --locked modelprofiles`: pass, 18 tests including positive and negative contract, resolution, ceiling, prompt, skill, memory, tool, transition, tool-call, and receipt cases.
- `cargo test -p animus-std --locked --features model-runtime-profiles modelprofiles`: pass, 3 tests including generated input, malformed/unknown/stale input, and missing explicit-path failure without discovery.
- `scripts/config/check-nickel-configs`: pass.
- `nix build path:$PWD#checks.x86_64-linux.model-runtime-profiles --no-link -L`: pass.
- `scripts/cairn/check-composition-conformance --root .`: pass.
- `scripts/cairn/check-adapter-conformance-index --root .`: pass.
- `scripts/cairn/check-component-catalog --root .`: pass.
- `scripts/cairn/check-runtime-facade-inventory --root .`: pass.
- `scripts/cairn/check-safe-receipts-artifacts --root .`: pass.
- `scripts/cairn/check-tool-catalog --root .`: pass.
- `scripts/cairn/check-router-policy --root .`: pass.
- `scripts/cairn/check-dogfood-readiness --root .`: pass.
- `scripts/cairn/check-roadmap --root .`: pass.
- `scripts/cairn/check-wasm-surface --root .`: pass.

## Widened evidence

- `cargo test --workspace --locked`: pass.
- `cargo check --workspace --all-targets --locked`: pass.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: pass.
- `cargo clippy -p animus-std --all-targets --features model-runtime-profiles --locked -- -D warnings`: pass.
- `cargo clippy -p animus --all-targets --features model-runtime-profiles --locked -- -D warnings`: pass.
- `cargo tigerstyle check`: pass after replacing unbounded diagnostic builders and ambiguous same-type interfaces with bounded pure returns and explicit typed inputs, and after adding success-path invariant assertions.
- `pre-commit run --all-files`: pass for generated Nickel configs, router conformance, and Octet lints.
- `nix flake check path:$PWD -L`: pass for the supported `x86_64-linux` checks; Nix reported only the expected incompatible-system omission for `aarch64-linux`.
- `nix run path:$PWD#cairn -- validate --root .`: pass with six active changes, nine accepted specs, and no issues.
- Proposal, design, and tasks gates for `add-per-model-runtime-profiles`: pass.

## Adversarial review reconciliation

VibeThinker was asked for an adversarial authority and isolation audit. Three bounded coverage suggestions were accepted: same-target retry now directly rejects a changed memory-plan identity, returned tool calls directly reject a changed tool-catalog identity, and the std shell directly proves a missing explicit profile path fails without discovery. The suggested cross-target authority case is already covered by requiring `RecomposeForTarget`, composing a materially distinct second target with no inherited skill, memory, or tools, and rejecting stale target/profile bindings before executor admission. A provider retry-loop limit, provider effects, heap-allocation policy, and automatic feature-driven discovery were rejected as gaps because this change owns no provider loop, executor effects, allocator policy, or automatic loading; claiming those would violate its explicit boundary.

## Claim boundary

This evidence establishes checked authoring shape, exact host-loaded revalidation, pure deterministic composition, identity-bound transition decisions, canonical declaration filtering, and metadata-only receipt construction for the supplied in-memory inputs. It does not prove provider truth, credential handling, model behavior, successful provider or tool effects, filesystem/process/network confinement, external verifier soundness, release eligibility, or whole-system correctness. Hosts remain responsible for explicit loading, current authority/grant inputs, provider dispatch, tool executor admission and effects, persistence, clocks, credentials, and durable receipt storage.

## Lifecycle evidence

- `cairn sync add-per-model-runtime-profiles --root .` dry-run: unblocked; `--execute`: completed.
- The `agent_harness_runtime.model_runtime_profiles.*` requirements are present in `cairn/specs/agent-harness-runtime/spec.md` after sync.
- `cairn archive add-per-model-runtime-profiles --root .` dry-run: unblocked; `CAIRN_ARCHIVE_DATE=2026-07-12 ... --execute`: completed.
- Post-archive Cairn validation reports `valid: true` with five active changes and no change or spec issues.
