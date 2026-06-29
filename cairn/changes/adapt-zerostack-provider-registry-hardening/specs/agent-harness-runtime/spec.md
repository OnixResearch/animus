# agent-harness-runtime Change Spec: Zerostack provider registry hardening adaptation

## Requirements

### Requirement: Explicit OpenAI-family API style

r[agent_harness_runtime.zerostack_provider_hardening.api_style] Animus provider registry shells MUST resolve OpenAI-family providers to explicit API style metadata such as Responses or Chat Completions before constructing a model service.

#### Scenario: Custom gateway defaults safely

r[agent_harness_runtime.zerostack_provider_hardening.api_style.gateway]
- GIVEN a custom OpenAI-compatible provider declares a base endpoint and no explicit API style
- WHEN shell provider resolution runs
- THEN the resolver selects the policy-defined compatible default or reports an ambiguous-style diagnostic
- AND reusable runtime receives only a validated injected model service.

### Requirement: Redacted custom headers

r[agent_harness_runtime.zerostack_provider_hardening.headers] Custom provider header descriptors MUST validate header names, resolve allowed environment references in the shell, and report only redacted source labels or failure classes in receipts and diagnostics.

#### Scenario: Missing header environment variable is safe

r[agent_harness_runtime.zerostack_provider_hardening.headers.missing]
- GIVEN a custom header references an unavailable environment-backed secret
- WHEN provider resolution runs
- THEN resolution fails with a missing-header-secret diagnostic
- AND the diagnostic does not include raw secret values, environment contents, or unrelated environment variables.

### Requirement: Provider-scoped extra body policy

r[agent_harness_runtime.zerostack_provider_hardening.extra_body] Provider-specific extra request body fields MUST be admitted by policy with provider/model scope, source identity, collision behavior, and receipt redaction before they can be sent with a model request.

#### Scenario: Extra body cannot cross providers silently

r[agent_harness_runtime.zerostack_provider_hardening.extra_body.scope]
- GIVEN an extra body descriptor is scoped to one provider or model
- WHEN a different provider or model is selected
- THEN the descriptor is ignored or rejected according to policy
- AND the request does not inherit unsupported provider-specific fields silently.

### Requirement: Agent-suitable model filtering

r[agent_harness_runtime.zerostack_provider_hardening.model_filter] Dynamic model listings SHOULD normalize model IDs, display names, model classes, context budgets, modalities, reasoning support, and cost hints, and MUST prevent non-agent model classes from being selected for text agent turns unless an explicit host override is present.

#### Scenario: Embedding model is rejected for agent turn

r[agent_harness_runtime.zerostack_provider_hardening.model_filter.reject]
- GIVEN model listing metadata identifies a model as embedding, image, audio, moderation, rerank, or another non-agent class
- WHEN the shell attempts to select it for a text agent turn without override
- THEN selection fails with a typed unsupported-model-class diagnostic
- AND runtime model dispatch is not attempted.

### Requirement: Provider selection receipts

r[agent_harness_runtime.zerostack_provider_hardening.receipts] Provider selection receipts MUST record safe provider ID, model ID, endpoint identity class, API style, capability metadata, credential source class, extra body descriptor IDs, and selection diagnostics without exposing raw credentials, headers, provider payloads, or full private endpoints when policy marks them sensitive.

#### Scenario: Sensitive endpoint is hashed

r[agent_harness_runtime.zerostack_provider_hardening.receipts.endpoint]
- GIVEN a provider endpoint is classified as sensitive
- WHEN a provider selection receipt is emitted
- THEN the endpoint is represented by a redaction marker or BLAKE3 hash
- AND API style, capability class, and diagnostic reason remain reviewable.
