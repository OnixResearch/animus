# agent-harness-runtime Change Spec: Packaged headless dogfood smoke

## Requirements

### Requirement: Packaged headless dogfood CLI remains non-default

r[agent_harness_runtime.packaged_headless_dogfood_smoke.cli] Animus MUST provide a packaged non-default headless dogfood smoke CLI that is gated behind an explicit host-shell feature and runs the accepted fake-backed dogfood path without making reusable crates depend on product behavior.

#### Scenario: Feature-gated packaged smoke runs fake-backed path

r[agent_harness_runtime.packaged_headless_dogfood_smoke.cli.feature_gated]
- GIVEN a host invokes the packaged smoke binary with the `dogfood-smoke` feature enabled
- WHEN the smoke runs a bounded prompt path
- THEN it calls the accepted fake-backed dogfood core through typed std-shell inputs
- AND default `animus-core`, `animus-adapters`, `animus-runtime`, and `animus-std` builds do not require live providers, credentials, network dispatch, process execution, persistence, daemon state, terminal UI, plugin loading, or product tools.

### Requirement: Packaged smoke admission parses prompt input once

r[agent_harness_runtime.packaged_headless_dogfood_smoke.admission] The packaged smoke MUST parse prompt text, policy profile, model capability, output contract, optional tool metadata, and deterministic failure fixtures at the shell boundary before constructing fake-backed dogfood DTOs.

#### Scenario: Missing prompt fails before dispatch

r[agent_harness_runtime.packaged_headless_dogfood_smoke.admission.missing_prompt]
- GIVEN a host invokes the packaged smoke without a prompt argument
- WHEN CLI admission runs
- THEN the smoke returns a typed missing-prompt failure
- AND it does not call model, tool, provider, network, persistence, daemon, or plugin services.

### Requirement: Packaged smoke receipts are safe line-oriented evidence

r[agent_harness_runtime.packaged_headless_dogfood_smoke.receipt] Packaged smoke output MUST contain only safe line-oriented evidence for schema, completed status, packaged evidence rung, core evidence rung, event counts, BLAKE3 prompt and receipt identities, ordered event labels, and omitted-surface non-claims.

#### Scenario: Raw prompt is omitted from smoke output

r[agent_harness_runtime.packaged_headless_dogfood_smoke.receipt.raw_prompt_omitted]
- GIVEN the packaged smoke receives raw prompt text
- WHEN it renders stdout
- THEN stdout contains the prompt BLAKE3 identity instead of the raw prompt
- AND stdout does not contain raw prompts, raw model outputs, raw tool bodies, provider payloads, credentials, private paths, or unbounded logs.

### Requirement: Packaged smoke docs name stronger-rung non-claims

r[agent_harness_runtime.packaged_headless_dogfood_smoke.docs] Packaged dogfood documentation and claim-hygiene fixtures MUST name packaged headless smoke as the highest completed rung and keep live providers, credential discovery, network dispatch, default product tools, persistence, daemon/attach, TUI dogfood, plugin runtime, production readiness, release tagging, and Clankers parity as non-claims unless separately indexed.

#### Scenario: Packaged claim does not imply live providers

r[agent_harness_runtime.packaged_headless_dogfood_smoke.docs.no_live_provider_claim]
- GIVEN a user reads the packaged smoke documentation or dogfood readiness fixture
- WHEN the completed packaged rung is claimed
- THEN optional live-provider, TUI, daemon, plugin-runtime, persistent-session, production-readiness, and Clankers-parity surfaces remain listed as non-claims
- AND a stronger rung requires separate safe receipts.

### Requirement: Packaged smoke verification covers success and fail-closed cases

r[agent_harness_runtime.packaged_headless_dogfood_smoke.verification] The packaged smoke change MUST include positive and negative tests or fixtures for successful packaged smoke, missing prompt, missing model service, denied capability, malformed fixture output, unsafe labels, safe output leak checks, dogfood readiness claim hygiene, Cairn validation, and Cairn gates before archive.

#### Scenario: Focused rail validates packaged smoke evidence

r[agent_harness_runtime.packaged_headless_dogfood_smoke.verification.focused_rail]
- GIVEN maintainers run the packaged headless dogfood smoke focused rail
- WHEN the rail completes
- THEN it verifies the positive CLI smoke, negative fail-closed cases, safe output omission, std-shell unit tests, dogfood readiness claim hygiene, and Nix check wiring
- AND skipped live-provider, TUI, daemon, plugin, persistence, production, and release-index checks are recorded as non-claims or future rungs.
