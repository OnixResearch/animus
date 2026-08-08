# Exact producer envelope for Kamacite cross-repository fixture admission.
# r[impl agent_harness_runtime.application_host_binding_pilot.kamacite_contract]
let contracts = import "../../policy/contracts/primitives.ncl" in
let profile = import "profile.ncl" in
let producer_revision_value = "58cab252797865df5ccb3c7364f0ad994d4c8ea7" in
let source_identity_value = "69fb71f312cc479e34aa91f00f01d16a12caf5253bff0cf003076f94d0a52339" in
let producer_non_claims = [
  "producer fixture does not prove producer correctness",
  "producer fixture does not grant host capability or authorization",
  "producer fixture does not prove runtime success or release eligibility",
] in
let SourceEntry = {
  path | String,
  blake3 | contracts.lowercase_blake3 "APPLICATION_BINDING_PRODUCER_SOURCE_HASH",
} in
let ProducerContract = {
  schema | contracts.exact "APPLICATION_BINDING_PRODUCER_SCHEMA" "animus.application-host-binding-producer.v1",
  producer | contracts.exact "APPLICATION_BINDING_PRODUCER_NAME" "animus",
  producer_revision | contracts.exact "APPLICATION_BINDING_PRODUCER_REVISION" producer_revision_value,
  source_identity | contracts.exact "APPLICATION_BINDING_PRODUCER_SOURCE_IDENTITY" source_identity_value,
  source_identity_algorithm | contracts.exact "APPLICATION_BINDING_PRODUCER_SOURCE_ALGORITHM" "blake3-of-ordered-b3sum-lines-v1",
  source_manifest | Array SourceEntry,
  kamacite_revision | contracts.exact "APPLICATION_BINDING_PRODUCER_KAMACITE_REVISION" "b21732a7458b660367f7281c717a5acc3f8c8190",
  binding | profile.binding_contract,
  non_claims | Array String,
} in
let _producer_validator = fun value =>
  if std.array.length value.source_manifest != 5 then
    'Error { message = "APPLICATION_BINDING_PRODUCER_SOURCE_COUNT" }
  else if !contracts.unique_by (fun entry => entry.path) value.source_manifest then
    'Error { message = "APPLICATION_BINDING_PRODUCER_DUPLICATE_SOURCE" }
  else if value.non_claims != producer_non_claims then
    'Error { message = "APPLICATION_BINDING_PRODUCER_NON_CLAIMS" }
  else
    'Ok
in
let CheckedProducerContract = contracts.with_validator ProducerContract _producer_validator in
{
  producer_contract = CheckedProducerContract,
  producer | CheckedProducerContract = {
    schema = "animus.application-host-binding-producer.v1",
    producer = "animus",
    producer_revision = "58cab252797865df5ccb3c7364f0ad994d4c8ea7",
    source_identity = "69fb71f312cc479e34aa91f00f01d16a12caf5253bff0cf003076f94d0a52339",
    source_identity_algorithm = "blake3-of-ordered-b3sum-lines-v1",
    source_manifest = [
      { path = "crates/animus-runtime/src/applicationbinding/core.rs", blake3 = "a261c83344aec5423a2d262864cefe9d12afc5ebecf0f1ff424df5a5315d6a2c" },
      { path = "crates/animus-runtime/src/applicationbinding/data.rs", blake3 = "a3ba79d1edeb0f36ea2b9ba3baec0533c9937570a75a9e8f6c761d55c4f0e3b6" },
      { path = "crates/animus-runtime/src/applicationbinding/mod.rs", blake3 = "419ac79bd9aabaeea73312621a94c2e533e2f3bfe18b19a4be9f9f16422aa4c4" },
      { path = "config/application-host-binding-pilot/profile.ncl", blake3 = "4c1ff8299c9edfb590dd85e0a40a302e94cc7aa32e665d16ed5f409bffb5c800" },
      { path = "config/application-host-binding-pilot/generated/animus-binding.json", blake3 = "4c39a2af47aac16e2dcc5eaf4a99c340602d0e8b8db128b15d010dbdbe00c083" },
    ],
    kamacite_revision = "b21732a7458b660367f7281c717a5acc3f8c8190",
    binding = profile.binding,
    non_claims = producer_non_claims,
  },
}
