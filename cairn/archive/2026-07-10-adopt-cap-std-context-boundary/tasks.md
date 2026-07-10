## Tasks

- [x] [serial] Add `cap-std` only to `animus-std` shell/adaptor code that owns filesystem effects. r[agent_harness_runtime.cap_std_context_boundary.dependency]
- [x] [serial] Introduce typed capability roots for workspace, context-file, artifact, and host-store roots. r[agent_harness_runtime.cap_std_context_boundary.root_wrappers]
- [x] [serial] Convert context and adjacent std-shell path opens to capability-relative operations while preserving reusable crate boundaries. r[agent_harness_runtime.cap_std_context_boundary.conversion]
- [x] [serial] Add positive fixtures for valid context-file discovery, layering, and provenance under declared roots. r[agent_harness_runtime.cap_std_context_boundary.tests.positive]
- [x] [serial] Add negative fixtures for traversal, absolute paths, symlink escapes, missing roots, and hidden host-file exposure. r[agent_harness_runtime.cap_std_context_boundary.tests.negative]
- [x] [serial] Document the std-shell filesystem authority boundary and host-effect non-claims. r[agent_harness_runtime.cap_std_context_boundary.docs]
- [x] [serial] Run focused `animus-std` tests, boundary scans, Cairn validation, and gates. r[agent_harness_runtime.cap_std_context_boundary.validation]
