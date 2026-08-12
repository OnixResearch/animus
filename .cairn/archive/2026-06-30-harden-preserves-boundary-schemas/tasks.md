# Tasks

- [x] [serial] Define Preserves schema coverage tiers for internal DTOs, stable wire contracts, compatibility imports/exports, and review fixtures. r[agent_harness_runtime.preserves_boundary.coverage_tiers]
- [x] [depends:coverage_tiers] Add versioned schema identities for adapter messages, runtime events, final reports, errors, session/provenance records, policy-visible DTOs, and dataset/export rows. r[agent_harness_runtime.preserves_boundary.versioning]
- [x] [depends:versioning] Add positive Preserves text/binary round-trip fixtures for stable boundary payload families. r[agent_harness_runtime.preserves_boundary.round_trip]
- [x] [depends:versioning] Add negative fixtures for wrong kind, missing fields, invalid text, unsupported versions, unbounded payloads, unsafe raw material, and compatibility leak attempts. r[agent_harness_runtime.preserves_boundary.malformed]
- [x] [depends:versioning] Implement unknown/future field handling that preserves safe bounded metadata or fails with typed compatibility errors. r[agent_harness_runtime.preserves_boundary.unknown_fields]
- [x] [depends:round_trip,malformed,unknown_fields] Add docs and dependency checks proving core behavior consumes typed values after boundary validation rather than raw Preserves ASTs. r[agent_harness_runtime.preserves_boundary.core_isolation]
- [x] [depends:round_trip,malformed,unknown_fields,core_isolation] Run Preserves fixtures, workspace checks, and Cairn gates with reviewable evidence. r[agent_harness_runtime.preserves_boundary.closeout]
