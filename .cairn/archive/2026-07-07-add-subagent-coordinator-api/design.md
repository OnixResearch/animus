# Design: Add a host-injected subagent coordinator API

Animus should provide the reusable orchestration skeleton while hosts provide concrete execution. The coordinator API should be boring: it validates, records state transitions, calls injected services, and classifies outcomes.

## Decisions

### 1. Coordinator owns transitions, not execution

**Choice:** The coordinator accepts validated child-run admissions and produces next actions or state transitions. It never constructs providers, queue clients, shells, workers, or external-agent clients.

**Rationale:** This preserves runtime purity and lets embedders plug in different execution systems.

### 2. Service operations are explicit

**Choice:** `SubagentService` exposes enqueue, status, cancel, and collect operations with typed handles and failure classes.

**Rationale:** These are the minimum common lifecycle operations already reflected by worker-team patterns and fresh-context scheduling.

### 3. Missing service fails before dispatch

**Choice:** Coordinator entrypoints return a service-unavailable failure when no service is injected for the child-run kind.

**Rationale:** Missing services should not trigger ambient fallback, CLI discovery, or model calls.

### 4. Lifecycle events are safe projections

**Choice:** State transitions emit bounded lifecycle events with safe identifiers, status labels, failure classes, and optional evidence references.

**Rationale:** UI, headless, provenance, and audit layers need visibility without raw child logs or transcripts.
