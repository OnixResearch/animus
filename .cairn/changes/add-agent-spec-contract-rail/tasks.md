## Tasks

- [ ] [serial] Define the spec-contract rail stages that run after a behavioral-model edit and before action handoff. r[animus.spec_contract.rail]
- [ ] [serial] Define the pinned checker adapter that implements the bounded checking contract with exact identity and version admission. r[animus.spec_contract.adapter]
- [ ] [serial] Implement pure receipt admission and model-hash matching so a receipt for a different model hash is stale. r[animus.spec_contract.receipt]
- [ ] [parallel] Implement fail-closed handoff denial for missing, stale, mismatched, and unsupported-checker outcomes, with deterministic denial receipts. r[animus.spec_contract.fail_closed]
- [ ] [parallel] Add positive fixtures for receipted pass-and-handoff and negative fixtures for stale, missing, mismatched, and unsupported-checker cases. r[animus.spec_contract.fixtures]
- [ ] [serial] Run focused core tests, clippy with warnings denied, lifecycle validation, Cairn gates, and relevant flake checks. r[animus.spec_contract.validation]
