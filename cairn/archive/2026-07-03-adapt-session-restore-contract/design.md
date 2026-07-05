# Design: Adapt session restore contract for Animus

## Context

Herdr's session-state documentation is valuable because it avoids ambiguous persistence claims. It distinguishes live process persistence from snapshot reconstruction, recent screen replay, native agent resume, and experimental handoff. Animus should encode the same distinctions as typed restore plans and safe receipts.

## Decisions

### 1. Restore claims are explicit capabilities

**Choice:** Model restore as a plan with declared capabilities: process still live, layout reconstructed, display history replayed, native conversation resumed, or unsupported fallback.

**Rationale:** Users and host adapters need to know whether a process survived or whether the host created a replacement shell or agent process.

### 2. Native session references are safe metadata

**Choice:** Store session references as typed records with source, agent, kind, safe value identity, freshness, uniqueness scope, and resume capability label. Receipts should use hashes or bounded safe identifiers by default.

**Rationale:** Session references may be sensitive and may become stale. They need validation before any host attempts resume.

### 3. History replay is display replay, not process survival

**Choice:** Pane or transcript history is disabled by default. If a host enables it, the restored history is marked as replayed display content with redaction and truncation metadata.

**Rationale:** Recent output can contain prompts, secrets, or command output. It also must not imply a process is still alive.

### 4. Restore planner is pure; execution is host-owned

**Choice:** Reusable logic validates snapshots and returns restore plans. Hosts own storage reads, PTY management, process spawning, command construction, and handoff mechanics.

**Rationale:** Restore planning can be tested deterministically, while effectful restoration remains in the imperative shell.

### 5. Degraded restoration is observable

**Choice:** Unsupported, duplicated, stale, malformed, or unavailable resume references produce explicit fallback outcomes and safe receipts.

**Rationale:** Silent downgrade from conversation resume to a blank shell would mislead users and automation.

## Risks / Trade-offs

- Hashing or redacting session references can make debugging harder. Explain records should preserve source and reason codes while avoiding sensitive values.
- Hosts differ in what can actually survive restart. Capability labels must be host-declared rather than assumed by core.
- History replay can tempt callers to feed old terminal output back to models. The contract should keep replay display-only unless a separate policy admits it as context.
