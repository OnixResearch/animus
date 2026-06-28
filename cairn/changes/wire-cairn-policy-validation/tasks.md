# Tasks

- [ ] [serial] Add repo-local generated Cairn policy JSON in the path expected by default Cairn validation. r[agent_harness_runtime.cairn_policy.generated_artifact]
- [ ] [depends:generated_artifact] Document the policy source, refresh command, check command, and default validation command. r[agent_harness_runtime.cairn_policy.refresh_docs]
- [ ] [depends:generated_artifact] Add a deterministic policy drift check or documented fixture that compares checked-in generated policy with canonical exported policy when the source is available. r[agent_harness_runtime.cairn_policy.drift_check]
- [ ] [depends:generated_artifact] Ensure default Cairn validation succeeds without passing an absolute external policy path. r[agent_harness_runtime.cairn_policy.default_validation]
- [ ] [depends:drift_check,default_validation] Add release/lifecycle evidence notes with policy source identity, policy hash, and validation command output. r[agent_harness_runtime.cairn_policy.evidence]
- [ ] [depends:refresh_docs,drift_check,default_validation,evidence] Run default and explicit Cairn validation plus proposal/design/tasks gates with reviewable evidence. r[agent_harness_runtime.cairn_policy.closeout]
