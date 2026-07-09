# Tasks

- [x] [serial] Define focus deck, focus item, lens task packet, lens report, synthesis report, contradiction, and review-criteria DTOs. r[agent_harness_runtime.hermes_multi_lens.focus_decks]
- [x] [depends:focus_decks] Add pure validation for focus IDs, deck bounds, retrieval budget references, evidence expectations, output contracts, and provenance labels. r[agent_harness_runtime.hermes_multi_lens.validation]
- [x] [depends:validation] Add host-owned execution planning over injected worker services or sequential checklist modes without default subagent spawning. r[agent_harness_runtime.hermes_multi_lens.host_owned]
- [x] [depends:host_owned] Add synthesis reducers that preserve lens provenance, skipped lenses, assumptions, contradictions, confidence, and unresolved evidence gaps. r[agent_harness_runtime.hermes_multi_lens.synthesis]
- [x] [depends:synthesis] Add positive and negative tests for deck validation, task packets, reports, synthesis, missing service, unsafe context, and no baked-in Hermes focus set. r[agent_harness_runtime.hermes_multi_lens.verification]
