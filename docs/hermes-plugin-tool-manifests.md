# Hermes-style plugin tool manifests

Animus adapts the Hermes plugin manifest boundary as typed host-provided metadata. Hosts may parse YAML, embedded bundles, application databases, or other stores, then pass manifest fields to `animus-adapters`; reusable Animus crates do not import plugin modules, run scripts, inspect installation directories, shell out, or call providers as a manifest side effect.

Validated plugin manifests include a toolset identity, version, toolset kind, runtime kind, safe provenance label, host trust status, declared tool descriptions, parameter schema identities, result contract identities, capability labels, sandbox/network claims, launch labels, and redaction hints.

Manifest-provided tools project into typed tool definitions only. They become prompt-active when the manifest is trusted and the host injects a matching executor. Missing executors fail with a typed `MissingExecutor` dispatch class and no fallback attempt. Capability labels distinguish read-only inspection from metrics writes, filesystem mutation, process execution, network access, and other side-effect classes before prompt or tool metadata is shown.

Action, event-hook, host-pane entrypoint, and link-handler roles are also manifest data. Animus validates stable IDs, titles, safe context labels, command/artifact labels, inherited compatibility, and warning-vs-blocking diagnostics without linking registries, opening panes, reading plugin storage, installing plugins, or executing commands. Hosts own link/unlink/enable/disable/list/action-list state and may only dispatch a planned role after compatibility, policy, enabled-state, and safe-context checks pass.
