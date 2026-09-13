//! Checked field framing and shared admission predicates.

pub(super) fn validate_identity(value: &str) -> Result<(), super::data::ChangeHandoffError> {
    if is_blake3_identity(value) {
        Ok(())
    } else {
        Err(error(super::data::ChangeHandoffIssue::InvalidIdentity))
    }
}

pub(super) const fn error(
    issue: super::data::ChangeHandoffIssue,
) -> super::data::ChangeHandoffError {
    super::data::ChangeHandoffError::new(issue)
}

pub(super) fn is_blake3_identity(value: &str) -> bool {
    const BLAKE3_HEX_CHARS: usize = 64;
    value.len() == BLAKE3_HEX_CHARS
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

pub(super) fn is_within_bound(count: usize, bound: u32) -> bool {
    u32::try_from(count).is_ok_and(|count| count <= bound)
}

pub(super) fn hash_fields(
    domain: &str,
    fields: &[&[u8]],
) -> Result<String, super::data::ChangeHandoffError> {
    let mut hasher = blake3::Hasher::new();
    hash_field(&mut hasher, domain.as_bytes())?;
    for field in fields {
        hash_field(&mut hasher, field)?;
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn hash_field(
    hasher: &mut blake3::Hasher,
    field: &[u8],
) -> Result<(), super::data::ChangeHandoffError> {
    let byte_count = u64::try_from(field.len())
        .map_err(|_overflow| error(super::data::ChangeHandoffIssue::InvalidIdentity))?;
    hasher.update(&byte_count.to_le_bytes());
    hasher.update(field);
    Ok(())
}
