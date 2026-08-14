# Tasks

## B. Baseline and ownership

- [ ] [serial] B1: Record DeepSeek Harness revision `47f943859bef60e4160492346772ded9b24f765a`, MIT license, developer-preview status, selected staged-interception semantics, and rejected dynamic plugin behavior. r[agent_harness_runtime.tool_execution_stages.plan]
- [ ] [serial] B2: Run and record tool-executor-kit, tool-catalog, coding-session, lifecycle-hook, authority-seal, composition-conformance, and evidence-safety baselines before core changes. r[agent_harness_runtime.tool_execution_stages.verification]

## C. Pure stage contract

- [ ] [depends:B1] C1: Define fixed phases, handler descriptors, immutable stage plans, BLAKE3 plan identities, candidate outcomes, transition issues, and metadata-only receipts. r[agent_harness_runtime.tool_execution_stages.plan]
- [ ] [depends:C1] C2: Bind every stage to the exact tool request, schema, executor, authority seal, mutation posture, output policy, cancellation, deadline, and handler cohort. r[agent_harness_runtime.tool_execution_stages.binding]
- [ ] [depends:C2] C3: Implement monotonic candidate validation that preserves sticky denial and rejects capability, authority, mutation, bound, status, identity, and commit-barrier widening. r[agent_harness_runtime.tool_execution_stages.monotonic]
- [ ] [depends:C3] C4: Implement bounded output-narrowing validation that preserves the original outcome class and binds input and narrowed-output identities. r[agent_harness_runtime.tool_execution_stages.output]

## H. Shell and session integration

- [ ] [depends:C3] H1: Define phase-specific injected handler ports and an ordered shell around the existing authoritative `tool_executor_kit`. r[agent_harness_runtime.tool_execution_stages.binding]
- [ ] [depends:H1] H2: Keep `plan_tool_execution` as final admission and final output validation inside the existing executor owner. r[agent_harness_runtime.tool_execution_stages.monotonic] r[agent_harness_runtime.tool_execution_stages.output]
- [ ] [depends:H2] H3: Bind the final tool result to coding-session durable commit before post-commit observation and provider continuation. r[agent_harness_runtime.tool_execution_stages.commit]
- [ ] [depends:H3] H4: Integrate the staged boundary with `compose-rlm-host-effect-executor` and the live Steel shell without creating built-in handlers or plugins. r[agent_harness_runtime.tool_execution_stages.commit]

## V. Verification and closeout

- [ ] [depends:H3] V1: Add positive fixtures for zero-handler execution, ordered narrowing handlers, declared wrappers, successful execution, durable commit, post-commit observation, and provider continuation. r[agent_harness_runtime.tool_execution_stages.verification]
- [ ] [depends:H3] V2: Add negative fixtures for handler substitution, phase reorder, sticky-denial reversal, capability widening, argument rewrite, mutation widening, bound increase, failure-to-success conversion, unsafe output, missing commit, pre-commit observation, and receipt leaks. r[agent_harness_runtime.tool_execution_stages.verification]
- [ ] [depends:V2] V3: Add a focused rail and update tool-executor, lifecycle, coding-session, RLM, component-catalog, activation-graph, composition, safety, and README reference documentation. r[agent_harness_runtime.tool_execution_stages.verification]
- [ ] [depends:V3] V4: Run focused tests before and after core changes, no-std and Wasm checks, formatting, Clippy, tool and composition rails, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.tool_execution_stages.verification]
