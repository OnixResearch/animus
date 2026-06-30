# Pi Provider Registry Shell

Animus adapts Pi-style provider/model registry behavior only at the `animus-std` or host-shell boundary. Reusable runtime crates continue to receive injected model services and typed request DTOs; they do not read provider config, inspect credentials, perform OAuth, or construct network clients.

## Shell-owned registry

`animus-std::ProviderRegistry` validates provider entries, model entries, model capabilities, credential requirements, and dynamic host-provided entries. A selected model produces `SelectedModel` metadata for a shell-owned injected model service:

- validated provider/model IDs
- API kind and endpoint identity
- input modalities
- context/output budgets
- reasoning support and effective thinking level
- optional integer cost metadata

The selected metadata is not a network client. The shell remains responsible for creating any concrete provider client and injecting it behind the model adapter boundary.

## Thinking-level policy

Model capabilities declare supported `ThinkingLevel` values. A `ModelSelection` resolves requested thinking with an explicit policy:

- `ThinkingPolicy::Clamp` chooses the highest supported level at or below the request.
- `ThinkingPolicy::Reject` returns a typed unsupported-thinking diagnostic.

## Nickel-owned config

`config/provider-registry.ncl` is the typed Nickel source for repo-owned provider registry examples. It declares contracts for provider IDs, API kinds, endpoints, modalities, budgets, reasoning support, thinking levels, credential labels, and cost metadata.

Run:

```sh
nix develop -c scripts/config/check-provider-registry
```

The check exports the valid registry and proves negative fixtures fail for empty IDs, unsupported API kinds, non-positive budgets, and missing endpoint identity.

## Credentials

Credential resolution is an imperative-shell service. `resolve_credential_from_env_pairs` is the deterministic core used by tests; a real shell supplies `std::env::vars()` or another secret store. Diagnostics expose only safe source labels such as `env:OPENAI_API_KEY`; secret values, OAuth tokens, and credential file contents are not included in errors or credential handles.

## Runtime purity evidence

Provider registry tests include static checks that `animus-runtime` does not depend on `animus-std` and does not reference provider registry or credential source types. Runtime dispatch remains injected-service only.
