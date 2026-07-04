# Prompt mode profiles

Animus prompt mode profiles adapt the Zerostack profile/directive pattern as typed adapter DTOs. Profiles are descriptors, not authority-bearing prompt blobs.

## Contract

A profile descriptor contains:

- stable profile ID and purpose;
- requested action posture such as ask, brainstorm, plan, code, debug, review, simplify, or write-prompt;
- required capability posture labels;
- source identity, trust scope, and host trust decision;
- deterministic precedence;
- bounded body text; and
- prompt-inclusion policy.

`animus-adapters` owns the pure descriptor validation and assembly functions. Hosts own discovery, file loading, admission decisions, and persistence.

## Directives

A profile may use a first-line directive in this form:

```text
#!animus mode=review chain=plan
```

Directive parsing happens before prompt assembly. Valid directives become typed metadata, and the directive line is stripped from model-visible body text. Unknown modes, malformed tokens, unsupported chain targets, and duplicate exclusive directives fail closed with typed diagnostics and cannot change runtime mode or continuation state.

## Authority boundary

Profile-requested modes are posture metadata only. `bind_prompt_profile_mode` evaluates requested capabilities against host authority and runtime policy inputs. A profile can narrow behavior, but it cannot grant missing write, exec, network, model, or tool authority; denied capabilities remain denied or ask-gated by stronger policy.

## Chain transitions

Chain-of-prompts transitions are explicit `PromptProfileChainPlan` records with source profile, target profile, host/user decision, and optional bounded extra instruction text. Declined or unavailable transitions keep the source profile active and record only safe metadata.

## Receipts

Prompt profile receipts store profile ID, source identity, directive outcomes, chain decision, omission reason, and a BLAKE3 hash of the profile body. Receipts do not persist raw profile bodies, hidden directives, credentials, provider payloads, tool input/output, or unbounded prompt material.

## Non-goals

This contract does not copy Zerostack prompt text, adopt Zerostack config directories, discover profile files in reusable crates, weaken sandbox constraints, or bypass host-owned permission policy.
