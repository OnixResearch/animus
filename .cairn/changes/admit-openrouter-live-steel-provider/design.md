## Context

`animus-std-openai` already owns exact OpenAI-compatible endpoint admission and HTTPS transport. `animus-live-steel-agent` composes that host, but its configuration parser hard-codes the OpenAI identity and credential label.

The product shell needs exact OpenRouter selection without adding a second transport, provider discovery, fallback, OAuth, or ambient credential authority.

## Decisions

### Decision: Extend the existing product configuration cohort

**Choice:** Add one pure cohort selector inside `animus-live-steel-agent`. It maps `openai` and `openrouter` to their exact endpoint identity, URL, and injected credential reference.

**Rationale:** The live shell remains the visible composition root. The existing `ReqwestOpenAiHost` remains the only network adapter.

### Decision: Reject partial or crossed matches

**Choice:** Admit a provider only when its provider identity, endpoint identity, endpoint URL, and credential reference match one reviewed tuple.

**Rationale:** Independent field allowlists could admit a crossed provider and endpoint pair.

### Decision: Keep credentials injected

**Choice:** Continue to accept the credential as an explicit input after configuration admission. Configuration stores only the reviewed reference label.

**Rationale:** This preserves the existing redaction and authority boundary. SecretSpec composition can inject the value without changing provider transport.

## Risks / Trade-offs

- The live shell supports only two exact provider cohorts.
- OpenRouter model behavior remains unproved without a credentialed current-revision smoke.
- This change does not add provider discovery, fallback, OAuth, or account login.
