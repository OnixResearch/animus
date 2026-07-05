# Tasks

- [ ] [depends:add-tui-conversation-render-model] Apply model-streaming text and thinking deltas and content-block start/stop lifecycle events to the conversation render model incrementally. r[agent_harness_runtime.tui_streaming.delta_lifecycle]
- [ ] [depends:delta_lifecycle] Render accumulated markdown blocks through host-only `rat-markdown` widgets bounded by the visible window, with no Rats types in reusable crates. r[agent_harness_runtime.tui_streaming.markdown_render]
- [ ] [depends:delta_lifecycle] Render multimodal image payloads as bounded display images with safe metadata and no retained raw provider bytes. r[agent_harness_runtime.tui_streaming.image_payloads]
- [ ] [depends:delta_lifecycle] Add a bounded scrollback window with safe eviction metadata so streaming does not grow render state unbounded. r[agent_harness_runtime.tui_streaming.scrollback]
- [ ] [depends:delta_lifecycle,markdown_render,image_payloads,scrollback] Run focused streaming/snapshot/image tests, dependency-isolation checks, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.tui_streaming.verification]
