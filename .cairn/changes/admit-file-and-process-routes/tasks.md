# Tasks

## 1. Admitted vocabulary

- [x] [serial] 1.1 Widen `ALLOWED_OPERATIONS` to the route classes the runtime implements. r[agent_harness_runtime.live_steel_agent_shell.configuration]
- [x] [serial] 1.2 Keep `REQUIRED_OPERATIONS` unchanged so admission stays an explicit subset choice. r[agent_harness_runtime.live_steel_agent_shell.configuration]
- [x] [serial] **Negative test** 1.3 Refuse `kernel_clear`, an operation the runtime implements but the vocabulary does not admit. r[agent_harness_runtime.live_steel_agent_shell.configuration]
