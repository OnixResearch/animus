# Design

## Context

The selected OMP worker has no child delegation export. Animus owns child-session facts; Lattice owns execution; Kiln owns attempt authority and display.

## Decisions

Bind receipt to sealed request, work, attempt, positive fence, worker incarnation and profile. Refuse crossed parent/generation and duplicate children; bound edges; retain explicit coverage. No raw events, prompts, or child output.

## Risks / Trade-offs

Missing or unverified evidence remains unseen, never proof of zero children or authority to retry.
