# Tasks

- [x] [serial] Define progressive tool discovery DTOs for list queries, summaries, details, schema identities, capability labels, result contracts, provenance, ceilings, and discovery receipts. r[agent_harness_runtime.goose_code_mode_tool_planning.discovery]
- [x] [depends:discovery] Define declarative tool plan DTOs for batch identity, steps, dependencies, intermediate bindings, batch policy, cancellation, timeout, and outcomes. r[agent_harness_runtime.goose_code_mode_tool_planning.plan_dtos]
- [x] [depends:plan_dtos] Add optional Steel or typed-fabric admission for producing tool plans without arbitrary JS/Deno execution or host code evaluation. r[agent_harness_runtime.goose_code_mode_tool_planning.fabric]
- [x] [depends:plan_dtos] Revalidate each planned step through catalog trust, exact capability labels, permission policy, argument schema, mutation windows, output bounds, cancellation, timeout, and receipt policy. r[agent_harness_runtime.goose_code_mode_tool_planning.step_admission]
- [x] [depends:step_admission] Add intermediate-data validation for bounded text, safe handles, hashes, counts, binary/image/secret classes, and oversized result denial. r[agent_harness_runtime.goose_code_mode_tool_planning.intermediates]
- [x] [depends:fabric,intermediates] Add safe plan receipts and leak checks for raw prompts, scripts, tool bodies, provider payloads, credentials, full paths, and unbounded intermediates. r[agent_harness_runtime.goose_code_mode_tool_planning.receipts]
- [x] [depends:receipts] Update docs and run focused discovery, plan DTO, fabric, step admission, intermediate-data, receipt leak, Cairn validation, and Cairn gate checks with reviewable evidence. r[agent_harness_runtime.goose_code_mode_tool_planning.verification]
