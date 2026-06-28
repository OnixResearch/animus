# Design: Adapt Claude Code headless automation surfaces for Animus

## Context

Claude Code can run from terminal, scripts, CI, and SDK integrations. The transferable contract is a stable noninteractive agent-run surface with structured input, event output, and final status. Animus should support those surfaces through adapter-neutral DTOs and shell-owned I/O.

Existing Pi headless/RPC work can inform transport details. This change focuses on run contract, event schema, serialization projections, noninteractive behavior, and CI-friendly completion.

## Decisions

### 1. Headless runs use typed request contracts

**Choice:** A headless run request declares objective, session/workspace identity, input references, policy profile, allowed capabilities, output contract, validation plan, and noninteractive behavior.

**Rationale:** Scripts and CI require stable validation before the runtime starts model or tool work.

### 2. Event streams are DTO-first

**Choice:** Define event DTOs independent of encoding, then provide JSONL and Preserves projections at adapter boundaries.

**Rationale:** JSONL is convenient for shells and CI, while Preserves is useful for portable typed boundaries. Neither should dictate core state.

### 3. Noninteractive prompts fail explicitly

**Choice:** Headless runs cannot silently prompt the user. If a permission ask, missing input, or policy decision requires interactivity, the run emits a typed prompt-required or input-required status according to policy.

**Rationale:** CI and scripts need deterministic outcomes, not hanging prompts.

### 4. Completion receipts are safe and machine-readable

**Choice:** Final status records success, denied, prompt-required, validation-failed, service-unavailable, cancelled, or execution-failed classes with safe evidence references and hashes.

**Rationale:** Automation needs precise failure modes and redacted evidence.

## Risks / Trade-offs

- Event schemas can become too broad. Start with lifecycle events already present at adapter boundaries.
- JSONL compatibility can encourage stringly typed core code. Keep serialization in adapters.
- CI users may want PR creation. Leave PR creation to explicit host integrations outside the default headless contract.
