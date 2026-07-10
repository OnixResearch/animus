# Std-shell cap-std context boundary

`animus-std` owns local filesystem effects. It wraps operator-declared workspace, context-file, artifact, and host-store roots as typed capability roots and resolves context files/directories through capability-relative `cap-std` operations.

Reusable crates continue to receive in-memory DTOs, adapter contracts, loaded context values, or validated relative locators. They do not depend on `cap-std` and do not open ambient filesystem paths.

Capability roots bound local filesystem reads and writes only. They do not sandbox model/tool behavior, provider trust, process execution, credentials, network access, daemon behavior, persistence safety, or TUI safety.
