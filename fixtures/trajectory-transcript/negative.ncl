let Blake3Hex = std.contract.from_predicate (fun value => std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value) in
({ helper_blake3 = "not-an-identity" } | { helper_blake3 | Blake3Hex })
