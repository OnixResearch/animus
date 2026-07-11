//! Review-only Preserves fixture declarations.
//!
//! These descriptors do not publish an encoder, decoder, compatibility import,
//! or stable external wire claim.

/// Returns the review-only dataset/export schema descriptor.
#[must_use]
pub fn dataset_export_descriptor() -> crate::SchemaDescriptor {
    crate::schema_catalog()[crate::SchemaFamily::DatasetExportRow as usize]
}

#[cfg(test)]
mod tests {
    #[test]
    fn dataset_export_remains_review_only_without_stable_fixture_claims() {
        let descriptor = super::dataset_export_descriptor();

        assert_eq!(descriptor.wire_status(), crate::WireStatus::ReviewOnly);
        assert!(!descriptor.coverage_tier().requires_stable_fixtures());
    }
}
