## Phase 1: Executor contract

- [x] [serial] Bind pure execution plans to exact request values and reject substituted requests before service effects. r[agent_harness_runtime.read_only_codebase_tool.request_binding]
- [x] [depends:request_binding] Return bounded model-visible output and metadata-only receipt as one executor outcome. r[agent_harness_runtime.read_only_codebase_tool.outcome]

## Phase 2: Concrete host

- [x] [depends:outcome] Add the non-default read-only codebase-query tool definition, pure argument decoder/renderer, and thin graph executor. r[agent_harness_runtime.read_only_codebase_tool.host]
- [x] [depends:host] Add positive and negative capability-root, stale-index, request-substitution, bound, and leak tests. r[agent_harness_runtime.read_only_codebase_tool.verification]
- [x] [depends:verification] Add the packaged deterministic smoke, root facade, metadata, docs, and focused Rust/Nix rail. r[agent_harness_runtime.read_only_codebase_tool.smoke]
- [x] [depends:smoke] Run focused and workspace checks, sync the accepted spec, archive the change, and record bounded evidence. r[agent_harness_runtime.read_only_codebase_tool.verification]
