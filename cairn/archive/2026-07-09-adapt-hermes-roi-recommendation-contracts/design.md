# Design: Adapt Hermes ROI recommendation contracts

## Context

ROI recommendations are adjacent to workflow commands, policy profiles, review reports, and Cairn lifecycle. The reusable value is the structured report and admission-gated action descriptor, not the command UI or model prompt.

## Decisions

### 1. ROI reports are bounded structured outputs

**Choice:** A recommendation report contains focus, best next step, score dimensions, first actions, runner-ups, lifecycle status, recommended action, reason, and missing context.

**Rationale:** Structured fields make ROI recommendations testable and host-renderable.

### 2. Score inputs are evidence labels, not raw scans

**Choice:** Reports can reference impact, effort, confidence, unblock value, risk reduction, and evidence labels supplied by hosts. Reusable logic does not scan repos or infer scores.

**Rationale:** Evidence collection and ranking heuristics are product behavior.

### 3. Actions are descriptors that require admission

**Choice:** Possible follow-ups such as begin now, create/update Cairn first, investigate, dismiss, or delegate are action descriptors. They cannot trigger work without host policy and admission.

**Rationale:** ROI tooling should not bypass lifecycle or mutate by suggestion.

### 4. Lifecycle routing is neutral

**Choice:** The contract can mention lifecycle status and route hints without assuming Cairn is always present. Animus-hosted repos may map hints to Cairn; other embedders can map to their own lifecycle systems.

**Rationale:** Animus is a framework for building agents, not a repo-specific lifecycle product.

## Risks / Trade-offs

- Recommendations may appear authoritative. Receipts should capture confidence and missing context.
- Hosts may want automatic action execution. Keep execution separate and admitted.
- Cairn-specific wording could leak into generic SDK APIs. Use lifecycle-routing descriptors with Cairn as one host mapping.
