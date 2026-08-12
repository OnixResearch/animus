# Transactional daemon handoff evidence

## Baseline

The pre-change baseline passed on `f1f8369272b5f55bd2e8ea7b96d66e37352d82fb`:

- `scripts/cairn/check-daemon-workers --root .`
- `scripts/cairn/check-host-control-extraction --root .`
- `scripts/cairn/check-agent-control --root .`
- `cargo test -p animus-runtime session_persistence --locked`
- `scripts/cairn/check-active-turn-recovery --root .`
- `scripts/cairn/check-pending-input-vault --root .`

## Boundary inventory

- `animus-session-core` owns pure agent control, active-turn recovery, authority profiles, repair, and session-state reducers.
- `animus-runtime` owns coding-session, durable-session, pending-input, and host-injected session-persistence composition.
- `animus-adapter-daemon` owns no-std daemon role DTOs. It has no process, socket, clock, lease, or persistence implementation.
- `animus-std` owns feature-gated host effects and explicit injected services.
- Host Control owns transport-neutral status and wait semantics. It does not own Animus session leases.
- Tile retains PTYs, workspace state, local socket policy, and rendering. This change does not edit Tile.

Strict idle checks provider, tool, repair, confirmation, cancellation, primary queue, child queue, steering, durable commit, and terminal-transition state. The handoff binding also retains exact worker, session, branch, lease, queue, active-turn, and authority generations.

## Reference boundary

- Reference: Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871`.
- License: ISC.
- Selected behavior: pause before release, one generation-bound claimant, readiness before old-worker exit, and cancellation before owned-tree teardown.
- Rejected behavior: Bash supervision, Lisp process model, ambient socket discovery, and runtime dependency.

## Bounded architecture search

Goal: add one validated transactional replacement handoff without moving effects into reusable crates.

Completion evidence: deterministic positive and negative production-path tests, strict Clippy, no-std and Wasm checks, metadata-only receipts, focused Cairn/Nix rails, accepted requirement sync, and archive receipts.

False completion: process spawn alone, same-user socket access, a Boolean takeover flag, task checkboxes, or smoke success without lease, claim, readiness, cancellation, and teardown evidence.

Budget: three serial architecture lenses, one adversarial audit, repository-local sources only, no subagents, and the existing deterministic toolchain.

| Family | Mechanism | Result | Risk or blocker | State |
| --- | --- | --- | --- | --- |
| runtime-owned shell | Put transaction and effects in `animus-runtime` | Small public surface | Violates the reusable-core authority boundary | falsified |
| adapter-owned transaction | Put state reduction and ports in `animus-adapter-daemon` | One no-std owner | Mixes product transaction semantics with implementation-free DTOs | falsified |
| split core and shell | Pure reducers in `animus-runtime`, DTO ports in `animus-adapter-daemon`, effects in feature-gated `animus-std` | Preserves owners and supports direct negative testing | Requires exact conversion checks at each boundary | active |

The selected split route uses a functional core and thin imperative shell. The adversarial audit targets skipped transitions, stale generations, duplicate and late claims, wrong endpoints, readiness drift, spawn failure, surviving descendants, rollback drift, receipt leaks, and accidental default-feature authority.

## Implemented boundary

- `animus_runtime::daemon_handoff` exposes pure strict-idle validation, monotonic transaction reduction, exact claim admission, readiness, cancellation, complete-tree teardown, next-generation restoration, Host Control status projection, and metadata-only receipt validation.
- `animus-adapter-daemon` exposes versioned closed DTOs plus implementation-free ingress, state, lease, process, endpoint, readiness, and exit ports.
- `animus_std::daemon_handoff_host` is non-default. It calls one injected `HandoffHostServices` implementation and preserves commit-before-release, readiness-before-exit, cancellation-before-termination, complete-tree-before-restore, and ingress-resumes-last ordering.
- `animus::daemon_handoff` is the feature-gated root facade. Default features do not enable handoff effects.
- The fake-backed packaged smoke emits bounded transaction metadata and required non-claims only.

## Adversarial evidence

- Eight runtime tests cover exact success, all strict-idle work classes, skipped and replayed transitions, tampering, stale process/generation/endpoint/protocol claims, stale authentication, duplicate and late claimants, complete-tree restoration, readiness drift, rollback drift, receipt leaks, and overclaims.
- Two adapter tests cover valid claims and termination, unsupported schema/protocol, unsafe and oversized labels, and inconsistent process-tree counts.
- Six host tests cover exact effect order, readiness timeout, spawn failure without invented ownership, wrong endpoint, surviving descendants, rollback drift, second-owner denial, and blocked ingress resume.
- The non-default Tiger Style feature probe reported no findings under `crates/animus-std/src/daemonhandoff/`. Its full command also exposed unrelated existing findings in prerequisite feature modules, so it is not a whole-feature pass claim.

## Validation

Passing checks:

- `scripts/cairn/check-transactional-daemon-handoff --root .`
- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets --locked`
- `cargo test --workspace --locked`
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- strict feature Clippy for `animus-runtime`, `animus-adapter-daemon`, `animus-std`, and `animus`
- runtime and adapter Wasm checks plus no-default std and root-facade checks
- `cargo tigerstyle check`
- component catalog, adapter conformance, runtime facade, request lifecycle, dogfood readiness, evidence safety, roadmap, daemon-worker, Host Control, agent-control, active-turn, pending-input, coding-session, and durable-store rails
- `nix run .#cairn -- validate --root .`
- Nix checks `transactional-daemon-handoff`, `adapter-conformance-index`, `component-catalog`, and `runtime-facade-inventory`

The local Nix builder completed each derivation after the configured remote builder connection failed. This does not weaken the derivation result.

## Non-claims

This evidence does not establish PTY migration, process-memory continuity, provider-session transfer, sandboxing, remote security, exactly-once effects, power-loss durability, production readiness, or release eligibility.

## Lifecycle closeout

- Sync dry-run was unblocked with receipt `cffe3f44397a01b01eda987dae4a807968036ce2dff3506718581c2a7731a3a7`.
- Sync execution applied all seven requirements with receipt `27579c7025d00101433ea42c46a5af0a8a2f47a0cfb6687bed92854da7ebd67f`.
- Accepted-spec after-hash: `4ab13d8ba3ff1cbee4a2c202f1ecee2ee1342e458503f9b02064312ab7af1150`.
- Sync mutation manifest: `0d1559de43d3041118d59277142f6d880e6e6fdd8c14fc3b21e26b0764539e26`.
- Scoped traceability audit found no missing or dangling `agent_harness_runtime.daemon_handoff` requirement.
- Global Tracey receipt `f6ad060ae88e6206d23ab0b4b0705ae0545fdc47322bdfd298ee5e3ad484caa8` remains invalid because of unrelated accepted-spec backlog. It reports 987 referenced markers across 3,333 requirements.
- Archive dry-run was unblocked with receipt `9db56e33e7b7caf6fbfe7df2787c624864f42b747f0a5d31d1334e78bb765c5a` and plan `3d299602aeef8ccb3616047d327f1913ed92d4d34766b7957929d5e83d57cd7f`.
- Archive execution completed with receipt `6527a4e8bfa32de1c0b5838ec4b45a066d88b2d0a263f50e329728e1a4823528`.
- Archive execution plan: `3214f9268f98ac476bb7a3f109e37996227fdf03f4cd51161f67425e26dee684`.
- Archive mutation manifest: `de17df7f7e62e4f6e5ad34aaaf3ffe0c1f9156ffd7cfd436cc36d48fa1fd56a1`.
