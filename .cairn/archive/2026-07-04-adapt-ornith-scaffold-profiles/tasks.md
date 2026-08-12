# Tasks

- [x] [serial] Define `ScaffoldProfile` and `ScaffoldRevision` DTOs with ID, revision ID, task class, strategy knobs, provenance, review status, route constraints, and BLAKE3 content hash. r[agent_harness_runtime.ornith_scaffold.profile_contract]
- [x] [depends:profile_contract] Add positive profile validation tests and negative malformed ID, duplicate revision, unsupported knob, missing provenance, unreviewed active revision, and non-matching hash tests. r[agent_harness_runtime.ornith_scaffold.profile_contract.invalid]
- [x] [serial] Add pure scaffold-selection planning over in-memory profiles, task class, host policy, and route capability labels. r[agent_harness_runtime.ornith_scaffold.selection]
- [x] [depends:selection] Add selection tests for exact task-class match, fallback profile, disabled profile, route mismatch, and no-compatible-profile failure. r[agent_harness_runtime.ornith_scaffold.selection.policy]
- [x] [serial] Define proposed scaffold revision admission requests and admitted, denied, unavailable, and stale-base outcomes. r[agent_harness_runtime.ornith_scaffold.admission]
- [x] [depends:admission] Add negative tests proving unadmitted or denied revisions do not persist, become active, or influence future prompt/Steel inputs. r[agent_harness_runtime.ornith_scaffold.admission.unreviewed]
- [x] [serial] Define scaffold influence receipt metadata with profile IDs, revision IDs, task class, review status, route labels, validation outcome, and BLAKE3 hashes. r[agent_harness_runtime.ornith_scaffold.receipts]
- [x] [depends:receipts] Add receipt leak-rejection tests for raw prompts, scaffold bodies, provider payloads, credentials, full paths, and unbounded tool output. r[agent_harness_runtime.ornith_scaffold.receipts.redacted]
- [x] [serial] Document scaffold profiles as route-neutral reviewed strategy data and explicitly reject executable model-authored harnesses. r[agent_harness_runtime.ornith_scaffold.scope]
- [x] [depends:profile_contract.invalid,selection.policy,admission.unreviewed,receipts.redacted,scope] Run focused scaffold tests, workspace checks, Cairn validation, roadmap validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.ornith_scaffold.verification]
