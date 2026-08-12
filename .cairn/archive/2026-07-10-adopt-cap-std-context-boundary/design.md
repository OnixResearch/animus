## Design

Animus will open ambient workspace, context-file, artifact, and host-store roots only inside `animus-std`. The std shell turns those roots into typed wrappers around `cap_std::fs::Dir` and passes safe relative reads or loaded in-memory content into reusable crates.

The first conversion target is `crates/animus-std/src/context.rs`, followed by any adjacent std-shell filesystem adapters used for context layering or host-owned storage. `animus-core`, `animus-adapters`, `animus-steel`, `animus-runtime`, `animus-tui`, and root facade contracts must not depend on `cap-std`.

Context resolution should keep deterministic precedence and safe provenance receipts, while denying parent traversal, absolute paths, symlink escapes, missing roots, and hidden-file exposure outside declared roots. Tests should include happy-path context layering and negative malformed/escape cases.

Documentation should state that capability roots bound local host filesystem reads only; they do not implement sandboxing for model/tool behavior, provider trust, process execution, credentials, network access, or TUI safety.
