## Why

`animus-std` is the standard-library host shell that discovers context files, environment-backed configuration, and default filesystem adapters. That shell is the right place to bound filesystem authority with `cap-std`; reusable runtime, adapter, Steel, and core crates should stay independent of local filesystem APIs.

## What Changes

- Adopt `cap-std` in `crates/animus-std` for context and host-owned filesystem boundary operations.
- Introduce typed capability roots for workspace, context-file, artifact, and host-store roots.
- Convert `crates/animus-std/src/context.rs` and related std-shell path opens to capability-relative operations.
- Add positive and negative fixtures for valid context layering, `../`, absolute paths, symlink escapes, and missing roots.

## Impact

- Host filesystem authority becomes explicit at the Animus std-shell edge.
- No-std core, adapter contracts, Steel fabric, reusable runtime, and TUI contracts remain filesystem-neutral.
- Invalid context paths deny before hidden host files enter the agent context.
