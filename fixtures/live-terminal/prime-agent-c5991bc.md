# Prime Agent live-terminal cohort fixture

- Repository: `https://github.com/PrimeIntellect-ai/prime-agent`
- Revision: `c5991bc853d27754aed345c13ff4d2e05c40f1f4`
- License: MIT
- Reviewed source: `packages/coding-agent/src/modes/interactive/interactive-mode.ts`
- Reviewed boundary: the interactive mode uses an `AgentConnection` for core execution.
- Reviewed lifecycle: the interactive mode registers signal cleanup, disposes its connection, and stops its terminal UI.

Animus adopts only the client, connection, snapshot, input, rendering, and cleanup split. It does not adopt the TypeScript runtime, ambient path access, tool downloads, direct process execution, provider code, persistence, credentials, extensions, images, remote transport, or fallback behavior.
