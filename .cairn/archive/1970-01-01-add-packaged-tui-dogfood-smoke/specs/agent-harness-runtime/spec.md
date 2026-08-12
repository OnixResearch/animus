# agent-harness-runtime Change Spec: Packaged TUI dogfood smoke

## ADDED Requirements

### Requirement: Packaged TUI dogfood smoke remains non-default

r[agent_harness_runtime.packaged_tui_dogfood_smoke.contract] Animus MUST provide a non-default packaged TUI smoke that composes the accepted headless dogfood path with a bounded Ratatui `TestBackend` render.

#### Scenario: Feature-gated smoke composes accepted surfaces

r[agent_harness_runtime.packaged_tui_dogfood_smoke.contract.feature_gated]
- GIVEN a host enables the packaged TUI dogfood feature
- WHEN the smoke runs a bounded prompt fixture
- THEN it calls the accepted packaged headless smoke before TUI projection
- AND default Animus builds do not require the packaged TUI binary or its Ratatui backend dependency.

### Requirement: TUI dogfood projection is pure and bounded

r[agent_harness_runtime.packaged_tui_dogfood_smoke.projection] Animus MUST validate the headless receipt, event count, viewport, and safe labels before it builds the TUI view plan.

#### Scenario: Safe headless evidence reaches the fixed viewport

r[agent_harness_runtime.packaged_tui_dogfood_smoke.projection.pass]
- GIVEN the accepted headless smoke returns a bounded completed receipt
- WHEN pure TUI projection runs
- THEN it builds an `AppShellState` snapshot from safe metadata only
- AND the host renders that snapshot through the fixed Ratatui `TestBackend` viewport.

#### Scenario: Oversized receipt fails before rendering

r[agent_harness_runtime.packaged_tui_dogfood_smoke.projection.oversized]
- GIVEN the supplied headless receipt exceeds the named byte limit
- WHEN TUI projection validates the receipt
- THEN it returns a stable oversized-receipt error
- AND it does not create or draw a Ratatui backend.

### Requirement: Terminal and cleanup observations gate success

r[agent_harness_runtime.packaged_tui_dogfood_smoke.host_gate] The packaged TUI smoke MUST require an available deterministic backend before dispatch and a complete cleanup observation before success.

#### Scenario: Terminal loss blocks dispatch

r[agent_harness_runtime.packaged_tui_dogfood_smoke.host_gate.terminal_loss]
- GIVEN the deterministic terminal backend is unavailable
- WHEN smoke admission runs
- THEN it returns a stable terminal-unavailable error
- AND it does not call the accepted headless dogfood path.

#### Scenario: Cleanup failure blocks completion

r[agent_harness_runtime.packaged_tui_dogfood_smoke.host_gate.cleanup_failure]
- GIVEN the bounded headless and render steps complete
- WHEN the supplied cleanup observation reports failure
- THEN the smoke returns a stable cleanup-failure error
- AND it does not emit a successful TUI evidence receipt.

### Requirement: Packaged TUI evidence is metadata-only

r[agent_harness_runtime.packaged_tui_dogfood_smoke.receipt] Successful smoke output MUST contain only schema, status, evidence-rung, backend, count, BLAKE3 identity, and non-claim fields.

#### Scenario: Raw content is omitted

r[agent_harness_runtime.packaged_tui_dogfood_smoke.receipt.omits_raw_content]
- GIVEN the smoke receives a raw prompt and fake model or tool content
- WHEN it renders the final receipt
- THEN stdout contains BLAKE3 identities instead of raw content
- AND stdout omits the prompt, model output, tool output, headless receipt body, rendered buffer, provider payloads, credentials, and private paths.

### Requirement: TUI dogfood claims stay bounded

r[agent_harness_runtime.packaged_tui_dogfood_smoke.docs] Documentation and claim fixtures MUST name the packaged deterministic TestBackend rung and preserve stronger terminal and product surfaces as non-claims.

#### Scenario: TestBackend evidence does not imply live terminal readiness

r[agent_harness_runtime.packaged_tui_dogfood_smoke.docs.non_claims]
- GIVEN the packaged TUI smoke passes
- WHEN readiness documentation describes its evidence
- THEN it can claim the bounded packaged TestBackend path
- AND it MUST retain live terminal, raw mode, input polling, daemon attach, live provider, plugin, production, release, and Clankers parity as non-claims.

### Requirement: Packaged TUI smoke has positive and negative verification

r[agent_harness_runtime.packaged_tui_dogfood_smoke.verification] The change MUST include focused positive and negative tests, a packaged rail, a Nix check, Cairn gates, and safe closeout evidence.

#### Scenario: Focused rail covers fail-closed behavior

r[agent_harness_runtime.packaged_tui_dogfood_smoke.verification.focused_rail]
- GIVEN maintainers run the packaged TUI dogfood rail
- WHEN the rail completes
- THEN it covers success, terminal loss, capability denial, oversized receipt input, cleanup failure, and raw-content omission
- AND it keeps live terminal and production behavior outside the claim.
