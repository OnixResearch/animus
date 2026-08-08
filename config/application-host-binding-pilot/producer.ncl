# Exact producer envelope for Kamacite cross-repository fixture admission.
# r[impl agent_harness_runtime.application_host_binding_pilot.kamacite_contract]
let contracts = import "../../policy/contracts/primitives.ncl" in
let profile = import "profile.ncl" in
let producer_revision_value = "c09991fa299f1f74b04d066bf31382e34108fa82" in
let source_identity_value = "1290a95e48b531cc15386490844178a6ed49f25d6c1355e9a29dfea972bfcc62" in
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
    producer_revision = "c09991fa299f1f74b04d066bf31382e34108fa82",
    source_identity = "1290a95e48b531cc15386490844178a6ed49f25d6c1355e9a29dfea972bfcc62",
    source_identity_algorithm = "blake3-of-ordered-b3sum-lines-v1",
    source_manifest = [
      { path = "crates/animus-runtime/src/applicationbinding/core.rs", blake3 = "a261c83344aec5423a2d262864cefe9d12afc5ebecf0f1ff424df5a5315d6a2c" },
      { path = "crates/animus-runtime/src/applicationbinding/data.rs", blake3 = "a3ba79d1edeb0f36ea2b9ba3baec0533c9937570a75a9e8f6c761d55c4f0e3b6" },
      { path = "crates/animus-runtime/src/applicationbinding/mod.rs", blake3 = "419ac79bd9aabaeea73312621a94c2e533e2f3bfe18b19a4be9f9f16422aa4c4" },
      { path = "config/application-host-binding-pilot/profile.ncl", blake3 = "339baed1d5854d3840c3b26415a02a1cca33e46833a1fb4a3115d9629611172e" },
      { path = "config/application-host-binding-pilot/generated/animus-binding.json", blake3 = "52ca72193a1f2e9668e0ceae3c85c16b34817dcab3d0b90532115e78d52d8e35" },
    ],
    kamacite_revision = "b21732a7458b660367f7281c717a5acc3f8c8190",
    binding = profile.binding,
    non_claims = producer_non_claims,
  },
}
