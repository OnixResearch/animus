# Proposal: Adapt Hermes plugin and tool manifests for Animus

## Why

Hermes exposes bounded toolsets through plugin manifests such as `review-gate/plugin.yaml`: operators can discover tool names and descriptions without loading raw implementation code into the agent runtime. Animus should adapt that manifest boundary so hosts can register toolsets declaratively, distinguish read-only and mutating actions, and inject executors through typed contracts.

## What Changes

- Define typed plugin/toolset manifest DTOs with toolset name, version, kind, provided tools, descriptions, capability labels, trust status, and manifest provenance.
- Separate manifest discovery from executor injection: reusable runtime consumes validated tool definitions and never imports plugin code from disk.
- Add explicit read-only, dry-run, and mutation capability labels for every advertised tool action.
- Add deterministic manifest/tool name collision handling and safe diagnostics.
- Add positive and negative fixtures based on the Hermes review-gate plugin shape.

## Impact

- **Files**: future tool definition DTOs, plugin manifest validation fixtures, capability-label prompt tests, executor-injection boundary tests, and docs.
- **Testing**: valid manifest acceptance plus malformed YAML/exported data, duplicate tool names, missing descriptions, missing capability labels, untrusted plugin, unknown kind, and absent executor rejection.
- **Non-claims**: this change does not adopt Hermes plugin installation, Python helper scripts, `plugin.yaml` as the only source format, or implicit plugin code loading.
