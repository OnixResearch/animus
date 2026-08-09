# Proposal: Pin OpenRouter smoke to DeepSeek V4 Flash 0731

## Why

The packaged OpenRouter smoke currently accepts any valid model identity. Future runs must use one reviewed model so model substitution cannot change the evidence subject.

## What Changes

- Pin the OpenRouter smoke to `deepseek/deepseek-v4-flash-0731`.
- Retain the explicit `--model` argument, but reject every other value before credential use or network contact.
- Update tests, focused rail constants, docs, and live evidence.

## Impact

The OpenAI smoke remains configurable. The OpenRouter smoke gains a narrower model boundary. No model quality, reliability, routing, billing, or production claim is added.
