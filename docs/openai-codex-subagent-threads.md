# OpenAI Codex-style subagent threads

Animus adapts Codex-style subagent threads as explicit, typed delegation contracts. Subagents start only from user, trusted-skill, or host-admitted automation requests; automatic or recursive spawning fails closed.

A delegation request records role, task, recipe/profile identity, parameter bindings, tool ceilings, workspace scope, model profile, timeout class, turn bound, return mode, summary schema, and optional external subagent descriptor. Capability resolution intersects parent policy, profile/recipe ceilings, requested capabilities, workspace scope, and host admission before a subagent may use tools.

Subagent transcripts stay isolated by default. The parent receives bounded handoff metadata, detail references only when the return mode and size bounds allow it, typed summary/aggregation outcomes, write-conflict visibility, and lifecycle report events for start, stop, continuation, failure, and aggregation. Raw child logs are not inserted into parent context unless explicitly imported under output bounds.
