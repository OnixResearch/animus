//! Private nominal identities for the ACP host.

#![allow(
    dead_code,
    reason = "closed nominal roles include constructors used by compile-time conformance fixtures"
)]

use crate::model::{ACP_HASH_HEX_BYTES, ACP_MAX_SAFE_LABEL_BYTES};

// r[impl agent_harness_runtime.acp_nominal_identities.types]
// r[impl agent_harness_runtime.acp_nominal_identities.validation]

/// Deterministic identity admission issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AcpIdentityIssue {
    Empty,
    Oversized,
    Malformed,
    UnsupportedDigest,
}

macro_rules! external_identity {
    ($name:ident) => {
        #[doc = concat!("Nominal external identity role `", stringify!($name), "`.")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub(crate) fn parse(value: impl Into<String>) -> Result<Self, AcpIdentityIssue> {
                let value = value.into();
                validate_external(value.as_str())?;
                Ok(Self(value))
            }

            pub(crate) fn from_admitted(value: String) -> Self {
                debug_assert!(validate_external(value.as_str()).is_ok());
                Self(value)
            }

            pub(crate) fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[cfg(feature = "nominal-compile-fixtures")]
            pub(crate) fn fixture(value: &'static str) -> Self {
                Self(value.to_string())
            }
        }

        impl core::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }
    };
}

macro_rules! blake3_identity {
    ($name:ident) => {
        #[doc = concat!("Nominal BLAKE3 identity role `", stringify!($name), "`.")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub(crate) fn parse(value: impl Into<String>) -> Result<Self, AcpIdentityIssue> {
                let value = value.into();
                validate_blake3(value.as_str())?;
                Ok(Self(value))
            }

            pub(crate) fn from_domain_hash(value: String) -> Self {
                debug_assert!(validate_blake3(value.as_str()).is_ok());
                Self(value)
            }

            pub(crate) fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[cfg(feature = "nominal-compile-fixtures")]
            pub(crate) fn fixture(value: &'static str) -> Self {
                Self(value.to_string())
            }
        }

        impl core::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }
    };
}

external_identity!(AcpCorrelationId);
external_identity!(AcpSessionId);
external_identity!(AcpRunId);
external_identity!(AcpEntryId);
external_identity!(SessionReference);
blake3_identity!(AnimusSessionId);
blake3_identity!(SessionIdentity);
blake3_identity!(AuthorityProfileIdentity);
blake3_identity!(AcpConnectionIdentity);
blake3_identity!(WorkspaceIdentity);
blake3_identity!(ResourceIdentity);
blake3_identity!(ReceiptIdentity);

fn validate_external(value: &str) -> Result<(), AcpIdentityIssue> {
    if value.is_empty() {
        return Err(AcpIdentityIssue::Empty);
    }
    if value.len() > ACP_MAX_SAFE_LABEL_BYTES {
        return Err(AcpIdentityIssue::Oversized);
    }
    if value.contains('\0') || value.chars().any(char::is_whitespace) {
        return Err(AcpIdentityIssue::Malformed);
    }
    Ok(())
}

fn validate_blake3(value: &str) -> Result<(), AcpIdentityIssue> {
    if value.is_empty() {
        return Err(AcpIdentityIssue::Empty);
    }
    if value.len() != ACP_HASH_HEX_BYTES {
        return Err(AcpIdentityIssue::UnsupportedDigest);
    }
    if !value.bytes().all(|byte| byte.is_ascii_hexdigit())
        || value.bytes().any(|byte| byte.is_ascii_uppercase())
    {
        return Err(AcpIdentityIssue::Malformed);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const OVERSIZED_EXTERNAL_BYTES: usize = ACP_MAX_SAFE_LABEL_BYTES + 1;

    #[test]
    fn same_role_external_and_blake3_identities_are_accepted() {
        // r[verify agent_harness_runtime.acp_nominal_identities.types.same_role]
        assert_eq!(
            AcpSessionId::parse("session.one")
                .expect("session")
                .as_str(),
            "session.one"
        );
        assert_eq!(SessionIdentity::parse(HASH).expect("hash").as_str(), HASH);
    }

    #[test]
    fn empty_oversized_malformed_and_unsupported_values_are_rejected() {
        // r[verify agent_harness_runtime.acp_nominal_identities.validation]
        assert_eq!(AcpSessionId::parse(""), Err(AcpIdentityIssue::Empty));
        assert_eq!(
            AcpSessionId::parse("x".repeat(OVERSIZED_EXTERNAL_BYTES)),
            Err(AcpIdentityIssue::Oversized)
        );
        assert_eq!(
            AcpSessionId::parse("session with space"),
            Err(AcpIdentityIssue::Malformed)
        );
        assert_eq!(
            SessionIdentity::parse("abcd"),
            Err(AcpIdentityIssue::UnsupportedDigest)
        );
        assert_eq!(
            SessionIdentity::parse(HASH.to_uppercase()),
            Err(AcpIdentityIssue::Malformed)
        );
    }

    #[test]
    fn nominal_roles_remain_distinct_at_compile_time() {
        // The distinct concrete types are also exercised by the focused rustc fixtures.
        fn needs_session(_value: AcpSessionId) {}
        needs_session(AcpSessionId::parse("session.one").expect("session"));
        let _animus = AnimusSessionId::parse(HASH).expect("animus");
    }
}
