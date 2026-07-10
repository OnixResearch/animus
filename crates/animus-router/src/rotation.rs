use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryFrom;

const POLICY_FIELD: crate::FieldName = crate::FieldName::new("router_rotation_policy");
const STALE_POLICY_FIELD: crate::FieldName = crate::FieldName::new("router_rotation_stale_policy");
const CURSOR_CLASS_FIELD: crate::FieldName = crate::FieldName::new("router_rotation_cursor_class");
const MODEL_LABEL_FIELD: crate::FieldName = crate::FieldName::new("router_model_label");
const RECIPE_ID_FIELD: crate::FieldName = crate::FieldName::new("router_recipe_id");
const NEXT_CURSOR_HASH_FIELD: crate::FieldName =
    crate::FieldName::new("router_rotation_next_cursor_hash");
const EMPTY_CURSORS_FIELD: &str = "router_rotation_cursors";
const EMPTY_REDACTIONS_FIELD: &str = "router_rotation_receipt_redactions";
const CURSOR_HASH_SCHEMA: &str = "animus.router-rotation.cursor.v1";
const RECEIPT_HASH_SCHEMA: &str = "animus.router-rotation.receipt.v1";
const POLICY_DISABLED: &str = "disabled";
const POLICY_ROLE_CANDIDATES: &str = "role_candidates";
const POLICY_ROLE_CANDIDATES_AND_FALLBACK: &str = "role_candidates_and_fallback";
const STALE_RESET: &str = "reset";
const STALE_DENY: &str = "deny";
const CURSOR_DISABLED: &str = "disabled";
const CURSOR_ABSENT: &str = "absent";
const CURSOR_MATCHED: &str = "matched";
const CURSOR_WRAPPED: &str = "wrapped";
const CURSOR_STALE_RESET: &str = "stale_reset";
const FIRST_CURSOR_INDEX: usize = 0;
const CURSOR_ADVANCE: usize = 1;
const MAX_CURSOR_POSITION: u32 = u32::MAX;

/// Router round-robin policy supplied by the host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouterRotationPolicy {
    /// Rotation is disabled; catalog order remains the deterministic tie-breaker.
    Disabled,
    /// Rotate primary recipe-role candidates only.
    RoleCandidates,
    /// Rotate primary recipe-role candidates and fallback-chain candidates.
    RoleCandidatesAndFallback,
}

impl RouterRotationPolicy {
    /// Parses a stable router rotation policy label.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(POLICY_FIELD, value, |normalized| match normalized {
            POLICY_DISABLED => Some(Self::Disabled),
            POLICY_ROLE_CANDIDATES => Some(Self::RoleCandidates),
            POLICY_ROLE_CANDIDATES_AND_FALLBACK => Some(Self::RoleCandidatesAndFallback),
            _unsupported => None,
        })
    }

    /// Returns the stable policy label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => POLICY_DISABLED,
            Self::RoleCandidates => POLICY_ROLE_CANDIDATES,
            Self::RoleCandidatesAndFallback => POLICY_ROLE_CANDIDATES_AND_FALLBACK,
        }
    }

    pub(crate) const fn rotates_role(self, role: crate::Role) -> bool {
        match (self, role) {
            (Self::Disabled, _) | (Self::RoleCandidates, crate::Role::Fallback) => false,
            (Self::RoleCandidates | Self::RoleCandidatesAndFallback, _) => true,
        }
    }
}

/// Host policy for stale rotation cursors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouterRotationStaleCursorPolicy {
    /// Reset stale cursors to the first eligible candidate.
    Reset,
    /// Deny selection when a cursor's catalog hash is stale.
    Deny,
}

impl RouterRotationStaleCursorPolicy {
    /// Parses a stable stale-cursor policy label.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(STALE_POLICY_FIELD, value, |normalized| match normalized {
            STALE_RESET => Some(Self::Reset),
            STALE_DENY => Some(Self::Deny),
            _unsupported => None,
        })
    }

    /// Returns the stable stale-cursor policy label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reset => STALE_RESET,
            Self::Deny => STALE_DENY,
        }
    }
}

