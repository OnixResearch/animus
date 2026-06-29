# Plugin manifest descriptor boundary

Animus treats Clankers plugin manifests as a bounded reference for descriptor shape, not as runtime behavior. The reusable crates may validate data-only extension descriptors, but they must not launch stdio processes, load Wasm modules, connect to a network, inherit environment values, read secrets, execute tools, or enforce a sandbox as part of descriptor parsing.

## Descriptor scope

The descriptor vocabulary is Animus-owned and optional. It covers:

- identity;
- runtime kind labels such as `host`, `stdio`, and `wasm`;
- declared tool names and descriptions;
- capability labels;
- launch policy metadata such as command or artifact labels, argument labels, and working-directory mode;
- environment allowlist names, never values;
- project-relative writable-root descriptors;
- sandbox requirements as host-enforced claims;
- network policy claims; and
- redaction claims for future host logs.

These fields are claims and metadata. A descriptor does not grant authority. A future host that wants execution must add a separate injected service boundary and prove sandbox, network, environment, process, Wasm, and receipt handling before dispatch.

## Validation behavior

`animus-adapters` owns the no-std functional core for descriptor validation. It rejects missing required identity/runtime/sandbox/network fields, unknown runtime kinds, missing launch labels for `stdio` and `wasm` descriptors, duplicate declared tools, built-in or catalog tool-name collisions, path escapes, absolute writable roots, environment value leakage, and outbound network claims without isolation.

Positive fixtures are encoded as unit tests for safe `stdio` and `host` descriptors. Negative fixtures cover missing identity, unknown runtime kind, missing launch label, duplicate tools, built-in collisions, catalog collisions, path escapes, absolute writable roots, environment values, and unsafe network/sandbox combinations.

## Runtime-neutrality rail

`scripts/cairn/check-plugin-manifest-runtime-neutral --root .` scans the manifest validation core for forbidden execution APIs and verifies the architecture note names descriptor-only scope. This rail is intentionally narrow: descriptor parsing must stay pure data validation, and any future execution support needs a separate change.
