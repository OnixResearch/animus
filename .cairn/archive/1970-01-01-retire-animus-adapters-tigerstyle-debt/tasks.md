## Phase 1: Canonical module structure

- [x] [serial] Move affected implementations behind neutral canonical modules while preserving r[agent_harness_runtime.adapters_tigerstyle.compatibility].
- [x] [depends:agent_harness_runtime.adapters_tigerstyle.zero_findings] Remove path-repetition, filename, and import findings without broad lint suppression.

## Phase 2: Local source quality

- [x] [depends:agent_harness_runtime.adapters_tigerstyle.zero_findings] Repair assertion, condition, naming, arithmetic, parameter, unwrap, and function-size findings.
- [x] [depends:agent_harness_runtime.adapters_tigerstyle.behavior] Prove existing positive and negative adapter behavior remains unchanged.

## Phase 3: Verification and lifecycle

- [x] [depends:agent_harness_runtime.adapters_tigerstyle.verification] Add and pass the zero-finding Octet regression rail plus Clippy, no-default, and Wasm checks.
- [x] [depends:agent_harness_runtime.adapters_tigerstyle.verification] Sync requirements, archive the change, and integrate the verified branch.
