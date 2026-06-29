# Design: Optional plugin manifest boundary

## Context

Clankers' plugin documentation and runtime code show useful descriptor fields, but the product runtime is larger than Animus' current scope. Animus can evaluate manifest contracts now while keeping plugin execution out of scope.

## Decisions

### 1. Manifest parsing is descriptor-only

Animus may parse plugin manifests as data. Parsing cannot launch stdio processes, load Wasm modules, connect to external services, inherit environment values, resolve credentials, or execute tools.

### 2. Runtime kind is descriptive until a host supplies execution

Runtime kinds such as `wasm`, `stdio`, or `host` are labels for descriptors. Execution requires an injected extension/tool service and a separate change.

### 3. Launch policy fields are sanitized

A manifest may declare command labels, args, working directory mode, environment allowlist names, sandbox mode, writable roots, and network intent. Validation rejects path escapes, absolute writable roots, unknown modes, missing required fields, and env value leakage.

### 4. Tool registrations cannot steal built-ins

Declared tools are validated against built-in names and existing catalog names. Collisions fail closed.

### 5. Sandbox declarations are claims, not enforcement

Manifest sandbox fields describe required host enforcement. They do not grant authority. If execution is later added, enforcement must happen in the host service before process/Wasm launch.

## Risks / Trade-offs

- Descriptor fields can create an illusion of support. Docs must state that execution is out of scope.
- Stdio and Wasm have different security models. Keep the first vocabulary generic and validation-focused.
- Path validation is host-dependent; use project-relative strings and reject absolute/escape forms by default.