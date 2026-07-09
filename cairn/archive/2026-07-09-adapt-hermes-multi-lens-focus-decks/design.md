# Design: Adapt Hermes multi-lens focus deck contracts

## Context

The accepted subagent and retrieval-budget contracts already establish scoped worker tasks, minimized context packets, and retrieval guidance. Multi-lens analysis can be modeled as a higher-level task planning and synthesis layer over those contracts.

## Decisions

### 1. Focus decks are data, not prompts

**Choice:** A focus deck contains focus IDs, descriptions, look-for criteria, expected evidence shape, retrieval budget reference, output contract, and optional synthesis criteria.

**Rationale:** Hosts can render prompts or UI from deck data, but reusable logic can validate deck safety without owning wording or execution.

### 2. Lens execution is host-owned

**Choice:** Animus can plan lens task packets and validate returned reports. It does not spawn agents, call models, create threads, or enforce parallelism.

**Rationale:** Parallel workers are product behavior. An SDK should support them through injected services.

### 3. Synthesis preserves provenance and contradictions

**Choice:** Synthesis results must cite lens IDs, inspected source labels, evidence hashes, assumptions, contradictions, and resolution status. Unresolved contradictions remain explicit.

**Rationale:** Multi-perspective analysis is valuable only if provenance survives aggregation.

### 4. Property-discovery is one deck family

**Choice:** Property-discovery focuses may be represented as a deck family, but the contract remains generic so hosts can define domain-specific decks.

**Rationale:** Avoid hard-coding Hermes' exact focus lists into Animus.

## Risks / Trade-offs

- Focus deck schemas can become too abstract. Keep initial fields aligned with task packets and synthesis needs.
- Without built-in execution, embedders must wire worker services. This is intentional for framework separation.
- Lens reports may disagree. Validation should preserve disagreement rather than hiding it.
