# Tasks

- [x] [depends:add-tui-conversation-render-model] Add a session picker that projects session-tree-ledger DTOs and issues typed switch commands without mutating sessions directly. r[agent_harness_runtime.tui_status.session_picker]
- [x] [depends:add-tui-conversation-render-model] Add a model picker that projects provider-registry/model-dispatch DTOs with labels and capabilities only, no credential leakage. r[agent_harness_runtime.tui_status.model_picker]
- [x] [depends:add-tui-conversation-render-model] Add a context/retrieval-budget gauge bounded and normalized from retrieval-budget DTOs. r[agent_harness_runtime.tui_status.context_gauge]
- [x] [depends:add-tui-conversation-render-model] Add a status bar and cost overlay following status-authority precedence and cost-model-routing DTOs, display-only with freshness metadata. r[agent_harness_runtime.tui_status.status_bar_cost]
- [x] [depends:add-tui-conversation-render-model] Add read-only git/branch, environment, and todo panels projecting process-job-profile and status surfaces, issuing typed commands for any mutation. r[agent_harness_runtime.tui_status.aux_panels]
- [x] [depends:session_picker,model_picker,context_gauge,status_bar_cost,aux_panels] Add negative tests for missing sessions, credential leakage, oversize context, unknown models, and reusable-crate contract leakage, plus positive panel tests. r[agent_harness_runtime.tui_status.verification]
