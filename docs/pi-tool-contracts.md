# Pi-derived tool contracts

Animus adapts Pi's tool-contract pattern as reusable data contracts, not as Pi's concrete tools or renderer stack.

## Contract boundary

Reusable crates expose typed tool metadata, current-shape argument validation, bounded output, mutation-target planning, exact capability admission, and typed dispatch failures. Hosts still own executor registration, schema engines, filesystem/process/network authority, output artifact storage, and mutation coordination.

Tool definitions carry validated name, label, description, parameter schema identity, prompt snippet, prompt guidelines, source/provenance labels, capability labels, executor status, trust status, and result metadata shape. Prompt construction and dispatch use only active trusted tool definitions, and every snippet or guideline remains attributed to the tool that supplied it.

## Validation rules

- Runtime-visible tool calls are current-shape only: name, schema identity, validated named arguments, correlation identity, output policy, mutation targets, required capability labels, and approval posture are required before dispatch.
- Legacy raw input strings, compatibility argument shims, missing schema identities, unknown fields, malformed values, duplicate fields, and raw response bodies fail before executor dispatch.
- Tool output exposes bounded model-visible text, completion/truncation status, byte/character/line counts, redaction class, safe host-owned references, and BLAKE3 hashes when required.
- Full-output references are host-owned handles, not raw absolute paths, URLs, credentials, provider payloads, or unbounded bodies.
- Mutating tools declare opaque target keys so a host-owned coordinator can serialize conflicting mutation windows while admitting read-only or non-overlapping work independently.
- Dispatch failures are typed: invalid input, missing executor, unavailable service, denied capability, approval denial, cancellation, timeout, malformed output, unsafe output reference, unbounded oversized output, mutation conflict, fallback denial, or execution failure.

## Non-claims

This contract does not adopt Pi's built-in tools, TypeBox schemas, npm extension runtime, terminal renderers, or filesystem mutation implementation. It only preserves the safety shape needed for swappable Animus host tools.
