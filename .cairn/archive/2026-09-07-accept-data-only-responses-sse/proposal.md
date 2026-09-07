## Why

OpenRouter returns Responses events as data-only SSE records with JSON event types and an optional trailing `[DONE]` marker.
The current parser rejects the first record before it can observe a completed GLM answer.
A retained live capture and an offline framing transformation reproduce and isolate the incompatibility.

## What Changes

- Admit data-only records through the same typed event validation as named records.
- Preserve explicit event-name agreement, sequence, identity, lifecycle, reasoning, and byte/event bounds.
- Accept one unnamed `[DONE]` marker only after an already validated completed response.
- Add positive and negative parser controls without expanding tool authority or completion meaning.

## Impact

- **Files**: `crates/animus-std-openai/src/stream.rs`, focused stream tests, and `docs/live-openai-provider-host.md`.
- **Testing**: focused baseline and regression tests, Clippy, pinned strict Octet, affected Nix checks, and product checks after the immutable dependency update.
- **Compatibility**: existing named records remain valid. The marker never replaces a completed response or suppresses remaining validation.
- **Publication**: implement against current `origin/main`, then publish a narrow backport from `958c488eb1d5ac3720e891e49cc9a8cb79271c6b` for the product. Do not import unrelated upstream changes.
- **Exclusions**: no deployment, lesson activation, benchmark quality claim, credential change, or production conversation access.
