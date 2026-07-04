# Tasks

- [ ] [serial] Define initial domain prelude modules for loop runtime, tool contracts, headless protocol, provenance, worker-team delegation, policy/middleware, and status selection. r[agent_harness_runtime.composability.domain_preludes]
- [ ] [depends:domain_preludes] Add positive import/compile tests proving each prelude can construct representative DTOs and call pure helpers without unrelated imports. r[agent_harness_runtime.composability.domain_preludes.compile]
- [ ] [serial] Document prelude selection guidance and keep the flat root exports documented as compatibility surfaces. r[agent_harness_runtime.composability.compatibility_facade]
- [ ] [depends:compatibility_facade] Add negative doc/facade checks preventing compatibility exports from being removed or silently renamed while preludes are introduced. r[agent_harness_runtime.composability.compatibility_facade.stable]
- [ ] [serial] Classify each prelude in the runtime facade inventory with owning crate, authority class, side-effect class, and feature requirements. r[agent_harness_runtime.composability.prelude_inventory]
- [ ] [depends:prelude_inventory] Extend inventory checks with negative fixtures for shell, provider, filesystem, process, and terminal imports in reusable preludes. r[agent_harness_runtime.composability.prelude_inventory.no_leak]
- [ ] [depends:domain_preludes.compile,compatibility_facade.stable,prelude_inventory.no_leak] Update README/docs and run focused prelude compile checks, facade inventory rail, workspace checks, and Cairn gates with evidence. r[agent_harness_runtime.composability.domain_preludes.verification]
