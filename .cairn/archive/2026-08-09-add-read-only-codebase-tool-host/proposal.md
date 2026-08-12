## Why

Animus has a coherent tool admission kit and a capability-scoped codebase graph, but no concrete product-neutral tool executor. The executor kit also omits request values from the service call and discards bounded model-visible output. A concrete executor would otherwise act without an exact value binding and could not return useful output.

## What Changes

- Bind each admitted tool plan to the exact current request values and pass that request to the selected executor.
- Return bounded model-visible output together with the existing metadata-only receipt.
- Add a non-default read-only codebase-query executor over the existing explicit `cap-std` graph host.
- Add a packaged deterministic smoke and focused Rust/Nix verification rail.
- Keep provider contact, mutation, shell execution, network access, path discovery, and live-provider evidence outside this change.

## Impact

- **Files**: executor-kit core/tests, `animus-std` feature/module/binary, root facade metadata, docs, Cairn catalog and accepted spec, focused scripts, and `flake.nix`.
- **Testing**: executor-kit positive and negative tests, capability-root filesystem fixtures, packaged smoke, dependency isolation, Clippy, Cairn gates, and focused Nix checks.
