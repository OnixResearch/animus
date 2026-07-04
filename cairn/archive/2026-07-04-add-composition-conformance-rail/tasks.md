# Tasks

- [x] [serial] Define the composition conformance matrix for provider-hardening plus model dispatch, tool contract plus provenance receipt, headless event projection plus runtime events, middleware denial plus failure reports, and capability absence plus no ambient fallback. r[agent_harness_runtime.composability.conformance_matrix]
- [x] [depends:conformance_matrix] Add matrix validation tests proving every scenario has an owner, positive case, negative case, feature requirements, and safe evidence label. r[agent_harness_runtime.composability.conformance_matrix.coverage]
- [x] [serial] Add pure fake-backed positive composition tests for selected matrix scenarios without live providers, tools, process execution, network access, terminal UI, or persistence. r[agent_harness_runtime.composability.positive_conformance]
- [x] [depends:positive_conformance] Add negative composition tests for malformed responses, unauthorized tools, missing services, oversized output, unsafe receipts, and disabled feature surfaces. r[agent_harness_runtime.composability.negative_conformance]
- [x] [serial] Add a focused conformance script and optional Nix check that run only deterministic in-memory composition scenarios. r[agent_harness_runtime.composability.conformance_rail]
- [x] [depends:conformance_rail] Add negative leak checks for rail evidence output containing raw prompts, provider payloads, credentials, full paths, or unbounded tool outputs. r[agent_harness_runtime.composability.conformance_rail.safe_evidence]
- [x] [depends:conformance_matrix.coverage,negative_conformance,conformance_rail.safe_evidence] Update docs and run focused conformance, workspace, Nix check if added, and Cairn gates with evidence. r[agent_harness_runtime.composability.conformance.verification]
