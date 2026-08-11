let GitRevision = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{40}$" value
) in
let Blake3 = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let RadicleRid = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^rad:z[1-9A-HJ-NP-Za-km-z]+$" value
) in
let SourceFile = {
  path | String,
  blake3 | Blake3,
} in
let Mechanism = {
  package | String,
  rid | RadicleRid,
  revision | GitRevision,
  license | String,
  vendor_path | String,
  files | Array SourceFile,
  non_claims | Array String,
} in
{
  schema = "animus.workspace-edit-mechanisms.v1",
  import_tool = "tools/sync-workspace-edit-mechanisms.rs",
  mechanisms | Array Mechanism = [
    {
      package = "cap-root-core",
      rid = "rad:z3cWBCK7VPbEmbL5MAvKJWKjCE745",
      revision = "8991e2572578c16e590c2f1c0915db6c46c5d9cb",
      license = "AGPL-3.0-or-later",
      vendor_path = "vendor/cap-root-core-selected",
      files = [
        { path = "Cargo.toml", blake3 = "d5ad34b0be84392e2e0f8d2cbee793ece6d5d3ebe2ce4aeb7184de9d0a4686b4" },
        { path = "crates/cap-root-core/Cargo.toml", blake3 = "9b4df619e1593b8b156046267bf6c5056dd62909ef699eb802e49b68e213e48f" },
        { path = "crates/cap-root-core/src/lib.rs", blake3 = "60fbdb7eae53dff8c2c0d41bc15aa123cd3a362ec1611eb4bc892fd6be58f3cb" },
        { path = "LICENSE", blake3 = "ca0292789c0a0ecba706f4b3f3493007a9951731c1d432c52c4aac1f1abba8ef" },
      ],
      non_claims = [
        "lexical admission is not root authority",
        "lexical admission is not no-follow filesystem I/O",
      ],
    },
    {
      package = "durable-file-publication",
      rid = "rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM",
      revision = "c57f01d9d80f27d4893961d75a31bf9c95d1f6c8",
      license = "MIT OR Apache-2.0",
      vendor_path = "vendor/durable-file-publication-selected",
      files = [
        { path = "Cargo.toml", blake3 = "9f0481a101e4f9fb40037ba1769f401e085420e3309371025aa14b824bcb60ce" },
        { path = "src/lib.rs", blake3 = "4bcb234a0d2ffb1c202a19ae41ef5a6b9529124abb9bbe002c2263049391d129" },
        { path = "src/core.rs", blake3 = "4d040eb172f0fc518f5fa662c1ffa0e4457319921e39fb7a6c1f76d994665469" },
        { path = "src/corpus.rs", blake3 = "d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da" },
        { path = "src/shell.rs", blake3 = "3fcf209f905c47aac62c30d97e7ae0dfb276949d2eebe68b1298c76d5b0bc05e" },
        { path = "LICENSE", blake3 = "7d9de1981c9bfba7919625c3c5716cb37db08ceda1b344df84253439de45c204" },
        { path = "LICENSE-APACHE", blake3 = "83cb3a2fcf829b6138e095b083016c34ddcdfa07b68d38782722c14fcf85ace6" },
        { path = "LICENSE-MIT", blake3 = "4832be0355265c184eb7a2fc7f24e4d22d353bca2a83f630416ea9c54f994262" },
      ],
      non_claims = [
        "one-file publication is not product authorization",
        "parent synchronization is not storage-device correctness",
        "rename visibility is not rollback evidence",
      ],
    },
  ],
}