/// Rotation cursor classification recorded in decisions and receipts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouterRotationCursorClass {
    /// Rotation policy was disabled.
    Disabled,
    /// No matching host cursor was supplied.
    Absent,
    /// A matching host cursor was used directly.
    Matched,
    /// A matching cursor wrapped around the eligible candidate list.
    Wrapped,
    /// A stale cursor was reset by explicit host policy.
    StaleReset,
}

impl RouterRotationCursorClass {
    /// Parses a stable cursor-class label.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(CURSOR_CLASS_FIELD, value, |normalized| match normalized {
            CURSOR_DISABLED => Some(Self::Disabled),
            CURSOR_ABSENT => Some(Self::Absent),
            CURSOR_MATCHED => Some(Self::Matched),
            CURSOR_WRAPPED => Some(Self::Wrapped),
            CURSOR_STALE_RESET => Some(Self::StaleReset),
            _unsupported => None,
        })
    }

    /// Returns the stable cursor-class label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => CURSOR_DISABLED,
            Self::Absent => CURSOR_ABSENT,
            Self::Matched => CURSOR_MATCHED,
            Self::Wrapped => CURSOR_WRAPPED,
            Self::StaleReset => CURSOR_STALE_RESET,
        }
    }
}

/// Host-owned rotation cursor for one router role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouterRotationCursor {
    role: crate::Role,
    position: u32,
    catalog_hash: Option<crate::SafeHash>,
}

impl RouterRotationCursor {
    /// Builds a validated host-owned rotation cursor.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when role or hash labels are malformed.
    pub fn new(draft: RouterRotationCursorDraft<'_>) -> Result<Self, crate::AdapterContractError> {
        Ok(Self {
            role: crate::Role::parse(draft.role)?,
            position: draft.position,
            catalog_hash: draft.catalog_hash.map(crate::SafeHash::new).transpose()?,
        })
    }

    /// Returns the role this cursor applies to.
    #[must_use]
    pub const fn role(&self) -> crate::Role {
        self.role
    }

    /// Returns the next host cursor position.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// Returns the optional catalog-generation hash bound to the cursor.
    #[must_use]
    pub fn catalog_hash(&self) -> Option<&str> {
        self.catalog_hash.as_ref().map(crate::SafeHash::as_str)
    }
}

/// Borrowed fields used to validate a router rotation cursor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouterRotationCursorDraft<'a> {
    /// Role label for this cursor.
    pub role: &'a str,
    /// Zero-based next candidate position.
    pub position: u32,
    /// Optional BLAKE3 catalog generation hash.
    pub catalog_hash: Option<&'a str>,
}

/// Host-owned rotation snapshot supplied as explicit router input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouterRotationSnapshot {
    policy: RouterRotationPolicy,
    stale_cursor_policy: RouterRotationStaleCursorPolicy,
    catalog_hash: crate::SafeHash,
    cursors: Vec<RouterRotationCursor>,
}

impl RouterRotationSnapshot {
    /// Builds a validated host-owned rotation snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when labels, hashes, or cursors are malformed.
    pub fn new(
        draft: RouterRotationSnapshotDraft<'_>,
    ) -> Result<Self, crate::AdapterContractError> {
        let cursors = draft
            .cursors
            .iter()
            .map(|cursor| RouterRotationCursor::new(*cursor))
            .collect::<Result<Vec<_>, _>>()?;
        if draft.policy != POLICY_DISABLED && cursors.is_empty() {
            return Err(crate::AdapterContractError::EmptyCollection {
                field: EMPTY_CURSORS_FIELD,
            });
        }
        Ok(Self {
            policy: RouterRotationPolicy::parse(draft.policy)?,
            stale_cursor_policy: RouterRotationStaleCursorPolicy::parse(draft.stale_cursor_policy)?,
            catalog_hash: crate::SafeHash::new(draft.catalog_hash)?,
            cursors,
        })
    }

    /// Returns the active rotation policy.
    #[must_use]
    pub const fn policy(&self) -> RouterRotationPolicy {
        self.policy
    }

    /// Returns the active stale-cursor policy.
    #[must_use]
    pub const fn stale_cursor_policy(&self) -> RouterRotationStaleCursorPolicy {
        self.stale_cursor_policy
    }

    /// Returns the catalog-generation hash.
    #[must_use]
    pub fn catalog_hash(&self) -> &str {
        self.catalog_hash.as_str()
    }

