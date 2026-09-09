use alloc::string::ToString;

/// Validated lowercase BLAKE3 identity.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ArtifactHash {
    value: alloc::string::String,
}

impl ArtifactHash {
    /// Builds a hash from validated lowercase BLAKE3 hex text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when the value is not lowercase BLAKE3 hex.
    pub fn new(value: &str) -> Result<Self, crate::Error> {
        validate_hash(value)?;
        Ok(Self {
            value: value.to_string(),
        })
    }

    /// Builds a hash from raw BLAKE3 bytes.
    ///
    /// # Panics
    ///
    /// Panics if the fixed-size BLAKE3 byte-to-hex encoder violates its nibble
    /// table or output-length invariants.
    #[must_use]
    pub fn from_bytes(bytes: &[u8; crate::BLAKE3_HASH_BYTES]) -> Self {
        let mut value = alloc::string::String::with_capacity(crate::BLAKE3_HASH_HEX_CHARS);
        for byte in bytes {
            let high = usize::from(byte >> crate::HEX_NIBBLE_SHIFT);
            let low = usize::from(byte & crate::HEX_LOW_NIBBLE_MASK);
            assert!(
                high < crate::HEX_RADIX,
                "high hex nibble must fit hex table"
            );
            assert!(low < crate::HEX_RADIX, "low hex nibble must fit hex table");
            value.push(char::from(crate::LOWER_HEX[high]));
            value.push(char::from(crate::LOWER_HEX[low]));
        }
        assert_eq!(
            value.len(),
            crate::BLAKE3_HASH_HEX_CHARS,
            "BLAKE3 hex encoding must produce the configured length"
        );
        Self { value }
    }

    /// Returns the validated hash text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl core::fmt::Display for ArtifactHash {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl core::str::FromStr for ArtifactHash {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// Animus hash admission uses non-const Unicode iterators; preserve the existing validation order.
#[cfg_attr(dylint_lib = "octet", allow(tigerstyle::missing_const_fn))]
fn validate_hash(value: &str) -> Result<(), crate::Error> {
    let actual_chars = value.chars().count();
    assert!(
        actual_chars <= value.len(),
        "hash char count must fit byte length"
    );
    if actual_chars != crate::BLAKE3_HASH_HEX_CHARS
        || !value
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        return Err(crate::Error::InvalidHash {
            field: crate::FIELD_ARTIFACT_HASH,
            reason: crate::HASH_LENGTH_REASON,
        });
    }
    Ok(())
}
