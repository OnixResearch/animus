# Tasks

- [x] [serial] Add a short architecture note documenting which Clash safety-harness patterns are in scope and which Clash subsystems are explicitly out of scope. r[agent_harness_runtime.clash_ucan.scope]
- [x] [depends:scope] Define typed capability request DTOs for guarded runtime actions using contract, resource, and ability fields. r[agent_harness_runtime.clash_ucan.capability_request]
- [x] [depends:capability_request] Add Nickel-authored policy contracts or fixtures mapping model, tool, filesystem, exec, network, and Steel actions to stable resource/ability names. r[agent_harness_runtime.clash_ucan.nickel_contracts]
- [x] [depends:nickel_contracts] Enforce contract/resource/ability authorization with UCAN grants before runtime dispatch treats a local allow as executable. r[agent_harness_runtime.clash_ucan.ucan_authority]
- [x] [depends:ucan_authority] Model user, project, session, and harness policy layers as local caveats that can narrow, deny, or ask, and require a verified UCAN grant for any session expansion. r[agent_harness_runtime.clash_ucan.local_caveats]
- [x] [depends:ucan_authority] Attach sandbox descriptors only as execution constraints for authorized actions, and fail closed when a required descriptor is malformed or unenforceable. r[agent_harness_runtime.clash_ucan.sandbox_constraints]
- [x] [depends:local_caveats,sandbox_constraints] Emit safe authorization receipts with BLAKE3 hashes, checked-capability evidence, caveat results, sandbox descriptor hashes, and explicit redaction markers. r[agent_harness_runtime.clash_ucan.receipts]
- [x] [depends:receipts] Add positive tests for authorized model/tool actions, local caveat narrowing, session grant authorization, sandbox-constrained dispatch, and redacted receipts. r[agent_harness_runtime.clash_ucan.verification]
- [x] [depends:receipts] Add negative tests for missing UCAN grants, contract/resource/ability mismatches, local policy denials, malformed sandbox descriptors, unsupported sandbox enforcement, and receipt leak attempts. r[agent_harness_runtime.clash_ucan.verification]
- [x] [depends:verification] Run focused capability/caveat/sandbox/receipt tests, workspace checks, Cairn validation, and Cairn proposal/design/tasks gates with reviewable evidence before archive. r[agent_harness_runtime.clash_ucan.closeout]
