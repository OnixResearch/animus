# Design: Packaged TUI dogfood smoke

## Context

The accepted packaged headless smoke runs a bounded prompt through deterministic fake services. The TUI crate already owns pure view reducers and host-only terminal dependencies.

The missing evidence is one packaged composition that uses both surfaces. This evidence must stay deterministic and must not require a live terminal.

## Decisions

### 1. Keep the new binary non-default

**Choice:** Add `animus-tui-dogfood-smoke` behind `animus-tui/packaged-tui-dogfood-smoke`.

**Rationale:** Default workspace users do not need a product TUI smoke or its direct Ratatui dependency.

### 2. Reuse the accepted headless path

**Choice:** Pass the admitted CLI arguments to `animus_std::run_packaged_headless_dogfood_smoke`.

**Rationale:** The TUI smoke must compose accepted behavior. It must not copy model, tool, capability, event, or receipt logic.

### 3. Separate pure projection from backend rendering

**Choice:** Pure helpers validate the bounded headless receipt and build an `AppShellState` snapshot plan. A small shell renders that plan with `ratatui::backend::TestBackend`.

**Rationale:** Receipt parsing, bounds, state selection, and evidence projection remain testable without a terminal. The backend shell only allocates, draws, and observes the fixed buffer.

### 4. Use fixed bounds and metadata-only output

**Choice:** Use named receipt, event, viewport, and render limits. Output only status labels, counts, BLAKE3 identities, and non-claims.

**Rationale:** Raw prompts, model output, tool output, provider payloads, private paths, and the complete rendered buffer are not evidence fields.

### 5. Treat cleanup as an evidence gate

**Choice:** Successful evidence requires an available test backend and a complete cleanup observation. Missing terminal state and cleanup failure return stable errors.

**Rationale:** A successful render is not sufficient when terminal ownership ends in an unknown state.

### 6. Keep the claim bounded

**Choice:** The focused receipt claims only a packaged deterministic TestBackend TUI path.

**Rationale:** This evidence does not prove raw-mode setup, live input polling, terminal restoration, arbitrary terminal compatibility, production readiness, or release eligibility.

## Risks / Trade-offs

- The TestBackend runs the real Ratatui render path, but it does not exercise a live terminal.
- The smoke depends on the safe text contract of the packaged headless receipt.
- The feature adds a direct optional Ratatui dependency only to `animus-tui`.
- The fixed viewport can miss behavior that occurs only at other terminal sizes.
