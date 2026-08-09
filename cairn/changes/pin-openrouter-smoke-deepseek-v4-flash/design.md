# Design: Pin OpenRouter smoke to DeepSeek V4 Flash 0731

## Success Contract

The packaged OpenRouter smoke must contact the provider only when `--model deepseek/deepseek-v4-flash-0731` is present. Every other model must fail before credential use or network contact.

Completion requires positive fixed-model tests, negative substitution tests, focused Rust and Nix rails, Cairn gates, and one clean-head live run.

## Decisions

### Keep the explicit argument and enforce equality

The binary retains `--model` for visible operator intent. Pure planning compares the argument with the exported reviewed model constant.

This is weaker than removing the argument because existing automation remains compatible. It is stronger than documentation-only guidance because model substitution fails in code.

### Keep the model result out of evidence output

Metadata-only evidence continues to omit the model output, prompt, credential, and raw response. The reviewed model identity remains visible in source, docs, and the invocation.

## Risks and Non-claims

The pin proves only the selected request subject. It does not prove DeepSeek quality, OpenRouter routing, availability, billing, or production readiness.
