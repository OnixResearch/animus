# Proposal: Add live OpenAI provider host

## Why

Animus has a checked OpenAI Responses dispatch plan, but no host block sends that plan through an HTTPS client. This gap keeps provider execution at the fixture-only evidence rung.

A narrow OpenAI host can close the implementation gap without adding network or credential authority to reusable crates. Default checks must remain deterministic and credential-free.

## What Changes

- Add `animus-std-openai`, a non-default authority-scoped host crate for one OpenAI Responses HTTPS endpoint.
- Keep request admission and response normalization pure over explicit plans and bounded bytes.
- Put HTTPS, bearer-header use, deadlines, cancellation observations, and bounded body reads in a thin optional shell.
- Add a root feature, host catalog entry, focused rail, deterministic positive and negative tests, and an ignored live smoke.
- Update readiness docs to distinguish host availability from live-provider evidence.

## Impact

- **Crates**: `animus-std-openai`, `animus-std`, and the root `animus` facade.
- **Lifecycle**: accepted runtime requirements, component metadata, host catalog, and focused evidence.
- **Testing**: pure projection and parser tests, fake transport tests, dependency isolation, root feature compilation, Cairn gates, and the focused Nix rail.
- **Non-claims**: no provider discovery, credential storage, automatic fallback, retries, streaming, tool calls, images, structured output, production reliability, or live evidence without an explicit smoke result.
