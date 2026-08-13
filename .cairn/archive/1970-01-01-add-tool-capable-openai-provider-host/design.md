# Design: Tool-capable OpenAI provider host

## Context

`animus-std-openai` admits text-only OpenAI and OpenRouter Responses requests. Its shell performs one bounded blocking HTTPS request and normalizes text output.

The canonical model contracts already represent tool declarations, tool calls, stream channels, reasoning, usage, and finish reasons. The new host must project this existing meaning without inventing provider policy.

## Success Contract

One exact OpenAI Responses cohort can stream an assistant response that requests `steel_repl`. The adapter returns a canonical correlated tool call.

A later request can submit the matching tool result and continue the same provider response chain. The stream parser emits ordered canonical frames.

Unsupported features fail before contact. Malformed streams, duplicate terminal frames, wrong tool identities, and unsafe reasoning projections fail closed.

A fake transport alone does not prove live behavior. A generic unreviewed endpoint does not satisfy the cohort.

## Decisions

### 1. Extend the authority-scoped OpenAI crate

The implementation stays in `animus-std-openai` behind a separate non-default feature. Reusable model contracts and runtime cores gain no network or credential authority.

### 2. Support one exact tool cohort

The first cohort supports:

- canonical text messages
- one or more canonical function declarations
- `steel_repl` tool calls with JSON arguments
- correlated function-call outputs
- visible content streaming
- separately classified provider reasoning summaries
- usage and finish frames

Images and structured output remain rejected unless a later change adds exact fixture coverage.

### 3. Use one production parser entrypoint

The SSE parser consumes bounded admitted byte chunks. Literal checked-in provider fixtures and the HTTPS shell use the same parser.

The parser is a deterministic core. The shell owns socket reads, deadlines, cancellation observations, and body bounds.

### 4. Preserve two correlation layers

Every frame binds the Animus model-call identity. Tool frames also bind the provider response and provider tool-call identities.

The continuation request must match all identities and the committed assistant tool-call message. Wrong, stale, duplicate, or missing identities fail before contact.

### 5. Keep reasoning separate from assistant text

Provider reasoning or summaries map to the reasoning stream channel. Default logs, receipts, transcripts, and final answer text do not receive raw reasoning.

### 6. Keep retry outside the parser

The host reports typed failure classes and semantic-stream progress. Existing retry policy decides whether another attempt is legal.

The host does not retry, fall back, or switch endpoints by itself.

## Authority Flow

```text
canonical model request
  -> capability negotiation
  -> pure OpenAI projection
  -> explicit credential and endpoint shell
  -> bounded SSE bytes
  -> pure stream parser
  -> canonical ordered frames
  -> coding-session tool loop
```

## Alternatives Considered

### Add tools to the existing non-streaming response parser only

Rejected because the interactive shell and TUI need ordered progress, cancellation, usage, and partial-stream failure facts.

### Use an OpenAI SDK

Rejected for the first cohort because an SDK can hide retries, defaults, and wire-shape changes.

### Treat tool calls as assistant text

Rejected because this loses correlation and can bypass tool admission.

## Risks and Controls

- Provider wire events can drift. Literal production-parser fixtures pin the reviewed cohort.
- Partial streams can make retry unsafe. Semantic progress and terminal state remain explicit.
- Tool arguments can be large or malformed. Named byte and structural bounds apply before canonical projection.
- Reasoning can leak. Separate channel policy and leak fixtures fail closed.

## Non-Claims

This design does not establish provider correctness, arbitrary endpoint compatibility, hidden reasoning safety, tool safety, task success, live reliability, broad provider parity, or production readiness.
