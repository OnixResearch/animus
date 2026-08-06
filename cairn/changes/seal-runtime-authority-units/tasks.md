## Phase 1: Baseline and ownership

- [x] [serial] I1 Record current session-authority profile, binding, enforcement, coding-session, safe-receipt, and fake-host baseline results. r[agent_harness_runtime.runtime_authority_seal.verification]
- [x] [serial] I2 Document the dispatch-unit scope and map each input, probe, commit, effect, and cleanup responsibility to its existing Animus owner. r[agent_harness_runtime.runtime_authority_seal.phase] r[agent_harness_runtime.runtime_authority_seal.boundary]

## Phase 2: Pure seal core

- [x] [serial] I3 Add pure typed authority-unit states, transition requests, deterministic issues, and terminal outcomes in `animus-runtime`. r[agent_harness_runtime.runtime_authority_seal.phase]
- [x] [serial] I4 Add canonical validation and domain-separated BLAKE3 identity over the exact profile, binding, enforcement, operation, adapter, resource, and view inputs. r[agent_harness_runtime.runtime_authority_seal.runtime_view]
- [x] [serial] I5 Add pure expected-versus-observed runtime-view comparison with exact missing, extra, duplicate, stale, unknown, and unsupported-class failures. r[agent_harness_runtime.runtime_authority_seal.runtime_view]
- [x] [serial] I6 Add monotonic transition checks that deny post-seal additions, replacement, rebind, discovery, fallback, repeated seal, and stale unit reuse. r[agent_harness_runtime.runtime_authority_seal.monotonic]

## Phase 3: Thin host pilot

- [x] [serial] I7 Add an `animus-std` shell that prepares injected services, obtains current observations, commits the seal receipt, dispatches one fake-backed effect, and reports cleanup. r[agent_harness_runtime.runtime_authority_seal.phase] r[agent_harness_runtime.runtime_authority_seal.receipt]
- [x] [serial] I8 Route an approved authority change through a new unit identity without mutating or widening the prior unit. r[agent_harness_runtime.runtime_authority_seal.monotonic]
- [x] [parallel] I9 Update the application-host binding pilot dependency and integration notes so it consumes the accepted seal instead of defining another seal model. r[agent_harness_runtime.runtime_authority_seal.boundary]

## Phase 4: Verification and documentation

- [x] [parallel] V1 Add positive core and host tests for canonical sealing, exact view match, deterministic identity, commit-before-effect ordering, successful dispatch, and a later new unit. r[agent_harness_runtime.runtime_authority_seal.verification]
- [x] [parallel] V2 Add negative tests for missing, extra, duplicate, stale, unknown, and unsupported observations plus post-seal widening, rebind, replacement, fallback, early effect, commit failure, stale reuse, leaks, and overclaims. r[agent_harness_runtime.runtime_authority_seal.verification] r[agent_harness_runtime.runtime_authority_seal.boundary]
- [x] [parallel] V3 Add property tests for input-order independence, deterministic issue order, and the absence of any transition from `sealed` or `running` to a wider authority set. r[agent_harness_runtime.runtime_authority_seal.monotonic] r[agent_harness_runtime.runtime_authority_seal.verification]
- [x] [serial] D1 Add focused documentation, contract indexes, safe receipt guidance, and the bounded Redox design reference. r[agent_harness_runtime.runtime_authority_seal.receipt] r[agent_harness_runtime.runtime_authority_seal.boundary]
- [ ] [serial] V4 Add and run `scripts/cairn/check-runtime-authority-seal --root .`, focused Cargo tests, purity checks, Nickel freshness, workspace checks, Clippy, Cairn validation, gates, and relevant Nix checks. r[agent_harness_runtime.runtime_authority_seal.verification]
