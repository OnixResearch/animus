## Context

Animus already treats exact model output and process exit as advisory. Repository checks and Cairn receipts retain completion authority.

The repository pinned an older Cairn revision. Its wrapper also omitted policy injection for `change` commands.

## Decisions

### Decision: Own the published lifecycle runner

**Choice:** Pin Cairn revision `5e1fe08d62149b286ff583eb056e655fe92e3cf1` and inject its generated policy for `change` commands.

**Rationale:** Animus changes can select the profile without an ambient sibling checkout or private model state.

### Decision: Preserve Animus completion authority

**Choice:** Keep `scripts/cairn/check-authoritative-agent-completion --root .` as repository-owned evidence.

**Rationale:** Cairn checks review structure and evidence identity. Animus retains the meaning of agent completion.

### Decision: Retain negative lifecycle evidence

**Choice:** Store the planned-state sync rejection before attaching completion evidence.

**Rationale:** The pilot must show that a fast unsupported result is rejected.

## Risks / Trade-offs

- The profile does not prove that the selected checks are sufficient.
- The Cairn source update can expose independent dependency or policy drift.
- Fresh-state success does not establish release readiness.
