## Why

Animus orchestrates agent loops, tools, and model calls behind typed adapters and safe receipts. Agents routinely create or mutate behavioral models. Nothing currently forces model-check validation in the loop and records the verdict as a receipt bound to the agent action. A stale or missing check can pass handoff silently, and the model can drift from what the agent claimed to validate.

The stack model layer provides the bounded checking contract. Animus should consume it through an adapter, admit only receipted verdicts, and fail handoff when the receipt is stale or absent.

## What Changes

- Add an agent spec-contract rail that runs a bounded model check after model edits and before handoff.
- Consume the external checker through a pinned adapter behind the bounded checking contract; no ambient checker authority.
- Attach the verdict receipt to the agent action and refuse handoff on a stale, missing, or non-passing receipt.
- Record checker identity, model hash, bounds, and verdict in the loop receipt.
- Add positive and negative fixtures for receipted passes and for stale, missing, mismatched, and unsupported-checker cases.

## Impact

- **Files**: Animus spec-contract rail, checker adapter, loop receipts, fixtures, and docs.
- **Testing**: Positive receipted check-and-handoff plus negative stale, missing, mismatched, and unsupported-checker cases.
- **Boundary**: The rail proves that a model check ran, within the declared bounds, before handoff, and that the receipt matches the edited model. It does not prove the model is correct, the checker is sound, or the implementation matches the model.
