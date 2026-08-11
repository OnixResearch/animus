# MCP client host

Animus provides one non-default MCP client host in `animus-std-mcp`. The host supports only newline-delimited JSON-RPC over standard input and output. It implements the reviewed MCP `2025-06-18` cohort.

The reviewed specification revision is `b25c0874bf0ba699a58e21ef06f659d839659de3`. The schema BLAKE3 is `528797f6970a8f24175daf1f994c742f2fe24f0b8230b9456885452a08132a90`. The license BLAKE3 is `fd12707e19e104da035c74958a676b0567b7dc2df01875ae07f058b79e59422f`.

The host does not support Streamable HTTP, SSE, remote catalogs, transport fallback, shell fallback, ambient server discovery, or inherited environment variables.

## Configuration

`config/mcp-client-host.ncl` is the source configuration. Its Nickel contract requires:

- one exact server identity;
- required or optional startup posture;
- one caller-authorized executable identity;
- direct process arguments;
- explicit environment-key and credential references;
- one caller-owned working-root identity;
- the exact protocol cohort and specification hashes;
- finite frame, catalog, schema, result, diagnostic, deadline, and call limits;
- exact tool policies; and
- all mandatory non-claims.

`scripts/config/export-nickel-configs` creates `config/generated/mcp-client-host.json`. Rust parses and validates this file again before it starts a process.

## Authority and lifecycle

The caller supplies the executable path, working directory, and operation-scoped credential resolver. The host clears the child environment. It starts the executable directly without a shell.

The pure lifecycle uses these states:

```text
configured -> starting -> initializing -> discovering -> ready
ready -> calling -> ready
calling -> cancelling -> stopping -> stopped
any live state -> stopping -> stopped
teardown uncertainty -> failed with restart blocked
```

Each state binds the server, configuration, protocol, policy, credential snapshot, process attempt, and generation. A credential identity change stops the old generation. The caller can start a new generation only after terminal old-generation evidence.

Required server failure blocks required readiness. Optional server failure stays visible and does not select another server.

## Discovery and calls

Discovery must return one complete finite catalog. A non-null pagination cursor, duplicate name, unknown tool, malformed schema, schema drift, or budget excess rejects the complete catalog. No partial catalog becomes model-visible.

Each call must pass the existing Animus MCP bridge and these current bindings:

1. mapped tool and schema;
2. model invocation plan;
3. runtime authority seal;
4. approval;
5. parent and caller capability intersection;
6. mutation plan;
7. cancellation observation; and
8. output bound.

The host resolves the credential snapshot again before dispatch. Stale authority or credentials stop the effect before transport dispatch.

## Instructions

Server instructions are untrusted input. The host admits them only through the request-context contribution contract with these fixed properties:

- source: `Mcp`;
- trust: `Untrusted`;
- posture: `Advisory`;
- lifetime: `CurrentRequest`; and
- sensitivity: `RequestLocal`.

The host does not store instructions in conversations, compaction, memory, quality records, or receipts.

## Cancellation and teardown

The host observes cancellation at a named interval. Cancellation or a call deadline sends the MCP cancellation notification when possible. It then tears down the owned process.

On Unix, the host creates and terminates an owned process group. Other platforms use direct-child termination and report only that narrower scope. Teardown uncertainty blocks same-server restart. This mechanism does not prove that no external process exists.

## Evidence and non-claims

Receipts contain metadata and BLAKE3 identities only. Receipt projection rejects raw frames, arguments, results, instructions, credentials, environment values, private paths, provider data, unrestricted errors, and unsupported claims.

The host does not prove server trust, server correctness, tool safety, sandboxing, network isolation, exactly-once effects, full MCP parity, production readiness, or release eligibility.

Run the focused rail:

```sh
nix develop -c scripts/cairn/check-concrete-mcp-client-host --root .
```

Run the packaged local smoke after both binaries are built:

```sh
cargo build -p animus-std-mcp --bins
"${CARGO_TARGET_DIR:-$HOME/.cargo-target}/debug/animus-mcp-client-smoke"
```

The smoke uses the local fixture server. It uses no network and no live credential.
