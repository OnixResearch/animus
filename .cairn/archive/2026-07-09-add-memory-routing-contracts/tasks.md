# Tasks

- [x] [serial] Define memory route request, route candidate, route plan, and route receipt DTOs for read/search, source/import, extraction, store, update/delete, and prompt-influence intents. r[agent_harness_runtime.memory_routing.route_plan]
- [x] [depends:route_plan] Add pure route planning over safe intent, category, scope, adapter-role, confidence, freshness, and budget signals. r[agent_harness_runtime.memory_routing.pure_planner]
- [x] [depends:pure_planner] Add fail-closed route outcomes for denied policy, malformed classifier output, unavailable adapters, unsupported roles, and unsafe metadata. r[agent_harness_runtime.memory_routing.fail_closed]
- [x] [depends:fail_closed] Add positive route-selection tests and negative denied-route, malformed-route, unavailable-adapter, unsupported-role, and leak fixtures. r[agent_harness_runtime.memory_routing.verification]
- [x] [depends:verification] Update docs and run focused memory routing tests, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.memory_routing.verification]
