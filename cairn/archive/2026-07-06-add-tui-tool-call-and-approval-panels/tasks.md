# Tasks

- [x] [depends:add-tui-conversation-render-model] Add tool-call cards derived from `ToolHostEvent` started/progress/output/result/failure events with collapsible bounded output and no raw tool payloads. r[agent_harness_runtime.tui_tools.cards]
- [x] [depends:cards] Add file-edit and diff views rendered from hashed-edit-protocol DTOs and tool-gateway-delivery receipts, host-only, with no direct file reads. r[agent_harness_runtime.tui_tools.diff_view]
- [x] [depends:cards] Add progress rendering for long-running tools from existing tool host progress events. r[agent_harness_runtime.tui_tools.progress]
- [x] [depends:cards] Add interactive approval prompts that extend the archived `ToolHostEvent`/`ToolHostCommand` bridge with finite `ToolCommandChoice` and `ToolCorrelationId`, adding no new approval contract. r[agent_harness_runtime.tui_tools.approval]
- [x] [depends:cards,diff_view,progress,approval] Add negative tests for denied approvals, mismatched correlation, oversized output, malformed edit DTOs, and direct-draw leakage, plus positive card/diff/progress/approval tests. r[agent_harness_runtime.tui_tools.verification]
