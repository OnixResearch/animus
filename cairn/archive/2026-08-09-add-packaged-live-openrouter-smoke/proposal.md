# Proposal: Add packaged live OpenRouter smoke

## Why

The packaged live smoke accepts only the official OpenAI endpoint. Pi stores an OpenRouter API key, so that smoke cannot produce live evidence with the available explicit credential.

OpenRouter publishes an OpenAI-compatible Responses endpoint. Animus needs an exact, bounded OpenRouter cohort rather than endpoint substitution or an unreviewed generic URL.

## What Changes

- Admit the exact `https://openrouter.ai/api/v1/responses` endpoint as a second OpenAI-compatible Responses cohort.
- Add a non-default `animus-live-openrouter-smoke` binary.
- Reuse the existing pure request projection, bounded response normalization, and one-attempt HTTPS shell.
- Require an exact model argument and a bounded API key through standard input.
- Emit metadata-only evidence that omits the prompt, API key, model output, and raw response.
- Add positive and negative endpoint-pair, planning, input, and leak tests.
- Run the live smoke with the OpenRouter key from Pi only after the indexed revision is clean.

## Impact

- **Crate**: `animus-std-openai` gains one exact OpenRouter endpoint cohort and a packaged binary.
- **Compatibility**: the official OpenAI endpoint remains admitted without behavior changes.
- **Evidence**: a successful current-head OpenRouter run can support the existing bounded `live-providers` claim.
- **Non-claims**: no OpenRouter reliability, model quality, routing choice, billing, production, retry, fallback, streaming, or credential-storage claim.
