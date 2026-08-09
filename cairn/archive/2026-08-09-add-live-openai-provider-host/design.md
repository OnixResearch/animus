# Design: Live OpenAI provider host

## Context

`animus-std::providers::concrete` owns a pure OpenAI Responses dispatch plan and safe fixture normalization. It does not create a network client or contact a provider.

The dogfood roadmap lists provider execution as an actionable blocker. The first host must remain narrow because the current canonical plan does not carry provider-ready image bytes, tool schemas, or structured-output schemas.

## Success Contract

The change is complete when an explicit OpenAI plan can cross a bounded HTTPS shell and return a canonical outcome. Deterministic tests must prove all pre-contact and post-contact rejection paths.

A compiled client, a passing fake, an ignored test, or a task marker alone does not prove live OpenAI behavior. Live evidence requires an explicit manual run and a separately indexed safe result.

## Decisions

### 1. Use a separate authority-scoped crate

**Choice:** Add `animus-std-openai` outside workspace default members. The root `openai-provider-host` feature enables its optional `reqwest` shell.

**Rationale:** Adding HTTPS and credential authority to `animus-std` would widen the default shell. A transport-only seam would not close the concrete execution gap.

### 2. Preserve a functional core and thin shell

**Choice:** Pure functions validate the admitted plan, build exact JSON bytes, classify status, parse bounded response bytes, and project canonical drafts. The shell only sends HTTPS and reads a bounded body.

**Rationale:** Request and response logic stays deterministic. Tests do not require network access or credentials.

### 3. Support one minimal OpenAI Responses cohort

**Choice:** The first live cohort supports text messages, optional reasoning effort, optional output limits, safe metadata, and non-streaming text responses.

**Rationale:** Images, tools, streaming, and structured output need data that the current plan does not carry safely. The host rejects them before contact.

### 4. Require exact explicit authority

**Choice:** The host requires an exact endpoint identity, the official HTTPS URL, an explicit API key value, a named timeout, a response-byte limit, and a cancellation observation.

**Rationale:** The host must not discover endpoints, credentials, providers, models, retries, or fallbacks from ambient state.

### 5. Keep live smoke optional and non-authoritative by default

**Choice:** Add an ignored live test that runs only with exact opt-in environment values. The focused rail compiles the test but does not run it.

**Rationale:** Default checks stay deterministic. An absent credential does not become a false failure or a false live-provider claim.

## Portfolio Search Record

- **Separate host crate**: selected because it contains network and credential authority.
- **Feature inside `animus-std`**: rejected because it widens the broad std package graph.
- **Injected transport only**: rejected because existing code already stops before real transport.
- **Provider SDK**: rejected because it can hide retries, fallback, and request-shape changes.

The serial architecture lenses are correlated because one agent performed them. The adversarial audit will target secret leaks, unbounded reads, unsupported request loss, endpoint substitution, hidden retries, and false live claims.

## Risks / Trade-offs

- The first cohort is intentionally smaller than the fixture projection surface.
- `reqwest` adds an optional dependency graph, but default and no-default root graphs remain free of it.
- OpenAI wire changes can break live behavior after deterministic tests pass. Live evidence remains separately scoped and dated.
