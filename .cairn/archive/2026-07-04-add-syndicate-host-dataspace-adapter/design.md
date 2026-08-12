# Design: Add optional Syndicate host dataspace adapter

## Context

The accepted dataspace contract already requires bounded assertions, explicit scope/lifecycle semantics, safe subscriptions, fake-backend fixtures, and dependency isolation. `animus-runtime` must not construct live Syndicate backends. `animus-std` is the right shell for host configuration, credentials, network/daemon handles, and optional live transport wiring.

## Decisions

### 1. Live coordination is a host adapter, not runtime state

**Choice:** Implement any Syndicate-compatible integration as an optional `animus-std` adapter that satisfies `animus_adapters::dataspace::AdapterService`.

**Rationale:** The reusable loop should publish typed assertions through an injected service. Host shells own side effects, process/network authority, daemon availability, and credentials.

### 2. Projection is a pure functional core

**Choice:** Add pure projection functions that convert approved runtime/headless/tool/background/session/agent status observations into dataspace assertion drafts, then keep actual publish/subscribe/event handling in the host shell.

**Rationale:** Projection rules need positive and negative tests without standing up a daemon. The imperative shell should only validate configuration, call transport APIs, and translate failures into safe diagnostics.

### 3. Redaction markers are mandatory for sensitive sources

**Choice:** Assertions derived from prompt, provider, file, tool, model, path, credential, or log-bearing sources must include explicit redaction markers and safe hashes/counts instead of raw content.

**Rationale:** Dataspace assertions are shared state. Making redaction evidence mandatory prevents accidental broadcast of private material and gives subscribers enough safe evidence to reconcile state.

### 4. Configuration and live smoke tests are opt-in

**Choice:** Default tests use the deterministic fake backend. Live Syndicate smoke tests run only when an explicit host config and environment gate are provided.

**Rationale:** Release evidence must remain deterministic and local. Live transport is valuable, but it should not be required for normal development, CI, or reusable crate consumers.

### 5. Preserves is useful but not required for in-process publication

**Choice:** The host adapter may use Preserves-encoded dataspace DTOs for cross-process boundaries when the Preserves dataspace projection change is available, but in-process publication can operate directly on typed DTOs.

**Rationale:** Syndicate is the coordination fabric; Preserves is the portable boundary language. They compose best at process/language boundaries without coupling either to core logic.

## Risks / Trade-offs

- Pulling in live transport can leak dependencies. Keep the feature non-default and extend isolation checks.
- Runtime event types are broad. Start with a small, documented set of status/progress/assertion classes and expand by adding safe projection fixtures.
- Live smoke tests may be flaky if they rely on external daemon state. Require explicit opt-in configuration and keep fake-backend tests authoritative for release evidence.
