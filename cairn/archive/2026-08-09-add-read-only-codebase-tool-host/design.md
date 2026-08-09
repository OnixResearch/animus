## Context

The reusable executor kit proves metadata admission and safe receipt projection. Its service receives only a plan, so argument values are not available or bound at the effect boundary. The kit also returns only a receipt after bounding output. The existing non-default codebase graph host already owns explicit capability-root reads, no-follow file admission, Rust parsing, freshness, and bounded syntactic queries.

## Decisions

### Decision: Bind request values before executor effects

**Choice:** Add a domain-separated BLAKE3 request identity to each pure execution plan. Pass the original request to `execute_admitted_tool` and the executor service, and reject any identity mismatch before the service call.

**Rationale:** Argument names alone cannot authorize a concrete effect. A private identity binds values without adding raw arguments to durable receipts or plan debug output.

### Decision: Return bounded output and receipt as one outcome

**Choice:** Return a `ToolExecutionOutcome` containing the existing bounded `ToolResponse` and metadata-only `ToolExecutionReceipt`.

**Rationale:** Concrete tool output must reach the model through the existing bounded adapter DTO. Durable evidence must continue to omit the body.

### Decision: Adapt the existing codebase graph host

**Choice:** Add a feature-gated `codebase_tool` module in `animus-std`. It owns one exact `codebase_graph_query` definition, pure request decoding and rendering, and a thin executor over a caller-built `CodeGraphHost`.

**Rationale:** Reusing the reviewed graph host avoids a second filesystem walker and retains explicit capability-root, path, parser, freshness, and budget checks.

### Decision: Keep the first executor read-only and deterministic

**Choice:** Admit exact `symbol`, `kind`, and `max_results` arguments. Return sorted relative-path and byte-span facts only. Add no mutation, process, network, environment, current-directory, provider, or plugin behavior.

**Rationale:** This is the weakest concrete tool authority that closes the roadmap blocker and can be tested without external services.

## Risks / Trade-offs

- Adding exact request binding changes the public executor-service method and execution return type. All in-repository implementations and tests must migrate together.
- Syntactic graph results are not semantic analysis, source truth, authorization, or release evidence.
- The packaged smoke proves one explicit local fixture path. It does not prove arbitrary repository coverage or live-provider tool use.
