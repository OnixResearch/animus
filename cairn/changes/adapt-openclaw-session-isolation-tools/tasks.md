# Tasks

- [ ] [serial] Define session isolation scope and non-claims for OpenClaw-inspired session planning without concrete stores, transcript files, clocks, channel connectors, or gateway persistence. r[agent_harness_runtime.openclaw_sessions.scope]
- [ ] [depends:scope] Add pure session-key planning for main, per-peer, per-channel-peer, per-account-channel-peer, group, channel, thread/topic, cron, hook, node, and subagent origins with identity-link normalization. r[agent_harness_runtime.openclaw_sessions.key_planning]
- [ ] [depends:key_planning] Add data-only session maintenance plans for stale-entry pruning, count caps, rotation thresholds, archive retention, and disk-budget decisions over host-loaded metadata. r[agent_harness_runtime.openclaw_sessions.maintenance_policy]
- [ ] [depends:key_planning] Add session-tool visibility planning for self, tree, agent, all, spawned-only sandbox clamps, cross-agent denial, and bounded history previews. r[agent_harness_runtime.openclaw_sessions.tools_visibility]
- [ ] [depends:tools_visibility] Add inter-session send/spawn provenance, reply/announce bounds, child lineage hooks, and safe transcript influence records. r[agent_harness_runtime.openclaw_sessions.inter_session_provenance]
- [ ] [depends:scope,key_planning,maintenance_policy,tools_visibility,inter_session_provenance] Update docs, subagent/ledger integration notes, facade inventory, and focused verification evidence. r[agent_harness_runtime.openclaw_sessions.verification]
