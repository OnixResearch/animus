# Proposal: Adapt Hermes host asset reconciliation contracts

## Why

Hermes keeps versioned source assets and live installed assets separate, with reconciliation categories for symlink-backed, disabled, mirrored, drifted, live-only, and repo-only entries. Animus can use that pattern for host-installed skills, resources, plugins, and tool catalogs, but must not bake in `~/.hermes`, `~/.pi`, symlinks, or any global config layout.

Animus should expose optional reconciliation manifests and drift reports so host shells can audit installed assets while the SDK remains store-neutral.

## What Changes

- Define host asset inventory, manifest entry, installation observation, drift class, reconciliation plan, and safe receipt DTOs.
- Support host-supplied stores for skills, resources, plugin/tool manifests, policy profiles, prompt profiles, and other optional assets.
- Keep filesystem traversal, symlink resolution, copying, linking, unlinking, enabling/disabling, and writing manifests host-owned.
- Add dry-run reconciliation plans and explicit mutation requests with safe diagnostics.

## Impact

- **Files**: future asset reconciliation DTOs, pure diff/classification logic, docs, tests, and focused fixtures.
- **Testing**: positive manifest/drift classification tests plus negative global-path, unsafe-handle, duplicate asset, untrusted mutation, stale observation, and leak fixtures.
- **Non-claims**: this does not require Hermes/Pi directories, manage global skills, link files, install plugins, mutate assets, or make any host store canonical by default.