    /// Returns the host-owned cursor list.
    #[must_use]
    pub fn cursors(&self) -> &[RouterRotationCursor] {
        self.cursors.as_slice()
    }

    fn cursor_for(&self, role: crate::Role) -> Option<&RouterRotationCursor> {
        self.cursors.iter().find(|cursor| cursor.role() == role)
    }
}

/// Borrowed fields used to validate a router rotation snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouterRotationSnapshotDraft<'a> {
    /// Rotation policy label.
    pub policy: &'a str,
    /// Stale cursor policy label.
    pub stale_cursor_policy: &'a str,
    /// BLAKE3 catalog generation hash.
    pub catalog_hash: &'a str,
    /// Host-owned cursors.
    pub cursors: &'a [RouterRotationCursorDraft<'a>],
}

/// Pure rotation decision for one role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouterRotationDecision {
    role: crate::Role,
    policy: RouterRotationPolicy,
    cursor_class: RouterRotationCursorClass,
    selected_model: String,
    candidate_count: u32,
    catalog_hash: crate::SafeHash,
    next_cursor: RouterRotationCursor,
    next_cursor_hash: String,
}

impl RouterRotationDecision {
    /// Returns the rotated role.
    #[must_use]
    pub const fn role(&self) -> crate::Role {
        self.role
    }

    /// Returns the rotation policy used for the role.
    #[must_use]
    pub const fn policy(&self) -> RouterRotationPolicy {
        self.policy
    }

    /// Returns the cursor class used for the decision.
    #[must_use]
    pub const fn cursor_class(&self) -> RouterRotationCursorClass {
        self.cursor_class
    }

    /// Returns the selected safe model label.
    #[must_use]
    pub fn selected_model(&self) -> &str {
        self.selected_model.as_str()
    }

    /// Returns the number of eligible candidates after filtering.
    #[must_use]
    pub const fn candidate_count(&self) -> u32 {
        self.candidate_count
    }

    /// Returns the catalog hash used for this decision.
    #[must_use]
    pub fn catalog_hash(&self) -> &str {
        self.catalog_hash.as_str()
    }

    /// Returns the next host cursor metadata.
    #[must_use]
    pub const fn next_cursor(&self) -> &RouterRotationCursor {
        &self.next_cursor
    }

    /// Returns the deterministic next-cursor BLAKE3 identity.
    #[must_use]
    pub fn next_cursor_hash(&self) -> &str {
        self.next_cursor_hash.as_str()
    }
}

/// Typed rotation validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RouterRotationError {
    /// Generic adapter contract validation failed.
    Contract(crate::AdapterContractError),
    /// No eligible candidates were supplied to the rotation core.
    NoEligibleCandidates,
    /// Cursor catalog generation mismatched and policy denied reset.
    StaleCursor,
    /// Receipt candidate included unsafe material.
    ReceiptLeak(crate::ReceiptLeak),
}

impl From<crate::AdapterContractError> for RouterRotationError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

/// Rotates already eligible candidates for one role using explicit host cursor state.
///
/// # Errors
///
/// Returns [`RouterRotationError`] when no eligible candidates exist or stale cursor policy denies reset.
pub fn rotate_eligible_candidates(
    role: crate::Role,
    candidates: &[String],
    snapshot: &RouterRotationSnapshot,
) -> Result<RouterRotationDecision, RouterRotationError> {
    if candidates.is_empty() {
        return Err(RouterRotationError::NoEligibleCandidates);
    }

    let (selected_index, cursor_class) = select_index(role, candidates.len(), snapshot)?;
    let selected_model = candidates[selected_index].clone();
    let next_index = next_index(selected_index, candidates.len());
    let next_position = u32::try_from(next_index).unwrap_or(MAX_CURSOR_POSITION);
    let next_cursor = RouterRotationCursor {
        role,
        position: next_position,
        catalog_hash: Some(snapshot.catalog_hash.clone()),
    };
    let next_cursor_hash = cursor_hash(&next_cursor, &snapshot.catalog_hash);
    let candidate_count = u32::try_from(candidates.len()).unwrap_or(MAX_CURSOR_POSITION);

    Ok(RouterRotationDecision {
        role,
        policy: snapshot.policy(),
        cursor_class,
        selected_model,
        candidate_count,
        catalog_hash: snapshot.catalog_hash.clone(),
        next_cursor,
        next_cursor_hash,
    })
}

