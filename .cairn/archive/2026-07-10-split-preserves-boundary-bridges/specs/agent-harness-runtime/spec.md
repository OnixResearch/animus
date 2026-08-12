# agent-harness-runtime Change Spec: Split Preserves boundary bridges by payload family

## Requirements

### Requirement: Preserves bridge families have focused ownership

r[agent_harness_runtime.preserves_bridge_split.family_crates] Animus MUST split Preserves boundary serialization into focused bridge families for adapter DTOs, runtime events/reports, provenance or session records, and policy-visible DTOs, with each family depending only on its owning payload crates and shared schema primitives.

#### Scenario: Adapter DTO serialization avoids runtime dependencies

r[agent_harness_runtime.preserves_bridge_split.family_crates.adapter_minimal]
- GIVEN an embedder needs only adapter DTO Preserves serialization
- WHEN they select the adapter bridge family
- THEN they do not pull runtime event/report, provenance/session, policy-visible, std shell, provider, filesystem, process, network, credential, terminal, or persistence behavior
- AND decoded adapter payloads are revalidated through adapter DTO validators before use.

### Requirement: Schema catalog records bridge coverage

r[agent_harness_runtime.preserves_bridge_split.schema_catalog] The Preserves schema catalog MUST record each bridge family's owner, schema identity, coverage tier, dependency set, safe compatibility metadata policy, stable-wire status, and focused fixture coverage.

#### Scenario: Coverage tier prevents accidental stability claims

r[agent_harness_runtime.preserves_bridge_split.schema_catalog.coverage]
- GIVEN a payload family is internal-only, compatibility import/export, or review-only
- WHEN the schema catalog is rendered or checked
- THEN the family is not documented as a stable external wire contract
- AND fixture requirements match the declared coverage tier.

### Requirement: Preserves compatibility facade remains stable

r[agent_harness_runtime.preserves_bridge_split.facade] `animus-preserves` MUST preserve existing facade exports during the bridge-family migration and MUST provide direct family import guidance before removing or renaming any public serialization item.

#### Scenario: Existing Preserves consumers remain source-compatible

r[agent_harness_runtime.preserves_bridge_split.facade.compat]
- GIVEN downstream code imports existing `animus_preserves` schema, message, or codec items
- WHEN bridge families are introduced
- THEN representative existing imports continue to compile through the compatibility facade
- AND docs name the smaller direct bridge dependencies for new code.

### Requirement: Preserves bridge split verification

r[agent_harness_runtime.preserves_bridge_split.verification] Preserves bridge splits MUST include focused positive and negative tests for family-specific round trips, facade compatibility, schema mismatch, unsupported versions, missing payloads, unbounded metadata, unsafe raw material, provider JSON, private paths, and compatibility leak rejection.

#### Scenario: Closeout evidence covers bridge safety

r[agent_harness_runtime.preserves_bridge_split.verification.closeout]
- GIVEN maintainers intend to archive a Preserves bridge split
- WHEN closeout validation runs
- THEN evidence names family round-trip fixtures, malformed-input fixtures, facade compatibility, schema catalog checks, docs drift checks, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
