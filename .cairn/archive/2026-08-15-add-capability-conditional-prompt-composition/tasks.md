## Phase 1: Composition contract

- [x] [serial] Add explicit resolved availability as a validated composition input. r[agent_harness_runtime.capability_conditional_prompt.availability]
- [x] [serial] Add conditional fragment rendering that drops fragments naming unavailable tools or skills. r[agent_harness_runtime.capability_conditional_prompt.conditional]

## Phase 2: Fail-closed validation and receipts

- [x] [serial] Add the mandatory missing-capability negative fixture and typed diagnostics for unresolvable capability inputs and dangling references. r[agent_harness_runtime.capability_conditional_prompt.missing] r[agent_harness_runtime.capability_conditional_prompt.fail_closed]
- [x] [depends:availability] Add metadata-only composition receipts that record included fragments, omitted fragments, and the availability-set identity. r[agent_harness_runtime.capability_conditional_prompt.receipts]

## Phase 3: Evidence

- [x] [parallel] Add positive fixtures for available tools and skills rendering their fragments and for omission receipts. r[agent_harness_runtime.capability_conditional_prompt.verification]
- [x] [parallel] Add negative fixtures for a role naming a missing tool, unresolvable capability input, dangling reference, and receipt leak rejection. r[agent_harness_runtime.capability_conditional_prompt.fail_closed] r[agent_harness_runtime.capability_conditional_prompt.verification]
- [x] [serial] Verify default builds remain independent of the composition invariant. r[agent_harness_runtime.capability_conditional_prompt.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document the availability inputs, conditional rendering rule, mandatory negative fixture, and non-claims. r[agent_harness_runtime.capability_conditional_prompt.conditional]
- [x] [serial] Run focused profile, role, workspace, formatting, clippy, Cairn validation, and Cairn gate rails, then sync and archive only after all deterministic gates pass. r[agent_harness_runtime.capability_conditional_prompt.verification]
