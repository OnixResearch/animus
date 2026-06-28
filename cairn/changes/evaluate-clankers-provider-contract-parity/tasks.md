# Tasks

- [ ] [serial] Add an architecture note evaluating Clankers provider/router contract parity patterns and documenting when Animus should adopt them. r[agent_harness_runtime.clankers_provider_parity.scope]
- [ ] [depends:clankers_provider_parity.scope] Define future shared request-field expectations for provider/router DTOs when Animus grows beyond the current model adapter contract. r[agent_harness_runtime.clankers_provider_parity.request_fields]
- [ ] [depends:clankers_provider_parity.request_fields] Define a constructor-inventory rail pattern for concrete request structs when multiple constructors exist. r[agent_harness_runtime.clankers_provider_parity.constructor_inventory]
- [ ] [depends:clankers_provider_parity.request_fields] Define provider-native message conversion fixture requirements and reject lossy generic JSON conversion for routed backends. r[agent_harness_runtime.clankers_provider_parity.message_conversion]
- [ ] [depends:clankers_provider_parity.scope] Define streaming parser-entrypoint test requirements for future stream/SSE providers. r[agent_harness_runtime.clankers_provider_parity.streaming]
- [ ] [depends:clankers_provider_parity.scope] Define known-provider-prefix fail-closed behavior for future provider-qualified model IDs. r[agent_harness_runtime.clankers_provider_parity.fail_closed_prefix]
- [ ] [depends:clankers_provider_parity.constructor_inventory,clankers_provider_parity.message_conversion,clankers_provider_parity.streaming,clankers_provider_parity.fail_closed_prefix] Run docs/rail validation, workspace tests if touched, and Cairn validation/gates before archive. r[agent_harness_runtime.clankers_provider_parity.closeout]
