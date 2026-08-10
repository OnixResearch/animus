# agent-harness-runtime Change Spec: Extract the session core

## ADDED Requirements

### Requirement: Dedicated session core crate

r[agent_harness_runtime.session_core_crate] Animus MUST own pure session state and evidence contracts in an internal `animus-session-core` crate without transferring host, repository, policy, evidence, license, or release authority.

#### Scenario: Session decisions remain pure

r[agent_harness_runtime.session_core_crate.pure]
- GIVEN admitted session authority, replay, projection, branch, recovery, repair, or redacted export inputs
- WHEN `animus-session-core` computes a state transition, plan, projection, identity, diagnostic, or receipt
- THEN it uses no provider, filesystem, process, network, credential, clock, terminal, concrete persistence, or Steel authority
- AND its supported reusable profile builds with `no_std` plus `alloc`.

#### Scenario: Existing runtime paths remain compatible

r[agent_harness_runtime.session_core_crate.compatibility]
- GIVEN a consumer uses an existing `animus_runtime` session module, root export, feature name, schema, or BLAKE3 identity
- WHEN the session family moves to `animus-session-core`
- THEN explicit compatibility re-exports preserve the existing source path and behavior
- AND the move does not change stable serialized forms, receipt schemas, or identity domains.

#### Scenario: Forbidden dependency fails closed

r[agent_harness_runtime.session_core_crate.dependencies]
- GIVEN a candidate `animus-session-core` manifest imports Steel, a provider client, filesystem API, process API, terminal crate, concrete persistence, or host clock
- WHEN the repository dependency-boundary check runs
- THEN it rejects the manifest and identifies the forbidden dependency
- AND malformed manifests fail without partial admission.

#### Scenario: Checked catalogs identify both owners

r[agent_harness_runtime.session_core_crate.catalog]
- GIVEN the implementation moves while compatibility paths remain in `animus-runtime`
- WHEN the component catalog and runtime facade inventory are checked
- THEN they identify `animus-session-core` as the implementation owner
- AND they retain `animus-runtime` as the compatibility facade owner.

#### Scenario: Extraction validation is bounded and complete

r[agent_harness_runtime.session_core_crate.validation]
- GIVEN the extracted crate and compatibility facade
- WHEN repository validation runs
- THEN positive and negative dependency, direct-crate, facade, no-std, Wasm, Cargo, Clippy, Octet, Cairn, catalog, and Nix checks run
- AND closeout evidence records exact source and receipt identities plus any inherited blocker or non-claim.
