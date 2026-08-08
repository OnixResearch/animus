# Prime Agent daemon-workers cohort fixture

- Repository: `https://github.com/PrimeIntellect-ai/prime-agent`
- Revision: `c5991bc853d27754aed345c13ff4d2e05c40f1f4`
- License: MIT
- Reviewed cohort: `prime-agent-daemon-workers`
- Reviewed architecture: client rendering/input, daemon supervisor discovery/routing/attachments/health, and one worker per root session tree.
- Reviewed semantics: detach ends a subscription without stopping execution; reattach uses a live stream or a current recovery snapshot; process separation provides lifecycle containment only.

Animus adopts only this bounded semantic subset. It does not adopt Prime Agent process spawning, sockets, transcript persistence, Python, TypeScript, sandboxing, or multi-machine supervision.