fn select_index(
    role: crate::Role,
    candidate_count: usize,
    snapshot: &RouterRotationSnapshot,
) -> Result<(usize, RouterRotationCursorClass), RouterRotationError> {
    if !snapshot.policy().rotates_role(role) {
        return Ok((FIRST_CURSOR_INDEX, RouterRotationCursorClass::Disabled));
    }

    let Some(cursor) = snapshot.cursor_for(role) else {
        return Ok((FIRST_CURSOR_INDEX, RouterRotationCursorClass::Absent));
    };

    if cursor
        .catalog_hash()
        .is_some_and(|hash| hash != snapshot.catalog_hash())
    {
        if snapshot.stale_cursor_policy() == RouterRotationStaleCursorPolicy::Deny {
            return Err(RouterRotationError::StaleCursor);
        }
        return Ok((FIRST_CURSOR_INDEX, RouterRotationCursorClass::StaleReset));
    }

    let raw_index = cursor.position() as usize;
    if raw_index >= candidate_count {
        return Ok((
            raw_index % candidate_count,
            RouterRotationCursorClass::Wrapped,
        ));
    }

    Ok((raw_index, RouterRotationCursorClass::Matched))
}

fn next_index(selected_index: usize, candidate_count: usize) -> usize {
    (selected_index + CURSOR_ADVANCE) % candidate_count
}

fn cursor_hash(cursor: &RouterRotationCursor, catalog_hash: &crate::SafeHash) -> String {
    let position = cursor.position().to_string();
    crate::hash_parts(&[
        CURSOR_HASH_SCHEMA,
        cursor.role().as_str(),
        position.as_str(),
        catalog_hash.as_str(),
    ])
}

/// Safe rotation receipt for router fairness evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouterRotationReceipt {
    recipe_id: String,
    route_family: crate::RouteFamily,
    role: crate::Role,
    policy: RouterRotationPolicy,
    cursor_class: RouterRotationCursorClass,
    selected_model: String,
    candidate_count: u32,
    catalog_hash: crate::SafeHash,
    next_cursor_hash: crate::SafeHash,
    redactions: Vec<crate::Redaction>,
    identity_hash: String,
}

impl RouterRotationReceipt {
    /// Builds a safe rotation receipt and deterministic BLAKE3 identity.
    ///
    /// # Errors
    ///
    /// Returns [`RouterRotationError`] when labels, hashes, redactions, or leak checks fail.
    pub fn new(draft: RouterRotationReceiptDraft<'_>) -> Result<Self, RouterRotationError> {
        let recipe_id = safe_receipt_label(RECIPE_ID_FIELD, draft.recipe_id)?;
        let route_family = crate::RouteFamily::parse(draft.route_family)?;
        let role = crate::Role::parse(draft.role)?;
        let policy = RouterRotationPolicy::parse(draft.policy)?;
        let cursor_class = RouterRotationCursorClass::parse(draft.cursor_class)?;
        let selected_model = safe_receipt_label(MODEL_LABEL_FIELD, draft.selected_model)?;
        let catalog_hash = crate::SafeHash::new(draft.catalog_hash)?;
        let next_cursor_hash = parse_receipt_hash(NEXT_CURSOR_HASH_FIELD, draft.next_cursor_hash)?;
        let redactions = collect_redactions(draft.redactions)?;
        let candidate_count = draft.candidate_count.to_string();
        let identity_hash = crate::hash_parts(&[
            RECEIPT_HASH_SCHEMA,
            recipe_id.as_str(),
            route_family.as_str(),
            role.as_str(),
            policy.as_str(),
            cursor_class.as_str(),
            selected_model.as_str(),
            candidate_count.as_str(),
            catalog_hash.as_str(),
            next_cursor_hash.as_str(),
        ]);

        Ok(Self {
            recipe_id,
            route_family,
            role,
            policy,
            cursor_class,
            selected_model,
            candidate_count: draft.candidate_count,
            catalog_hash,
            next_cursor_hash,
            redactions,
            identity_hash,
        })
    }

