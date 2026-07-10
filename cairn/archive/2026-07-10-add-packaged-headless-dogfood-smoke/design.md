# Design: Packaged headless dogfood smoke

## Context

The archived fake-backed dogfood shell added a pure runtime core in `animus-runtime::dogfood_shell`. That core admits already-loaded DTOs and produces ordered events plus safe receipts, but it is not itself a packaged host-shell entrypoint.

The dogfood readiness roadmap names **packaged headless smoke** as the next rung after fake-backed integration. The new surface must stay non-default and host-owned so reusable crates keep no ambient provider, credential, filesystem, network, process, daemon, TUI, plugin, persistence, or production behavior.

## Decisions

### 1. Gate the binary behind `animus-std/dogfood-smoke`

**Choice:** Add `animus-dogfood-smoke` as an `animus-std` binary with `required-features = ["dogfood-smoke"]`, and make `animus-runtime` an optional no-default-features dependency of `animus-std` for that feature.

**Rationale:** `animus-std` owns std-shell behavior, while the root workspace and reusable crates remain free of default product behavior. A required feature keeps `cargo check --workspace --all-targets` from implicitly shipping the smoke binary as a default surface.

### 2. Keep the functional core pure and the binary shell thin

**Choice:** Implement argument parsing, request construction, smoke execution, and safe output rendering as deterministic functions in `dogfood_smoke.rs`; keep `src/bin/animus-dogfood-smoke.rs` to `std::env::args`, stdout/stderr, and exit-code plumbing.

**Rationale:** Tests can exercise the logic without process I/O. The binary only handles the imperative shell responsibilities.

### 3. Project packaged evidence without mutating core evidence

**Choice:** The CLI output states `packaged_rung=packaged-headless-smoke` and preserves the core receipt's `core_rung=fake-backed-integration`.

**Rationale:** The packaged smoke proves executable packaging of the fake-backed path; it must not rewrite the core receipt into a stronger claim. Optional live-provider, TUI, daemon, plugin, persistence, production, and Clankers-parity surfaces remain non-claims.

### 4. Validate through a focused rail

**Choice:** Add `scripts/cairn/check-packaged-headless-dogfood-smoke --root .` and wire `checks.x86_64-linux.packaged-headless-dogfood-smoke`.

**Rationale:** The rail runs unit tests, a positive CLI smoke, negative fail-closed cases, and output leak checks with no credentials or network access.

## Risks / Trade-offs

- The smoke uses deterministic fake services only, so it does not provide live-provider quality or availability evidence.
- The Cargo binary is intentionally feature-gated, so users must opt in with `--features dogfood-smoke` or the Nix check/app surface.
- The line-oriented output is intentionally minimal; richer JSON can be a future shell-owned receipt format if needed.
