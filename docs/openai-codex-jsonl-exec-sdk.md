# OpenAI Codex-style JSONL exec and SDK control

Animus models non-interactive automation as typed runtime contracts rather than Codex CLI flags or SDK packages.

`animus-runtime` exposes `RunRequest` for prompt input, external context handles, sandbox profile, output preference, and optional resume identity. Validation happens before model, tool, hook, or sandbox adapters run.

Automation consumers receive ordered, versioned `AutomationEnvelope` records that can be rendered as stable JSONL lines or Preserves-compatible tags. The stream covers thread, turn, item, model, tool, policy, hook, usage, completion, and failure events, with bounded event bodies and strict sequence ordering.

Final output is selected from the terminal completion event, not from progress logs. Structured final output is validated against a schema identity before a run is reported successful. Thread controller contracts expose start, resume, run, inspect, and cancel operations over typed thread and ledger identities, and resume/cancel guards fail closed on identity or ledger mismatch.