    /// Builds a safe receipt from a rotation decision.
    ///
    /// # Errors
    ///
    /// Returns [`RouterRotationError`] when receipt metadata fails validation.
    pub fn from_decision(
        recipe_id: &str,
        route_family: crate::RouteFamily,
        decision: &RouterRotationDecision,
        redactions: &[&str],
    ) -> Result<Self, RouterRotationError> {
        Self::new(RouterRotationReceiptDraft {
            recipe_id,
            route_family: route_family.as_str(),
            role: decision.role().as_str(),
            policy: decision.policy().as_str(),
            cursor_class: decision.cursor_class().as_str(),
            selected_model: decision.selected_model(),
            candidate_count: decision.candidate_count(),
            catalog_hash: decision.catalog_hash(),
            next_cursor_hash: decision.next_cursor_hash(),
            redactions,
        })
    }

    /// Returns recipe identifier.
    #[must_use]
    pub fn recipe_id(&self) -> &str {
        self.recipe_id.as_str()
    }

    /// Returns route family.
    #[must_use]
    pub const fn route_family(&self) -> crate::RouteFamily {
        self.route_family
    }

    /// Returns role.
    #[must_use]
    pub const fn role(&self) -> crate::Role {
        self.role
    }

    /// Returns rotation policy.
    #[must_use]
    pub const fn policy(&self) -> RouterRotationPolicy {
        self.policy
    }

    /// Returns cursor class.
    #[must_use]
    pub const fn cursor_class(&self) -> RouterRotationCursorClass {
        self.cursor_class
    }

    /// Returns selected safe model label.
    #[must_use]
    pub fn selected_model(&self) -> &str {
        self.selected_model.as_str()
    }

    /// Returns eligible candidate count.
    #[must_use]
    pub const fn candidate_count(&self) -> u32 {
        self.candidate_count
    }

    /// Returns catalog hash.
    #[must_use]
    pub fn catalog_hash(&self) -> &str {
        self.catalog_hash.as_str()
    }

    /// Returns next-cursor hash.
    #[must_use]
    pub fn next_cursor_hash(&self) -> &str {
        self.next_cursor_hash.as_str()
    }

    /// Returns redaction markers.
    #[must_use]
    pub fn redactions(&self) -> &[crate::Redaction] {
        self.redactions.as_slice()
    }

    /// Returns deterministic BLAKE3 receipt identity.
    #[must_use]
    pub fn identity_hash(&self) -> &str {
        self.identity_hash.as_str()
    }
}

/// Borrowed fields used to build a rotation receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouterRotationReceiptDraft<'a> {
    /// Recipe identifier.
    pub recipe_id: &'a str,
    /// Route-family label.
    pub route_family: &'a str,
    /// Role label.
    pub role: &'a str,
    /// Rotation policy label.
    pub policy: &'a str,
    /// Cursor-class label.
    pub cursor_class: &'a str,
    /// Safe selected model label.
    pub selected_model: &'a str,
    /// Bounded eligible-candidate count.
    pub candidate_count: u32,
    /// BLAKE3 catalog-generation hash.
    pub catalog_hash: &'a str,
    /// BLAKE3 next-cursor hash.
    pub next_cursor_hash: &'a str,
    /// Redaction markers.
    pub redactions: &'a [&'a str],
}

fn safe_receipt_label(field: crate::FieldName, value: &str) -> Result<String, RouterRotationError> {
    crate::validate_receipt_text(value).map_err(RouterRotationError::ReceiptLeak)?;
    crate::normalized_adapter_field(field, value).map_err(RouterRotationError::from)
}

fn parse_receipt_hash(
    _field: crate::FieldName,
    value: &str,
) -> Result<crate::SafeHash, RouterRotationError> {
    crate::validate_receipt_text(value).map_err(RouterRotationError::ReceiptLeak)?;
    crate::SafeHash::new(value).map_err(RouterRotationError::from)
}

fn collect_redactions(values: &[&str]) -> Result<Vec<crate::Redaction>, RouterRotationError> {
    if values.is_empty() {
        return Err(RouterRotationError::Contract(
            crate::AdapterContractError::EmptyCollection {
                field: EMPTY_REDACTIONS_FIELD,
            },
        ));
    }
    values
        .iter()
        .map(|value| crate::Redaction::parse(value).map_err(RouterRotationError::from))
        .collect()
}
