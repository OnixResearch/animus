# Live OpenAI-compatible provider host

`animus-std-openai` is a non-default host for reviewed OpenAI-compatible Responses HTTPS endpoints. It requires explicit network and credential authority.

The root feature is `openai-provider-host`. New embedders can depend on `animus-std-openai` directly and enable `reqwest-client`.

## Supported cohorts

The host admits two exact endpoint pairs. OpenRouter documents its cohort in the [Responses API overview](https://openrouter.ai/docs/api/reference/responses/overview).

- `openai-responses` with `https://api.openai.com/v1/responses`; and
- `openrouter-responses` with `https://openrouter.ai/api/v1/responses`.

Crossed pairs and arbitrary URLs fail before credential use or network contact.

Both cohorts support:

- text-only system, developer, user, and assistant messages;
- non-streaming responses;
- optional reasoning effort;
- optional output-token and temperature controls;
- safe metadata; and
- completed text or refusal responses with bounded usage data.

The host rejects images, tools, tool correlation, structured output, stop sequences, streaming, endpoint substitution, and unsupported response items before data loss can occur.

Provider-qualified model identities, such as `openai/gpt-5.5`, are admitted only as model identifiers. The slash does not become valid in provider or credential identifiers.

## Authority and execution

`build_openai_request_body` and `normalize_openai_response` are pure over explicit plans and bytes. They do not read the environment or contact a network.

`ReqwestOpenAiHost` performs one HTTPS POST. It uses an explicit `OpenAiHostConfig`, `OpenAiApiKey`, and `CancellationState`.

The client disables redirects and ambient proxy configuration. It does not discover providers, models, endpoints, or credentials. It does not retry or fall back.

The response reader stops after the configured byte limit plus one byte. Status and parse errors contain no provider body or API key.

## Example

```rust
use std::time::Duration;
use animus_std_openai::{
    CancellationState, OpenAiApiKey, OpenAiHostConfig, ReqwestOpenAiHost,
    OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY, OFFICIAL_OPENAI_RESPONSES_URL,
};

const REQUEST_TIMEOUT_SECONDS: u64 = 60;
const RESPONSE_LIMIT_BYTES: u64 = 262_144;

let config = OpenAiHostConfig::new(
    OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    OFFICIAL_OPENAI_RESPONSES_URL,
    Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
    RESPONSE_LIMIT_BYTES,
)?;
let host = ReqwestOpenAiHost::new(config)?;
let key = OpenAiApiKey::new("env:ANIMUS_OPENAI_LIVE_API_KEY", api_key_value)?;
let outcome = host.execute(&dispatch_plan, &key, CancellationState::Active)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

The caller must construct `dispatch_plan` through `animus_std::plan_provider_dispatch`. The plan binds the selected provider, model, endpoint identity, request, credential source, deadline, cancellation label, and receipt policy.

## Packaged live smokes

Run a packaged smoke only from the clean revision that will own the evidence.

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
model_id='openai/gpt-5.5'
cargo run -q -p animus-std-openai \
  --features reqwest-client \
  --bin animus-live-openrouter-smoke -- \
  --model "${model_id}" \
  --credential-stdin < /run/credentials/openrouter-api-key
```

Each binary reads a bounded API key from standard input. Neither binary reads credential or model environment variables.

Each binary prints provider, endpoint, status, counts, and BLAKE3 identities. It omits the prompt, API key, model output, and raw response.

After a successful run, generate a current-head index:

```sh
scripts/cairn/generate-current-head-release-evidence-index \
  --check live-providers=pass \
  --claim live-providers \
  --out target/cairn/release-evidence/live-openai.lines
scripts/cairn/check-release-evidence-index \
  --index target/cairn/release-evidence/live-openai.lines
```

The validator rejects a live-provider claim from a dirty head or a stale optional receipt.

## Validation

Run the deterministic focused rails:

```sh
scripts/cairn/check-live-openai-provider-host --root .
scripts/cairn/check-packaged-live-openai-smoke --root .
scripts/cairn/check-packaged-live-openrouter-smoke --root .
```

The ignored integration test remains available for compatibility with explicit environment-based test execution.

Do not store the API key in source, command history, receipts, or test output.

## Evidence and non-claims

Deterministic tests prove request admission, exact JSON projection, fake transport execution, bounded response parsing, cancellation, safe errors, and dependency isolation.

A packaged binary does not prove live behavior until an operator runs it with explicit credentials. Without that result, live availability remains a non-claim.

This host does not prove OpenAI or OpenRouter reliability, routing choice, model quality, billing correctness, remote cancellation, memory erasure, production readiness, streaming, tool use, image input, or structured output.
