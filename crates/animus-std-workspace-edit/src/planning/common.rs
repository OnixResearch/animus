pub(crate) const BLAKE3_HEX_CHARS: usize = 64;
pub(crate) const FILE_RESOURCE_CLASS: &str = "file";
pub(crate) const FRAME_DOMAIN_SNAPSHOT: &[u8] = b"animus.workspace-edit.snapshot.v1";
pub(crate) const FRAME_DOMAIN_VISIBILITY: &[u8] = b"animus.workspace-edit.visibility.v1";
pub(crate) const FRAME_DOMAIN_REPLACEMENT: &[u8] = b"animus.workspace-edit.replacement.v1";
pub(crate) const FRAME_DOMAIN_PLAN: &[u8] = b"animus.workspace-edit.plan.v1";
pub(crate) const KIND_REPLACE: &str = "replace";
pub(crate) const KIND_DELETE: &str = "delete";
pub(crate) const KIND_INSERT: &str = "insert";
pub(crate) const KIND_INSERT_END: &str = "insert-end";
pub(crate) const ADJACENT_OPERATION_WINDOW: usize = 2;

#[derive(Clone, Copy)]
pub(crate) struct IdentityInput<'a> {
    pub(crate) field: &'static str,
    pub(crate) value: &'a str,
}

fn is_blake3_identity(value: &str) -> bool {
    value.len() == BLAKE3_HEX_CHARS
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

pub(crate) fn require_identity(
    input: IdentityInput<'_>,
) -> Result<String, crate::model::HostIssue> {
    if !is_blake3_identity(input.value) {
        return Err(crate::model::HostIssue::InvalidIdentity(input.field));
    }
    let identity = String::from(input.value);
    debug_assert_eq!(identity.len(), BLAKE3_HEX_CHARS);
    Ok(identity)
}

pub(crate) fn hash_bytes(bytes: &[u8]) -> String {
    let identity = blake3::hash(bytes).to_hex().to_string();
    debug_assert_eq!(identity.len(), BLAKE3_HEX_CHARS);
    identity
}

pub(crate) fn frame_part(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

pub(crate) fn hash_parts(domain: &[u8], parts: &[&[u8]]) -> String {
    let mut hasher = blake3::Hasher::new();
    frame_part(&mut hasher, domain);
    for part in parts {
        frame_part(&mut hasher, part);
    }
    let identity = hasher.finalize().to_hex().to_string();
    debug_assert_eq!(identity.len(), BLAKE3_HEX_CHARS);
    identity
}
