## Why

[Animus #17](https://github.com/OnixResearch/animus/issues/17) asks Animus to consume host-supplied model readiness and request capacity independently of agent-worker placement. Lattice #24 (serving half) and Lattice #13 (per-request model usage) are blocked on it. Animus already selects model metadata and builds bounded routing plans, but no contract binds serving readiness, serving-owner request admission, or per-request usage provenance to a named model route.

## What Changes

- Add the pure `animus_adapters::serving` core: bounded `animus.serving-availability-snapshot.v1` snapshots, the `animus.serving-capacity-profile.v1` profile, eligibility screening before any preference, the authoritative pre-dispatch gate, and the metadata-only `animus.serving-invocation-receipt.v1` receipt. r[agent_harness_runtime.serving_admission.snapshot] r[agent_harness_runtime.serving_admission.eligibility] r[agent_harness_runtime.serving_admission.gate] r[agent_harness_runtime.serving_admission.receipt]
- Add optional version-bound rollout facts and requirements. r[agent_harness_runtime.serving_admission.rollout]
- Add the non-default `serving-admission` feature with one complete host path over the existing Mesh shell, an injected `OwnerPort`, a fail-closed `UnavailableOwner`, a deterministic `FixtureOwner`, and a JSON codec. r[agent_harness_runtime.serving_admission.host_path]
- Add the reviewed Nickel profile `config/serving-capacity-profile.ncl` with positive and negative fixtures.

## Impact

- **Files**: `crates/animus-adapters/src/serving*`, `crates/animus-adapters/src/mesh.rs` (derives), `crates/animus-std/src/serving*`, `crates/animus-std/src/mesh.rs` (`ResponseFacts` on transport outcomes), Cargo features, `src/facades/host.rs`, Nickel config and export scripts, `flake.nix` config diff, `fixtures/serving-admission/`.
- **Testing**: `cargo test -p animus-adapters serving`, `cargo test -p animus-std --no-default-features --features serving-admission`, clippy, `adapters-octet`, Nickel config checks.
- **Compatibility**: `animus_std::mesh::TransportOutcome` and `ProviderObservation` gain a required `facts` field; callers construct `ResponseFacts::UNREPORTED` when nothing is reported.
