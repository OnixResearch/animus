# Design: Packaged live OpenRouter smoke

## Context

`animus-std-openai` owns an OpenAI Responses wire projection, bounded response normalization, and one-attempt HTTPS shell. Its configuration currently admits only the official OpenAI endpoint.

OpenRouter documents an OpenAI-compatible, stateless Responses endpoint at `https://openrouter.ai/api/v1/responses`. Pi stores an explicit OpenRouter API key and lists `openai/gpt-5.5` for that provider.

## Success Contract

The change is complete when a packaged binary sends one fixed text request to the exact OpenRouter Responses endpoint with the Pi OpenRouter key. A successful run must print only metadata and BLAKE3 identities.

Completion evidence requires positive and negative deterministic tests, exact endpoint-pair denial, focused Rust and Nix rails, Cairn gates, and one successful current-head live run.

An HTTP connection, a `2xx` response without canonical normalization, raw provider output, a stale run, or the existing OpenAI smoke is not completion evidence.

## Decisions

### 1. Add an exact endpoint cohort to the existing wire host

**Choice:** Admit the exact OpenRouter identity and URL pair in `OpenAiHostConfig`.

**Rationale:** OpenRouter uses the same Responses wire contract. A new transport crate would duplicate projection, limits, and safe error behavior.

### 2. Keep endpoint pairs closed

**Choice:** Admit only the official OpenAI pair and the exact OpenRouter pair. Reject crossed identities, crossed URLs, and arbitrary URLs.

**Rationale:** This prevents credential forwarding to an unreviewed endpoint and preserves explicit authority.

### 3. Add a separate packaged binary

**Choice:** Add `animus-live-openrouter-smoke` with its own provider identity, endpoint identity, credential label, status prefix, and tests.

**Rationale:** A separate binary keeps operator intent visible and avoids ambient provider selection.

### 4. Reuse the functional core

**Choice:** Share cohort-driven planning and metadata projection. Keep credential reading and HTTPS execution in thin shells.

**Rationale:** The provider-specific facts are explicit inputs to pure deterministic functions.

## Portfolio Search Record

The exact goal is a bounded live OpenRouter path using the Pi credential. The serial lenses are correlated because one agent performed them.

- **Exact second cohort in the existing Responses host**: selected. It reuses the compatible wire core and preserves closed endpoint admission.
- **Replace the official OpenAI endpoint**: falsified because it would regress the accepted OpenAI host.
- **Create a duplicate OpenRouter crate**: rejected because it duplicates transport and normalization authority.
- **Allow an arbitrary endpoint argument**: rejected because it permits credential forwarding to unreviewed hosts.
- **Use the OpenRouter key against OpenAI**: falsified by HTTP `401` and by provider authority boundaries.
- **Use raw `curl` as release evidence**: rejected because it bypasses canonical planning, bounded normalization, and metadata-only output.

The adversarial audit targets crossed endpoint pairs, credential leakage, raw response leakage, hidden retries, arbitrary URL admission, provider mislabeling, stale evidence, and OpenAI regressions.

The retrieval budget covers the accepted host, packaged smoke, OpenRouter Responses documentation, Pi model metadata, and focused lifecycle rails. The validation budget covers focused Rust, Clippy, Nix, Cairn, and one live request.

## Risks and Trade-offs

- OpenRouter can change its compatible response shape after deterministic checks pass.
- One model and prompt do not establish provider reliability or model quality.
- The shared crate name remains OpenAI-specific while its host admits one reviewed OpenAI-compatible endpoint.
