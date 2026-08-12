# Tasks

- [x] [serial] Define plugin/toolset manifest DTOs for toolset name, version, kind, provided tools, descriptions, schema identities, result contracts, capability labels, provenance, and trust status. r[agent_harness_runtime.hermes_plugin_manifests.toolset]
- [x] [depends:toolset] Add positive fixtures based on the Hermes review-gate plugin manifest shape. r[agent_harness_runtime.hermes_plugin_manifests.toolset.valid]
- [x] [serial] Add pure manifest validation for missing descriptions, invalid names, unsupported kinds, missing capability labels, duplicate tools, unsafe provenance, and malformed schema/result identities. r[agent_harness_runtime.hermes_plugin_manifests.validation]
- [x] [depends:validation] Add duplicate-tool and malformed-manifest negative tests. r[agent_harness_runtime.hermes_plugin_manifests.validation.duplicate]
- [x] [serial] Wire manifest-provided tools to host-injected executors only, with typed missing-executor failures. r[agent_harness_runtime.hermes_plugin_manifests.executor_boundary]
- [x] [depends:executor_boundary] Add negative tests proving missing executors do not trigger scripts, shell fallback, module imports, or provider calls. r[agent_harness_runtime.hermes_plugin_manifests.executor_boundary.missing]
- [x] [serial] Require side-effect capability labels and expose them in prompt/tool metadata. r[agent_harness_runtime.hermes_plugin_manifests.capabilities]
- [x] [depends:capabilities] Add tests separating read-only summary tools from metrics-write or mutating tools. r[agent_harness_runtime.hermes_plugin_manifests.capabilities.metrics_write]
- [x] [depends:toolset.valid,validation.duplicate,executor_boundary.missing,capabilities.metrics_write] Update docs and run focused plugin-manifest tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.hermes_plugin_manifests.verification]
