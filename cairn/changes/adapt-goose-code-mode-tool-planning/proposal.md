# Proposal: Adapt Goose Code Mode as typed tool planning

## Why

Goose Code Mode reduces tool-context pressure by exposing a small set of meta-tools that discover tool definitions on demand and execute batches of tool calls through generated code. Animus already has typed tool contracts, Steel loop policy, tool catalog descriptors, and runtime revalidation. It should adapt the useful idea of progressive tool discovery and batched tool planning without adopting arbitrary JavaScript, Deno execution, or unchecked tool dispatch.

Animus should model Code Mode as a typed tool-plan fabric: scripts or model decisions produce declarative tool plans, and Rust validates every discovered tool and every planned step before host adapters run.

## What Changes

- Define tool discovery, tool summary, tool-detail, tool-plan, plan-step, intermediate binding, batch policy, and tool-plan receipt DTOs.
- Allow an optional Steel-backed or typed-fabric planner to request progressive tool metadata and return a declarative batch plan.
- Revalidate each step against catalog identity, capability labels, permission policy, argument schema, output bounds, mutation windows, cancellation, and timeout policy before adapter dispatch.
- Reject arbitrary code execution, ambient tool discovery, unbounded intermediate data, binary-only result chaining, and fallback to tools omitted from the admitted discovery set.

## Impact

- **Files**: future runtime/tool-planning DTOs, Steel policy descriptors, catalog projection helpers, docs, tests, and Cairn artifacts.
- **Testing**: positive on-demand discovery and batch planning tests plus negative undeclared tool, denied capability, unsafe intermediate, unsupported binary result, arbitrary-code, and receipt leak fixtures.
- **Non-claims**: this does not adopt Goose's JavaScript Code Mode, Deno/pctx, dynamic package loading, automatic MCP execution, browser sandboxing, or product UI behavior.
