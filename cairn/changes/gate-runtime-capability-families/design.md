# Design: Gate runtime capability families

## Context

The runtime crate currently exposes many independent domains through one broad facade. Crate-level boundaries remain useful, but feature-level boundaries can improve embedder composition and keep optional concepts from becoming accidental requirements.

## Decisions

### 1. Define capability families before adding gates

**Choice:** Create a maintained map from runtime modules and public exports to capability families such as `loop`, `headless`, `provenance`, `batch`, `worker-team`, `policy`, `quality`, `research`, `scaffold`, `compaction`, and `dataset-export`.

**Rationale:** Feature gates should reflect documented composition domains, not arbitrary file boundaries.

### 2. Keep a compatibility bundle

**Choice:** Provide an explicit compatibility feature or default bundle that preserves the current broad surface for existing consumers.

**Rationale:** Fine-grained composition should be opt-in for new embedders without breaking current ones.

### 3. Gate exports and tests together

**Choice:** Public exports, module inclusion, docs, examples, and tests must use the same feature names.

**Rationale:** A feature gate that only hides exports but still compiles unrelated modules does not improve composability.

### 4. Validate disabled-family absence

**Choice:** Add compile and facade inventory checks that prove minimal builds do not expose or require disabled family APIs.

**Rationale:** Disabled-family leaks are easy to miss when a compatibility facade exists.

### 5. Avoid premature crate explosion

**Choice:** Start with Cargo features and inventory mapping; split crates only when a family needs independent dependency ownership or release cadence.

**Rationale:** Feature gates can reduce surface area without creating too many packages at once.

## Risks / Trade-offs

- Feature matrices can become hard to test. Keep a small set of canonical profiles: minimal, selected family, compatibility/all.
- Conditional docs can rot. Use inventory checks and compile examples as the source of truth.
- Existing consumers may depend on default broad exports. Keep compatibility defaults stable until a planned migration changes them.
