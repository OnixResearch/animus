# agent-harness-runtime Change Spec: Fake-backed headless dogfood shell

## Requirements

### Requirement: Non-default headless dogfood entrypoint

r[agent_harness_runtime.dogfood_headless_shell.entrypoint] Animus MUST provide a non-default headless or CLI dogfood entrypoint that can run one bounded prompt path through accepted runtime, adapter, policy, and receipt contracts using injected deterministic fake services.

#### Scenario: Host runs a fake-backed prompt

r[agent_harness_runtime.dogfood_headless_shell.entrypoint.fake_prompt]
- GIVEN a host invokes the non-default dogfood shell with a safe prompt request and deterministic fixture configuration
- WHEN the shell starts a run
- THEN it constructs the run from typed request, policy, model, tool, and output descriptors
- AND it does not read ambient credentials, discover providers, open the network, spawn processes, start a daemon, write a global store, or require a terminal UI.

### Requirement: Prompt and adapter admission precede dispatch

r[agent_harness_runtime.dogfood_headless_shell.admission] The dogfood shell MUST admit prompt input, policy profile, selected model capability, optional tool metadata, output contract, and noninteractive behavior before any model or tool adapter is called.

#### Scenario: Missing model service fails closed

r[agent_harness_runtime.dogfood_headless_shell.admission.missing_model]
- GIVEN a dogfood run request is otherwise valid
- AND no model service is injected
- WHEN admission evaluates the dispatch path
- THEN the shell returns a typed missing-service failure
- AND it does not attempt fallback provider discovery, network dispatch, tool execution, or final-response fabrication.

### Requirement: Dogfood events are ordered runtime facts

r[agent_harness_runtime.dogfood_headless_shell.events] A fake-backed dogfood run MUST emit ordered runtime events for prompt admission, model request, model response, optional tool-admission decision, completion status, and shutdown using the same event taxonomy consumed by headless, SDK, receipt, and future UI surfaces.

#### Scenario: Successful fake model response produces terminal event

r[agent_harness_runtime.dogfood_headless_shell.events.success]
- GIVEN the fixture model adapter returns a valid bounded response
- WHEN the dogfood run completes
- THEN the event sequence records admitted model dispatch before the model response
- AND the terminal completion event appears after all adapter outcome events.

### Requirement: Dogfood receipts store safe evidence only

r[agent_harness_runtime.dogfood_headless_shell.receipts] The dogfood shell MUST produce a safe receipt index that stores status classes, event counts, adapter labels, redaction markers, BLAKE3 identities, and omitted-surface non-claims without raw prompts, raw model outputs, raw tool bodies, provider payloads, credentials, private paths, or unbounded logs.

#### Scenario: Receipt leak attempt is rejected

r[agent_harness_runtime.dogfood_headless_shell.receipts.leak_rejection]
- GIVEN a fake-backed run attempts to persist raw prompt text or raw model output in durable receipt metadata
- WHEN receipt validation runs
- THEN validation rejects the receipt with a leak diagnostic
- AND no dogfood readiness claim is indexed from that receipt.

### Requirement: Dogfood docs name evidence rung and non-claims

r[agent_harness_runtime.dogfood_headless_shell.docs] Dogfood shell documentation and examples MUST name the highest completed evidence rung and list live providers, credential discovery, network dispatch, default product tools, persistence, daemon/attach, TUI dogfood, plugin runtime, production readiness, and Clankers parity as non-claims unless separately indexed.

#### Scenario: Example remains fake-backed only

r[agent_harness_runtime.dogfood_headless_shell.docs.fake_backed_only]
- GIVEN a user reads the dogfood shell example
- WHEN the example describes expected evidence
- THEN it states that the run proves fake-backed integration only
- AND it points to the next validation rung for packaged headless or optional live-provider claims.

### Requirement: Dogfood shell verification

r[agent_harness_runtime.dogfood_headless_shell.verification] The dogfood shell change MUST include positive and negative tests or fixtures for fake-backed success, missing model service, denied capability, malformed fixture output, unsafe evidence, receipt leak rejection, and omitted-surface overclaim detection before archive.

#### Scenario: Closeout evidence is bounded

r[agent_harness_runtime.dogfood_headless_shell.verification.closeout]
- GIVEN maintainers prepare to archive the dogfood shell change
- WHEN closeout validation runs
- THEN evidence names the fake-backed smoke, negative fixtures, adapter conformance, dogfood readiness, docs, Cairn validation, and Cairn gates
- AND skipped optional live-provider, TUI, daemon, plugin, persistence, and production checks are recorded as non-claims or blockers.
