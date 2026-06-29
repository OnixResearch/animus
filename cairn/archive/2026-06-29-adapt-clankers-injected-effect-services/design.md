# Design: Injected services and fail-closed effects

## Context

Clankers' embedding boundary separates catalog construction, provider/router execution, extension services, auth stores, confirmation brokers, and effect handlers. Animus needs the same safety split in a smaller form: reusable runtime logic selects typed actions; host-owned services decide and execute side effects.

## Decisions

### 1. Runtime dispatch consumes injected services

Model/provider calls, tool execution, host effects, confirmation, cancellation, policy checks, and persistence-capable operations are accessed through injected traits or adapter contracts. Reusable runtime code must not construct concrete clients or read ambient host state.

### 2. Missing service is a first-class result

Absent, unavailable, denied, cancelled, timed-out, and malformed-output outcomes are typed. They block further side effects and produce safe receipts or events that do not claim success.

### 3. Confirmation defaults to deny

When a protected action requires confirmation and no broker is available, the runtime denies by default. Broker errors for unavailable, timed-out, or cancelled decisions also deny unless an explicitly typed fatal error should stop the turn.

### 4. Receipts record decisions, not payloads

Effect receipts include service kind, capability class, decision status, reason class, bounded counts, hashes, and redaction markers. They omit credentials, raw provider bodies, raw tool args/output, environment values, and full paths.

### 5. Boundary tests forbid ambient behavior

Focused tests or static rails assert reusable runtime code does not read credentials, discover provider config, open network connections, execute product tools, or persist sessions except through injected services.

## Risks / Trade-offs

- More traits can over-abstract early code. Start with the smallest service seams needed by existing model/tool adapters.
- Deny-by-default behavior can surprise embedders. It is safer and easier to document than implicit ambient authority.
- Static boundary tests may need maintenance as crates move.