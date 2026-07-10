# Tasks

- [x] [serial] Add a non-default headless/CLI dogfood shell entrypoint that accepts a bounded prompt request and wires existing runtime contracts through injected deterministic fake services. r[agent_harness_runtime.dogfood_headless_shell.entrypoint]
- [x] [depends:entrypoint] Admit prompt, policy profile, model capability, optional tool metadata, output contract, and noninteractive behavior before any adapter dispatch. r[agent_harness_runtime.dogfood_headless_shell.admission]
- [x] [depends:admission] Dispatch through fixture model/tool adapters and emit ordered runtime events for admission, model request, model response, optional tool admission, completion, and shutdown. r[agent_harness_runtime.dogfood_headless_shell.events]
- [x] [depends:events] Produce a shell-specific safe receipt index with event counts, adapter labels, status classes, BLAKE3 identities, redaction markers, and omitted-surface non-claims. r[agent_harness_runtime.dogfood_headless_shell.receipts]
- [x] [depends:receipts] Add docs/examples that name the fake-backed integration evidence rung and explicitly exclude live-provider, persistence, daemon, TUI, plugin, production, and Clankers-parity claims. r[agent_harness_runtime.dogfood_headless_shell.docs]
- [x] [depends:docs] Add positive fake-backed smoke tests and negative missing-service, denied-capability, malformed-output, unsafe-evidence, receipt-leak, and overclaim fixtures; run dogfood readiness and adapter conformance rails. r[agent_harness_runtime.dogfood_headless_shell.verification]
