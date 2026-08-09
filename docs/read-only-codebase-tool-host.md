# Read-only codebase tool host

Animus exposes one concrete product-neutral tool through the non-default `codebase-tool-host` feature. The tool adapts the existing capability-scoped Rust codebase graph to `animus_runtime::tool_executor_kit`.

## Authority boundary

The caller supplies an explicit capability root and an exact list of relative Rust files. The host does not inspect the current directory, environment, Git state, global indexes, plugins, providers, or network services.

The `codebase_graph_query` tool accepts three exact arguments:

- `symbol`: exact syntactic symbol text;
- `kind`: `any`, `definition`, or `reference`;
- `max_results`: a finite positive limit within the selected graph profile.

The executor returns sorted relative paths, symbol classes, and byte spans. It does not return source bodies. It cannot mutate files, run commands, contact a provider, or choose another executor.

## Exact request binding

`plan_tool_execution` derives a domain-separated BLAKE3 identity from the complete current request. The identity covers argument names and values, correlation identity, output policy, approval posture, capability labels, and mutation targets.

`execute_admitted_tool` recomputes that identity before it calls the service. A request with changed values fails as `RequestBindingMismatch`, even if its tool, schema, and argument names still match.

Successful execution returns `ToolExecutionOutcome`. Its `ToolResponse` contains only bounded model-visible text. Its `ToolExecutionReceipt` contains counts, safe labels, BLAKE3 identities, and `raw-output-omitted`. The receipt contains no output body.

## Packaged smoke

Run the non-default deterministic smoke with explicit inputs:

```sh
cargo run -q -p animus-std \
  --features codebase-tool-host \
  --bin animus-codebase-tool-smoke -- \
  --root . \
  --path crates/animus-std/src/codebasetool/core.rs \
  --symbol decode_codebase_query \
  --kind definition \
  --max-results 2
```

The smoke prints metadata-only completion evidence. It does not print the root, relative file names, source text, or query value.

## Verification and non-claims

Run:

```sh
scripts/cairn/check-codebase-tool-host --root .
nix build .#checks.x86_64-linux.codebase-tool-host --no-link -L
```

This evidence proves one bounded syntactic query through the production executor kit. It does not prove semantic analysis, type correctness, complete reference resolution, arbitrary repository coverage, live-provider tool use, mutation safety, sandboxing, or production readiness.
