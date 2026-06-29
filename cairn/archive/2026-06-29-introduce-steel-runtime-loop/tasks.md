# Tasks

- [x] [serial] Define the `animus-runtime` crate boundary and public types for runtime input, runtime config, turn events, final report, and runtime errors. r[agent_harness_runtime.loop.contract]
- [x] [depends:loop.contract] Add a pure runtime loop core that consumes a loop-fabric decision stream and returns typed events/reports without reading files, environment, clocks, or process state. r[agent_harness_runtime.loop.core]
- [x] [depends:loop.core] Wire `animus-steel` as the default loop fabric while keeping the runtime boundary generic enough for future fabrics. r[agent_harness_runtime.steel_fabric.default]
- [x] [depends:steel_fabric.default] Dispatch `CallModel`, `CallTool`, `Respond`, and `Stop` decisions through injected adapters and typed report state. r[agent_harness_runtime.adapter_dispatch]
- [x] [depends:adapter_dispatch] Add configurable max-turn handling that fails closed with a typed event/report instead of spinning indefinitely. r[agent_harness_runtime.loop.max_turns]
- [x] [depends:adapter_dispatch] Add positive runtime tests for one-turn model calls, tool calls, final responses, and explicit stop behavior. r[agent_harness_runtime.tests.positive]
- [x] [depends:tests.positive] Add negative runtime tests for invalid Steel decisions, model adapter failure, tool adapter failure, max-turn exhaustion, missing response, and empty/malformed inputs. r[agent_harness_runtime.tests.negative]
- [x] [depends:tests.negative] Add Preserves schema files and round-trip tests for stable adapter/runtime boundary messages once event/report fields are finalized. r[agent_harness_runtime.preserves_schema.contract]
- [x] [depends:preserves_schema.contract] Update README, safety contract, root feature wiring, and flake checks to describe `animus-runtime` as the harness loop crate and Steel as the default fabric. r[agent_harness_runtime.docs.validation]
- [x] [depends:docs.validation] Run focused crate tests, workspace check/clippy/tests, pre-commit, Octet/Tigerstyle, flake checks, and Cairn validation/gates with reviewable evidence before archive. r[agent_harness_runtime.validation]
