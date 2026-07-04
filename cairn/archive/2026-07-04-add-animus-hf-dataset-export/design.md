# Design: Add Animus Hugging Face dataset export

## Context

Animus is building neutral session ledgers, replay context, provenance, and runtime policy boundaries. Those session files are valuable for model improvement only if they can be exported into well-known dataset shapes without losing safety semantics or smuggling internal harness data into training rows.

`claude_converter` demonstrates the right product surface size: load sessions, convert records to `messages`, optionally write JSON, and inspect quality. Animus should adapt that conversion shape while replacing the source model with Animus session snapshots and preserving structured tool/reasoning metadata until an export policy intentionally includes or removes it.

## Decisions

### 1. Animus sessions are the source of truth

**Choice:** The exporter consumes validated Animus session snapshots or replay contexts selected by active branch/session policy.

**Rationale:** Training data should reflect Animus' canonical replay semantics, including summaries, branches, labels, provenance, and policy decisions. Claude Code JSONL or any other external transcript can be imported separately, but export always starts from Animus-owned DTOs.

**Boundary:** File scanning, JSONL parsing, JSONL writing, dataset sharding, and Hugging Face upload remain shell/adapter concerns.

### 2. Chat export uses the standard Hugging Face `messages` shape

**Choice:** The primary SFT output is a row containing ordered `{role, content}` messages suitable for `tokenizer.apply_chat_template()`.

**Rationale:** This is the lowest-friction target for Transformers, TRL SFTTrainer, Axolotl, and LLaMA-Factory-style chat datasets.

**Boundary:** Animus does not call Transformers or tokenize rows. It emits deterministic rows and reports enough metadata for downstream tooling to decide tokenization.

### 3. Export policy controls tool, system, reasoning, and metadata inclusion

**Choice:** Every lossy or sensitive transformation is driven by an explicit typed export policy with safe defaults.

**Rationale:** Agent sessions contain tool outputs, hidden reasoning, environment details, prompts, failures, and private metadata. Default export must be useful for SFT while failing closed on leakage risks.

**Boundary:** Policy evaluation is pure over in-memory rows. Shell configuration may be sourced from Nickel or host config before typed policy construction.

### 4. Preference and RL formats are first-class targets

**Choice:** The exporter supports supervised chat rows, preference rows, and trajectory rows as distinct typed outputs.

**Rationale:** Fine-tuning and RL use different data contracts. Trying to squeeze every session into chat messages loses chosen/rejected alternatives, rewards, actions, observations, and terminal outcome metadata.

**Boundary:** Animus defines durable row DTOs; trainers own algorithm-specific packaging such as DPO, ORPO, IPO, PPO, GRPO, or custom replay buffers.

### 5. Inspection is data, rendering is shell

**Choice:** Dataset inspection returns structured quality reports instead of printing directly.

**Rationale:** The same report should be usable in CLI output, CI gates, dashboards, and tests. Pure report generation also makes negative leakage checks easy to prove.

**Boundary:** ANSI color, tables, JSON pretty-printing, and filesystem output belong in std-shell renderers.

## Risks / Trade-offs

- Overly permissive defaults can train on secrets, hidden reasoning, or brittle tool traces. Defaults should exclude sensitive categories and require opt-in inclusion.
- Overly strict filters can discard useful trajectories. Reports must explain skipped rows and policy decisions.
- Preference/RL rows require trustworthy outcome signals. The exporter must reject malformed chosen/rejected pairs and missing reward labels instead of fabricating labels.
- Hugging Face dataset conventions vary across tools. Keep the core output stable and provide thin named presets rather than hard dependencies.
