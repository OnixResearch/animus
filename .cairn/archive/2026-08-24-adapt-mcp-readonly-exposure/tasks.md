## Phase 1: Default read-only exposure

- [x] [serial] r[animus.mcp.exposure] Add the pure descriptor-to-exposed-catalog reducer.
- [x] [parallel] r[animus.mcp.exposure.readonly] Add a positive `readOnlyHint: true` exposure fixture.
- [x] [parallel] r[animus.mcp.exposure.mutating] Add negative missing, false, malformed, and unknown annotation fixtures.

## Phase 2: Authority and claims

- [x] [serial] r[animus.mcp.authority] Intersect non-read-only exposure with exact host and Basalt authority.
- [x] [parallel] r[animus.mcp.authority.exact] Add positive and negative exact-tool authority fixtures.
- [x] [parallel] r[animus.mcp.authority.non_claim] Add a fixture that forbids read-only behavioral claims from descriptor metadata.

## Phase 3: Catalog and child isolation

- [x] [serial] r[animus.mcp.collision] Reject built-in and configured tool-name collisions without silent renaming.
- [x] [parallel] r[animus.mcp.collision.duplicate] Add a negative duplicate-name fixture that names every source.
- [x] [serial] r[animus.mcp.child] Intersect the root MCP set with child-session authority and server generation.
- [x] [parallel] r[animus.mcp.child.intersection] Add a fixture that rejects child authority broadening.

## Phase 4: Verification

- [x] [serial] r[animus.mcp.verification] Add the focused positive, negative, and boundary matrix and run the rails.
