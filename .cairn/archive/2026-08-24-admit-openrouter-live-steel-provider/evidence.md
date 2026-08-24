# Evidence

Date: 2026-08-24

Outcome: pass for the declared configuration and fail-closed provider-cohort scope.

## Passing evidence

- `nix build .#checks.x86_64-linux.live-openai-provider-host --no-link -L --option builders ''` passed.
- `nix build .#checks.x86_64-linux.live-steel-agent-shell --no-link -L --option builders ''` passed.
- `nix build .#checks.x86_64-linux.packaged-live-openrouter-smoke --no-link -L --option builders ''` passed.
- Exact OpenAI, OpenRouter, and local dSpark configuration fixtures passed.
- Crossed endpoint, crossed credential, unknown provider, arbitrary endpoint, and fallback fixtures failed closed.
- Strict Clippy passed through the packaged focused rails.
- Strict Octet passed with zero findings through `cargo tigerstyle check`.
- Component-catalog and runtime-facade inventories passed with positive and negative fixtures.
- Cairn validation and proposal, design, and task gates passed before sync.

## Bounded warnings and non-claims

- The optional local binary cache timed out. Nix disabled it and built the required Octet derivation locally.
- No credentialed OpenAI or OpenRouter request ran.
- Provider availability, account authority, model behavior, remote correctness, production readiness, and release eligibility remain non-claims.
