# Tasks

## B. Baseline and ownership

- [x] [serial] B1: Record DeepSeek Harness revision `47f943859bef60e4160492346772ded9b24f765a`, MIT license, developer-preview status, selected invariant, rejected runtime concepts, and overlap with existing Animus owners. r[agent_harness_runtime.model_request_snapshot.projection]
- [x] [serial] B2: Run and record focused model-profile, request-context, coding-session, active-turn-recovery, provider-retry, authority-seal, durable-store, and evidence-safety baselines before core changes. r[agent_harness_runtime.model_request_snapshot.verification]

## C. Pure snapshot contract

- [x] [depends:B1] C1: Define provider-neutral snapshot drafts, validated snapshots, canonical field ordering, versioned domain tags, BLAKE3 identities, and typed issue classes in `animus-session-core`. r[agent_harness_runtime.model_request_snapshot.projection]
- [x] [depends:C1] C2: Bind ordered messages, system sections, selected contributions, tool schemas, output controls, budgets, model route, invocation plan, authority seal, and adapter cohort without credentials or transport fields. r[agent_harness_runtime.model_request_snapshot.projection]
- [x] [depends:C2] C3: Add a private durable session-entry contract and public redaction rules for request snapshots. r[agent_harness_runtime.model_request_snapshot.durability] r[agent_harness_runtime.model_request_snapshot.evidence]

## H. Host ordering and recovery

- [x] [depends:C3] H1: Extend the coding-session reducer and shell so exact snapshot commit completes before provider dispatch. r[agent_harness_runtime.model_request_snapshot.durability]
- [x] [depends:H1] H2: Bind initial dispatch and same-request retry to the committed snapshot identity, active leaf, request sequence, adapter cohort, and current authority facts. r[agent_harness_runtime.model_request_snapshot.dispatch]
- [x] [depends:H2] H3: Add explicit committed-unobserved recovery and route any retry decision through the existing provider-retry owner. r[agent_harness_runtime.model_request_snapshot.recovery]
- [x] [depends:H3] H4: Integrate the gate with the live Steel shell prerequisites without adding provider-native persistence or fallback. r[agent_harness_runtime.model_request_snapshot.dispatch]

## V. Verification and closeout

- [x] [depends:H3] V1: Add positive fixtures for exact reconstruction, durable commit, initial dispatch, same-request retry, terminal response, and restart classification. r[agent_harness_runtime.model_request_snapshot.verification]
- [x] [depends:H3] V2: Add negative fixtures for missing sections, order drift, stale context, changed tool schema, profile drift, authority drift, adapter substitution, partial commit, body leaks, unsupported fields, and automatic uncertain replay. r[agent_harness_runtime.model_request_snapshot.verification]
- [x] [depends:V2] V3: Add a focused rail and Nix check. Update request lifecycle, session, provider-retry, authority-seal, component catalog, composition, safety, and README reference documentation. r[agent_harness_runtime.model_request_snapshot.evidence] r[agent_harness_runtime.model_request_snapshot.verification]
- [x] [depends:V3] V4: Run focused tests before and after core changes, no-std and Wasm checks, formatting, Clippy, Nickel drift checks, Cairn validation and gates, and the relevant Nix checks. Record exact blockers and non-claims. r[agent_harness_runtime.model_request_snapshot.verification]
