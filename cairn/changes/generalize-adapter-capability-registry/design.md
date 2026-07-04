# Design: Generalize adapter capability composition

## Context

Animus already requires host integrations to cross typed adapter or service contracts. As the runtime adds more optional domains, the composition problem shifts from defining individual traits to wiring them predictably. The runtime should know which capability a decision requires, but it must not construct or discover the concrete service.

## Decisions

### 1. Capabilities are explicit typed requirements

**Choice:** Represent required host authority as typed capability descriptors with stable labels, service kind, optional scope, and fail-closed policy.

**Rationale:** A runtime plan should make capability needs reviewable before dispatch.

### 2. Host supplies a registry or bundle

**Choice:** Introduce a host-supplied capability registry or service bundle that owns references to injected services. Runtime code can resolve declared capabilities from the bundle but cannot create services.

**Rationale:** This provides uniform composition without becoming a global service locator.

### 3. Existing model/tool ergonomics remain

**Choice:** Keep `RuntimeAdapters::new(model, tool)` or equivalent simple constructors as thin compatibility wrappers around the broader service bundle.

**Rationale:** Common usage should remain boring while advanced hosts can compose more capabilities.

### 4. Missing and bad capabilities fail closed

**Choice:** Resolution returns typed outcomes for absent, duplicate, denied, unavailable, cancelled, timed out, malformed, and completed states. Non-completed outcomes stop effects and prevent success receipts.

**Rationale:** Composability cannot trade away safety semantics.

### 5. No ambient fallback

**Choice:** Tests and inventory checks must prove missing services do not trigger environment reads, provider discovery, plugin loading, filesystem traversal, network calls, or process execution.

**Rationale:** Explicit injection is the core safety boundary.

## Risks / Trade-offs

- A registry can become too generic. Keep capabilities typed and prefer narrow service traits over dynamic strings.
- Lifetimes and trait objects can complicate no-std surfaces. Start in runtime shell wiring and keep pure descriptors implementation-free.
- Existing adapter constructors may duplicate paths. Implement them as thin wrappers and test identical outcomes.
