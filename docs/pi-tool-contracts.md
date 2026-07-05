# Pi-derived tool contracts

Animus adapts Pi's tool-contract pattern as reusable data contracts, not as Pi's concrete tools or renderer stack.

## Contract boundary

Reusable crates expose typed tool metadata, argument preparation, bounded output, mutation-target planning, and typed dispatch failures. Hosts still own executor registration, schema engines, filesystem/process/network authority, output artifact storage, and mutation coordination.

Tool definitions carry validated name, label, description, parameter schema identity, prompt snippet, prompt guidelines, source/provenance labels, capability labels, and result metadata shape. Prompt construction uses only active tool definitions, and every snippet or guideline remains attributed to the tool that supplied it.

## Validation rules

- Legacy or resumed arguments may pass through a pure compatibility mapping before strict current-schema validation.
- Unknown fields, malformed values, and duplicate fields after preparation fail before executor dispatch.
- Tool output exposes bounded model-visible text, counts, safe references, and BLAKE3 hashes when truncation or evidence identity requires them.
- Full-output references are host-owned handles, not raw absolute paths, URLs, credentials, or unbounded bodies.
- Mutating tools declare opaque target keys so a host-owned coordinator can serialize conflicting mutation windows.
- Dispatch failures are typed: invalid input, missing executor, unavailable service, denied capability, cancellation, malformed output, unbounded oversized output, or execution failure.

## Non-claims

This contract does not adopt Pi's built-in tools, TypeBox schemas, npm extension runtime, terminal renderers, or filesystem mutation implementation. It only preserves the safety shape needed for swappable Animus host tools.
