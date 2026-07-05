# Pi-style resource discovery

Animus adapts Pi's progressive resource-discovery pattern as typed host-provided data. Hosts discover skills, prompt templates, context guidance, and application resources, then pass validated descriptors into Animus. Reusable adapter/runtime crates do not scan project directories, install packages, resolve symlinks, read resource bodies by ambient path, or execute extension code.

The generic prompt-resource descriptor includes resource kind, name, summary description, source scope, safe provenance label, host trust status, and a lowercase BLAKE3 content identity. Invalid names, unsupported kinds/scopes/trust labels, path-like provenance, missing descriptions, and unsafe content handles fail before prompt construction.

Project-local resources fail closed unless the host marks them trusted. Prompt construction advertises bounded summaries only; full resource bodies are included only when the host supplies matching content for an explicit selected resource. Duplicate names disable lookup deterministically and emit safe diagnostics instead of allowing map overwrite behavior.
