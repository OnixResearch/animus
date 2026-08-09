# Live OpenAI provider host

`animus-std-openai` is a non-default host for one OpenAI Responses HTTPS endpoint. It requires explicit network and credential authority.

The root feature is `openai-provider-host`. New embedders can depend on `animus-std-openai` directly and enable `reqwest-client`.

## Supported cohort

The first cohort supports:

- the exact `openai-responses` endpoint identity;
- `https://api.openai.com/v1/responses`;
- text-only system, developer, user, and assistant messages;
- non-streaming responses;
- optional reasoning effort;
- optional output-token and temperature controls;
- safe metadata; and
- completed text or refusal responses with bounded usage data.

The host rejects images, tools, tool correlation, structured output, stop sequences, streaming, endpoint substitution, and unsupported response items before data loss can occur.

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

## Validation

Run the deterministic focused rail:

```sh
scripts/cairn/check-live-openai-provider-host --root .
```

Compile and run the ignored live smoke only with explicit values:

```sh
ANIMUS_OPENAI_LIVE_API_KEY='...' \
ANIMUS_OPENAI_LIVE_MODEL='...' \
cargo test -p animus-std-openai \
  --features reqwest-client \
  --test live-openai \
  -- --ignored --exact explicit_live_openai_smoke
```

Do not store the API key in source, command history, receipts, or test output.

## Evidence and non-claims

Deterministic tests prove request admission, exact JSON projection, fake transport execution, bounded response parsing, cancellation, safe errors, and dependency isolation.

The ignored test does not prove live behavior until a maintainer runs it with explicit credentials. Without that result, live availability remains a non-claim.

This host does not prove OpenAI reliability, model quality, billing correctness, remote cancellation, memory erasure, production readiness, streaming, tool use, image input, or structured output.
