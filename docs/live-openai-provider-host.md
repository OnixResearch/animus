# Live OpenAI-compatible provider host

`animus-std-openai` is a non-default host for reviewed OpenAI-compatible Responses endpoints. It requires explicit network and credential authority.

The root text-only feature is `openai-provider-host`. The tool-capable feature is `openai-tool-provider-host`. Direct users can enable `reqwest-client` or `tool-streaming` on `animus-std-openai`.

## Endpoint cohorts

The host admits three exact endpoint pairs:

- `openai-responses` with `https://api.openai.com/v1/responses`;
- `openrouter-responses` with `https://openrouter.ai/api/v1/responses`; and
- `local-dspark-responses` with the reviewed private HTTP Responses URL.

Crossed pairs and arbitrary URLs fail before credential use or network contact. HTTPS remains mandatory for OpenAI and OpenRouter. Cleartext HTTP is allowed only for the exact private local-dspark pair.

The local cohort must use model `deepseek-v4-flash-0731-ablit-100` at the product or packaged-smoke boundary. Its request projection sends explicit `reasoning.effort = "none"`. Without this field, the server emits hidden-reasoning events that the common parser rejects.

The text-only cohort supports non-streaming text and refusal responses. It supports system, developer, user, and assistant messages. It also supports bounded usage, reasoning effort, output-token controls, temperature controls, and safe metadata.

The text-only cohort rejects images, tools, structured output, stop sequences, and streaming.

## Tool-capable cohort

The separate `tool-streaming` cohort supports one exact function:

- name: `steel_repl`;
- schema label: `animus.steel-repl.v1`; and
- schema shape: a strict JSON object with one required string field named `code`.

These identities come from `animus-adapter-rlm-kernel`. The provider adapter does not execute the function. An authority-scoped shell must admit and execute the returned tool request.

The cohort supports these canonical roles:

- `user` for the initial request; and
- `tool` for one correlated continuation result.

The selected model must declare `steel_repl`, the exact schema label, and all required stream channels. The registry validator rejects tool support without these facts.

The host emits `parallel_tool_calls: false`. Tool choice can be `auto`, `required`, or the exact declared function. The host rejects all other tools and schema labels before contact.

The host preserves one continuation with:

- the Animus model-call identity;
- the provider response identity;
- the provider output-item identity;
- the provider function-call identity;
- the exact tool name; and
- the BLAKE3 identity of the completed arguments.

The continuation request uses `previous_response_id` and one matching `function_call_output`. It does not replay or invent provider-native state.

## Streaming cohort and provenance

`OpenAiResponsesSseParser` is the one production parser for literal fixtures, fake transports, and live HTTPS streams. It parses bounded JSON SSE records from the reviewed OpenAI Responses event cohort.

The reviewed event definitions came from the OpenAI OpenAPI document and the matching `openai-python` event classes. The checked fixtures use the reviewed field names and event lifecycle:

- `response.created` and `response.in_progress`;
- `response.output_item.added` and `response.output_item.done`;
- `response.content_part.added` and `response.content_part.done`;
- `response.output_text.delta` and `response.output_text.done`;
- `response.reasoning_summary_part.added` and `response.reasoning_summary_part.done`;
- `response.reasoning_summary_text.delta` and `response.reasoning_summary_text.done`;
- `response.function_call_arguments.delta` and `response.function_call_arguments.done`;
- `response.completed`, `response.failed`, and `response.incomplete`; and
- `error`.

The parser accepts named SSE records and data-only records with a required JSON event type.
An explicit SSE event name must match that type. Empty or duplicate fields remain errors.
One unnamed `[DONE]` marker can follow an already validated completed response.
The marker creates no canonical frame and cannot replace completion evidence.
Premature markers, repeated markers, and later records fail. Markers consume the existing byte and event bounds.
Comment-only SSE keep-alive records are ignored. Each record consumes the configured event and byte bounds. These parser bounds do not establish a wall-clock deadline.

The parser binds every accepted event to one response and output-item lifecycle. It also checks sequence numbers, tool-call identities, completed arguments, terminal state, and configured bounds.

`response.reasoning_text.delta` and `.done` events for registered reasoning items are accepted like reasoning summary events: they produce redacted reasoning frames only, and each reasoning delta must carry a `summary_index` or `content_index` correlation field. Content parts select their item kind from the part type, so `reasoning_text` parts validate against reasoning items. Hidden reasoning events still fail closed.

Visible output becomes canonical content frames. Function arguments become redacted tool-call frames. Usage and terminal events become redacted canonical frames. Provider response identities remain host-local.

The parser projects its accepted frames into the existing runtime retry stream state. Any accepted semantic frame blocks unsafe replay under the current retry policy.

## Authority and execution

`build_openai_request_body`, `build_tool_capable_openai_request_body`, `normalize_openai_response`, and the SSE parser are deterministic over explicit inputs. They do not read the environment or contact a network.

`ReqwestOpenAiHost` owns transport. It uses explicit configuration, credentials, dispatch plans, cancellation, and byte bounds. The client disables redirects, ambient proxies, discovery, fallback, and hidden retry. It permits cleartext transport only after exact local-dspark pair validation.

Both transport paths set an explicit request timeout. It applies from connection start through response-body completion, including a body with regular keep-alive bytes.
The blocking client timeout remains an additional operation limit. That client-level setting alone restarts its wait on each read and does not bound total request time.
The request timeout excludes credential resolution, request projection, worker lifetime, and cleanup. It does not establish a complete process deadline.

