# Animus Live Steel Agent

This non-default package owns the headless live Steel agent composition root.
It selects the provider, durable store, coding session, Steel kernel, resumable
bridge, RLM executor, child service, skill loader, controls, events, and receipts.
Completed output returns the ordered private runtime-event snapshot to its worker.
The metadata-only receipt stores only the event count.

The package uses `AGPL-3.0-or-later` because it composes
`animus-std-rlm-host`. It does not change the workspace default members.

Configuration comes from `config/live-steel-agent-shell.ncl`. The application
validates its deterministic JSON export before it opens a store or uses a
credential. It admits only the exact OpenAI and OpenRouter Responses tuples.
Crossed endpoints, unknown providers, arbitrary URLs, OAuth, and fallback fail
before provider construction. The live binary accepts the selected credential
through standard input. It does not read environment variables, credential
files, or provider accounts.

See `docs/live-steel-agent-shell.md` for the claim boundary and validation rail.
