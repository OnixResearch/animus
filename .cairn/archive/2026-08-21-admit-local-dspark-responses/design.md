## Context

`animus-std-openai` already owns deterministic Responses request projection, exact endpoint admission, bounded transport, and production SSE parsing. The `local-dspark` server implements the same Responses and tool-stream event shapes at one private HTTP address. A live probe showed that default requests emit unsupported hidden-reasoning events. The same server omits those events when it receives explicit `reasoning.effort = "none"`.

## Decisions

### Decision: Extend the focused Responses host

**Choice:** Add one exact endpoint cohort to `animus-std-openai` instead of creating a parallel provider stack.

**Rationale:** The existing component owns the matching protocol, request projection, parser, bounds, credential seam, and retry boundary. Reusing it avoids duplicate provider authority.

### Decision: Bind cleartext transport to one exact private URL

**Choice:** Derive transport security from the already validated endpoint identity and URL pair. Keep HTTPS mandatory for OpenAI and OpenRouter. Permit HTTP only for `local-dspark-responses` at the reviewed private address.

**Rationale:** A generic HTTP switch or caller-supplied URL would weaken endpoint admission. An exact cohort keeps the exception closed.

### Decision: Disable reasoning explicitly for the local cohort

**Choice:** When the admitted local cohort receives canonical reasoning `none`, project `{"reasoning":{"effort":"none"}}`. Preserve the current omission rule for the other cohorts.

**Rationale:** The local server otherwise emits `response.reasoning_text.*`. Those events can contain hidden reasoning and must remain rejected by the common parser.

### Decision: Keep model selection outside endpoint configuration

**Choice:** The reusable host admits the endpoint pair. The packaged smoke and consuming product bind model `deepseek-v4-flash-0731-ablit-100` exactly.

**Rationale:** Endpoint admission and product model policy have different owners. The host must not gain model discovery or fallback authority.

## Risks / Trade-offs

- Cleartext traffic is visible on the private network. The exact endpoint restriction limits scope but does not provide transport confidentiality.
- The local server can change its event shape. The parser fails closed, and current-head live evidence is required for release.
- Disabling reasoning changes model behavior. This is required to preserve the existing hidden-reasoning boundary.
