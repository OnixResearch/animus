# Proposal: Adapt Pi provider registry shell for Animus

## Why

Pi's provider/model registry shows useful shell-level behavior: model capability metadata, scoped model selection, thinking-level support, custom provider definitions, OAuth/API-key resolution, and dynamic provider updates. Animus should adapt these ideas only at the std-shell boundary. The reusable runtime must continue to receive injected model/provider services and must not discover credentials or construct network clients internally.

This change creates a clear line between host/provider configuration and runtime dispatch, preventing provider convenience features from leaking into the functional core.

## What Changes

- Define shell-owned provider and model registry contracts with typed provider IDs, model IDs, API kind, endpoint identity, capability metadata, context/output budgets, cost metadata, and reasoning/thinking-level support.
- Keep repo-owned provider policy/config in Nickel with typed contracts where Animus owns the config shape.
- Define credential resolution as an imperative-shell service with explicit precedence and redacted diagnostics.
- Map selected provider/model entries into injected runtime model services without runtime client construction or credential lookup.
- Add positive and negative tests for model selection, capability clamping, missing credentials, malformed provider configs, unavailable providers, and runtime boundary purity.

## Impact

- **Files**: `animus-std` provider registry/config modules, Nickel policy/config, generated config checks if needed, tests, and docs.
- **Testing**: positive registry/model selection tests; negative tests for bad configs, unsupported API kinds, missing credentials, invalid thinking levels, and attempts to use registry from runtime core.
- **Non-claims**: this change does not add real network provider clients, Pi OAuth flows, subscription login, package-installed providers, or provider discovery inside the runtime core.
