# Design: Concrete std runtime adapters

## Context

Animus' architecture correctly defers host effects to adapters. The next missing plan is where real std-shell adapters live, what contracts they must consume, and how deterministic tests cover them without requiring credentials, network, or host mutation.

## Decisions

### 1. Keep concrete adapters in the shell

Concrete model clients, filesystem tools, process execution, HTTP/network helpers, credential resolution, and platform sandboxing belong in `animus-std` or optional host crates. Reusable runtime crates receive injected services only.

### 2. Gate effects through accepted contracts

Concrete adapters must check accepted tool schemas, provider/model registry metadata, UCAN/local policy decisions, sandbox descriptors, and host approval outcomes before dispatching effects. If a prerequisite contract is absent, the adapter fails closed.

### 3. Prefer deterministic fixture adapters in default tests

Default tests use fake provider, fake filesystem, fake process, fake clock, and fake network adapters over in-memory data. Live provider calls, real process execution, and filesystem mutation are opt-in smoke tests with explicit host configuration.

### 4. Emit safe receipts and events

Every concrete adapter result maps into runtime events and redacted receipts that include stable identities, status classes, counts, hashes, and issue codes without raw prompts, provider payloads, credentials, full paths, unbounded command output, or private file bodies.

### 5. Split adapter families when scope grows

If model providers, filesystem/process tools, or HTTP/network helpers grow independently, each family should receive a narrower follow-up change instead of expanding this boundary change indefinitely.

## Risks

- Concrete adapters can accidentally become default dependencies. Guard with feature flags and dependency-isolation checks.
- Live tests can become flaky. Keep deterministic fixtures authoritative and live checks opt-in.
