# agent-harness-runtime Change Spec: Animus Hugging Face dataset export

## Requirements

### Requirement: Animus sessions are dataset export sources

r[agent_harness_runtime.hf_dataset_export.session_source] Animus MUST export training datasets from validated Animus session snapshots or replay contexts, with typed row identity, source span metadata, target selection, and export errors, without treating Claude Code JSONL, Hugging Face libraries, filesystem paths, tokenizers, or trainer frameworks as core dependencies.

#### Scenario: Export starts from validated replay context

r[agent_harness_runtime.hf_dataset_export.session_source.validated]
- GIVEN an Animus session snapshot has been validated and replay context has been selected
- WHEN dataset export begins
- THEN the exporter consumes the selected Animus replay data and safe metadata
- AND it does not read provider-native transcripts, external agent session files, filesystem paths, tokenizers, or trainer APIs inside pure export logic.

#### Scenario: Invalid session source fails before row emission

r[agent_harness_runtime.hf_dataset_export.session_source.rejects_invalid]
- GIVEN an Animus session snapshot has broken parent links, duplicate row source spans, unsupported entry kinds, or no selected replay path
- WHEN dataset export is requested
- THEN export returns a typed source-validation error
- AND no partial dataset rows are emitted unless an explicit partial-export policy is selected.

### Requirement: Hugging Face chat-message export

r[agent_harness_runtime.hf_dataset_export.chat_messages] Animus MUST export supervised fine-tuning rows in a Hugging Face-compatible chat `messages` shape using deterministic user, assistant, system, and tool role mapping from Animus replay context.

#### Scenario: SFT row is chat-template compatible

r[agent_harness_runtime.hf_dataset_export.chat_messages.sft]
- GIVEN a validated Animus replay path contains user and assistant model-visible turns
- WHEN the chat-message exporter runs with the default SFT policy
- THEN it emits a dataset row with ordered `messages` entries containing `role` and `content` fields
- AND the row is suitable for downstream `tokenizer.apply_chat_template()` without Animus calling a tokenizer.

#### Scenario: Bad chat rows fail closed

r[agent_harness_runtime.hf_dataset_export.chat_messages.rejects_bad_rows]
- GIVEN replay context contains invalid roles, empty conversations, empty assistant targets, unsupported content blocks, or malformed tool-result ordering
- WHEN chat-message export runs
- THEN it returns a typed export error or deterministic skipped-row reason
- AND it does not silently emit malformed `messages` rows.

### Requirement: Explicit safe export policy

r[agent_harness_runtime.hf_dataset_export.policy] Animus MUST require a typed export policy for lossy or sensitive choices, including system text, tool calls, tool results, hidden reasoning, metadata projection, redaction, and quality thresholds.

#### Scenario: Default policy prevents private leakage

r[agent_harness_runtime.hf_dataset_export.policy.leakage_default]
- GIVEN a session contains hidden reasoning, raw tool outputs, environment metadata, provider details, and private provenance
- WHEN export runs with the default policy
- THEN hidden reasoning, raw tool outputs, private environment fields, and private provenance are excluded from dataset rows
- AND the inspection report records safe policy decisions without exposing the raw private payloads.

#### Scenario: Opt-in policy is explicit and testable

r[agent_harness_runtime.hf_dataset_export.policy.opt_in]
- GIVEN a host selects a policy that includes tool calls, tool results, system text, or selected metadata
- WHEN export runs
- THEN the emitted rows contain only the explicitly enabled categories
- AND negative tests prove disabled categories remain absent.

### Requirement: Preference-learning row export

r[agent_harness_runtime.hf_dataset_export.preference_rows] Animus MUST export preference-learning rows only from explicit Animus outcome annotations that identify prompt context, chosen responses, rejected responses, and safe metadata.

#### Scenario: Preference row preserves chosen and rejected alternatives

r[agent_harness_runtime.hf_dataset_export.preference_rows.validated]
- GIVEN a session records a prompt span with explicit chosen and rejected assistant alternatives
- WHEN preference export runs
- THEN it emits a row containing the prompt messages, chosen assistant content, rejected assistant content, row identity, and safe metadata
- AND it does not infer preference labels from file order or branch order alone.

#### Scenario: Malformed preference examples are rejected

r[agent_harness_runtime.hf_dataset_export.preference_rows.rejects_malformed]
- GIVEN an example lacks a prompt span, duplicates a candidate, lacks an outcome label, or requests unsafe metadata projection
- WHEN preference export runs
- THEN it returns a typed export error or deterministic skipped-row reason
- AND no fabricated chosen/rejected pair is emitted.

### Requirement: RL trajectory export

r[agent_harness_runtime.hf_dataset_export.rl_trajectories] Animus MUST export RL trajectory rows as ordered prompt, action, observation, reward, terminal outcome, and safe metadata records when sessions contain explicit trajectory annotations.

#### Scenario: Complete trajectory exports deterministically

r[agent_harness_runtime.hf_dataset_export.rl_trajectories.validated]
- GIVEN an Animus session contains an explicitly annotated trajectory with prompts, actions, observations, rewards, and terminal outcome
- WHEN RL trajectory export runs
- THEN it emits the trajectory in deterministic order with safe row identity and metadata
- AND tool/action links remain structured rather than flattened into ambiguous prose.

#### Scenario: Incomplete trajectory fails closed

r[agent_harness_runtime.hf_dataset_export.rl_trajectories.rejects_incomplete]
- GIVEN a trajectory has missing rewards, no terminal outcome, orphaned observations, invalid action/tool links, or unsafe metadata requests
- WHEN RL trajectory export runs
- THEN it returns a typed export error or deterministic skipped-row reason
- AND it does not fabricate rewards or terminal outcomes.

### Requirement: Structured dataset inspection

r[agent_harness_runtime.hf_dataset_export.inspection] Animus MUST provide structured dataset inspection reports that summarize row counts, skipped rows, empty or short turns, tool/reasoning leakage warnings, usage or token summaries when available, and export policy decisions.

#### Scenario: Inspection explains export quality safely

r[agent_harness_runtime.hf_dataset_export.inspection.safe_reports]
- GIVEN a batch of Animus sessions contains exportable rows, skipped rows, and policy-sensitive payloads
- WHEN inspection runs
- THEN it returns deterministic aggregate counts, warning classes, skipped-row reasons, and safe metadata summaries
- AND the report does not include raw hidden reasoning, raw private tool output, or private provenance payloads.

### Requirement: Closeout evidence covers dataset export targets

r[agent_harness_runtime.hf_dataset_export.closeout] Dataset export implementation MUST be closed out with focused positive and negative tests, documentation for Hugging Face/TRL integration, workspace checks for touched crates, pre-commit hooks, Cairn validation, and Cairn gates.

#### Scenario: Export change is ready to archive

r[agent_harness_runtime.hf_dataset_export.closeout.evidence]
- GIVEN the dataset export change is implemented
- WHEN closeout validation runs
- THEN evidence includes SFT chat-message tests, preference-row tests, RL trajectory tests, leakage negative tests, inspection-report tests, documentation review, pre-commit results, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