Private transport tests cover completed bodies and deadline rejection under regular body activity for both text and streaming paths. They use synthetic credentials and a loopback server. This test-only wire seam does not widen public endpoint admission.

Private TLS tests use the same internal client builder and both production request paths.
The fixture alone replaces the trust roots with an ephemeral synthetic certificate and resolves the requested hostname directly to loopback.
Positive controls accept the synthetic trust anchor. Negative controls reject an untrusted certificate and a trusted certificate with the wrong hostname before an HTTP request arrives.
Stalled handshakes and active TLS response bodies must respect the configured request timeout on both transport paths.
The public constructor retains bundled trust roots, certificate and hostname verification, and the exact endpoint allowlist. It exposes no new root or resolver override.
These controls do not contact OpenAI or OpenRouter. They do not prove public certificate validity, external DNS, provider availability, or a complete process deadline.

The streaming transport is behind an injected reader seam. Deterministic tests use the same request admission and parser loop as the live client.

The response reader checks cancellation between reads. A cancellation before provider response creation produces a canonical cancelled outcome without a provider response identity.

## Z.ai chat-completions status

The separate chat parser admits the exact Coding Plan endpoint at `https://api.z.ai/api/coding/paas/v4/chat/completions`.
It does not use OpenRouter or the Z.ai pay-as-you-go endpoint.

The chat parser requires a finish reason and a complete `[DONE]` record before it accepts a provider outcome.
It processes deltas in the final choice before completion. A parse error permanently blocks further input, cancellation, and successful completion.
The parser rejects records after `[DONE]`, malformed tool-argument JSON, non-object arguments, duplicate tool identities, and unsupported choice indices.
An explicit model field must match the admitted model. Optional absent model fields do not prove provider model identity.
Cancellation before a response identity produces a cancelled outcome with an empty provider identity.
These controls use synthetic streams and do not prove live Coding Plan behavior.

Chat continuation retains the admitted original messages, completed assistant messages, exact reasoning text, tool arguments, and correlated tool results.
The host binds this private history to the model, provider, endpoint, credential declaration, reasoning effort, and tool-schema identity.
It sends `thinking.clear_thinking = false` and an explicit thinking mode. Unset temperature is omitted.
Canonical content, thought, and tool deltas preserve whitespace. They are not identifier labels.

History is limited to 64 messages and 1,048,576 serialized bytes. A request is limited to 2,097,152 encoded bytes.
These are local memory and encoding limits, not approved campaign budgets.
Excess history is rejected, never trimmed. Parallel calls and reused tool-call identities are rejected.
Debug output omits private history. This state is not a durable session-recovery format.

`OpenAiChatCompletionsSseParser::for_request` binds an admitted request for offline or injected stream acquisition.
The production transport prepares the same request and replay state before contact.
The standalone `new` constructor has no original request. Its completed calls cannot authorize chat continuation.
Responses and chat continuation bindings cannot cross protocol families.

Offline tests do not prove subscription access, live model behavior, or release readiness.
The all-feature build does not prove feature isolation. The `reqwest-client` feature also requires a separate build without `tool-streaming`.

## Packaged live smokes

The existing packaged smokes remain text-only. Run one only from the clean revision that will own the evidence.

For the official OpenAI endpoint:

```sh
model_id='REVIEWED_MODEL_ID'
cargo run -q -p animus-std-openai \
  --features reqwest-client \
  --bin animus-live-openai-smoke -- \
  --model "${model_id}" \
  --credential-stdin < /run/credentials/openai-api-key
```

For the OpenRouter endpoint:

```sh
model_id='deepseek/deepseek-v4-flash-0731'
cargo run -q -p animus-std-openai \
  --features reqwest-client \
  --bin animus-live-openrouter-smoke -- \
  --model "${model_id}" \
  --credential-stdin < /run/credentials/openrouter-api-key
```

For the exact private local-dspark endpoint:

```sh
model_id='deepseek-v4-flash-0731-ablit-100'
printf '%s\n' 'local' | cargo run -q -p animus-std-openai \
  --features reqwest-client \
  --bin animus-live-local-dspark-smoke -- \
  --model "${model_id}" \
  --credential-stdin
```

The binaries read a bounded API key from standard input. Their evidence omits prompts, credentials, model output, reasoning, tool arguments, tool output, and raw responses.

## Validation

Run these deterministic rails:

```sh
scripts/config/check-provider-registry
scripts/cairn/check-live-openai-provider-host --root .
scripts/cairn/check-packaged-live-openai-smoke --root .
scripts/cairn/check-packaged-live-openrouter-smoke --root .
```

The focused host rail runs tool-streaming tests and Clippy. The literal fixtures cover an initial call, one `steel_repl` result, and a prior-response continuation.

## Evidence and non-claims

Deterministic tests prove bounded admission and parsing for the checked fixture cohort. They prove exact tool and continuation correlation through the fake transport seam.

Without a separately run current-head live smoke, live availability remains a non-claim.

This host does not prove provider reliability, model quality, billing correctness, remote cancellation, memory erasure, hidden-reasoning absence, exactly-once tool execution, tool safety, retry safety outside the checked policy, or production readiness. It does not discover providers, choose fallback targets, read ambient credentials, or execute tools.
