# Proposal: Adapt Clankers remote admission receipts for Animus

## Why

Clankers' remote daemon work demonstrates a useful admission pattern independent of its daemon, Matrix, and P2P product surfaces: every remote or headless entrypoint should evaluate a typed admission request, check UCAN/Basalt-style authority and local caveats, fail closed before mutation, and emit redacted receipts. Animus should adopt the admission receipt contract for future headless RPC, MCP, remote session, and automation entrypoints without adopting Clankers remote transport defaults.

## What Changes

- Define remote/headless admission request DTOs for audience, actor, entrypoint, session/workspace scope, requested action, capabilities, caveats, policy hash, and credential evidence.
- Require admission checks before session create, attach/resume, prompt submission, tool execution, memory mutation, artifact delivery, or workflow command side effects.
- Record safe admission receipts with verdict, reason class, resource/ability labels, credential hashes, caveat summary, policy hash, audience hash, and BLAKE3 receipt hash.
- Keep iroh, Matrix, daemon sockets, MCP transports, OAuth stores, and credential persistence in host adapters.
- Add positive and negative fixtures for authorized admission, wrong audience, expired/revoked/missing credentials, local caveat denial, unsupported entrypoints, missing policy, and receipt leak attempts.

## Impact

- **Files**: future admission DTOs, policy/authority checks, receipt fixtures, docs, tests, and Cairn artifacts.
- **Testing**: positive admission fixtures and negative audience/credential/caveat/policy/unsupported/leak fixtures.
- **Non-claims**: this does not add a daemon, P2P transport, Matrix bridge, remote auth CLI, or mandatory Basalt dependency in core crates.
