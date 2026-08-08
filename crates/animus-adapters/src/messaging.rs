//! Pure family-scoped messaging, routing, limit, and receipt contracts.
//!
//! Hosts derive identities, provide roster and counter snapshots, and perform
//! transport or queue effects. This module never opens a transport, writes a
//! queue, persists content, or treats message text as identity authority.

use alloc::{format, string::String, vec, vec::Vec};

/// The reviewed messaging schema.
pub const MESSAGING_SCHEMA: &str = "animus.messaging-receipts.v1";
/// The reviewed Prime Agent messaging cohort.
pub const REVIEWED_MESSAGING_COHORT: &str = "prime-agent-messaging-receipts";
/// The reviewed cohort license.
pub const REVIEWED_MESSAGING_LICENSE: &str = "MIT";
/// The exact reviewed Prime Agent revision.
pub const REVIEWED_MESSAGING_REVISION: &str = "c5991bc853d27754aed345c13ff4d2e05c40f1f4";
/// The policy identity domain.
pub const MESSAGING_POLICY_DOMAIN: &str = "animus.messaging-policy.v1";
/// The roster identity domain.
pub const MESSAGING_ROSTER_DOMAIN: &str = "animus.messaging-roster.v1";
/// The receipt identity domain.
pub const MESSAGING_RECEIPT_DOMAIN: &str = "animus.messaging-receipt.v1";
/// The BLAKE3 hexadecimal identity length.
pub const MESSAGING_BLAKE3_HEX_CHARS: usize = 64;
/// The minimum accepted message sequence.
pub const MESSAGING_MIN_SEQUENCE: u64 = 1;
/// The required safe non-claim for delivery.
pub const MESSAGING_NON_CLAIM: &str = "not-read-or-acted";
/// The required transport non-claim.
pub const MESSAGING_TRANSPORT_NON_CLAIM: &str = "not-transport-security";
/// The required ordering non-claim.
pub const MESSAGING_ORDERING_NON_CLAIM: &str = "not-exactly-once-or-ordered";

const HASH_SEPARATOR: u8 = 0;
const MAX_LABEL_BYTES: usize = 256;
const MAX_ROSTER_MEMBERS: usize = 256;
const MAX_TARGETS: usize = 256;

/// A family member role used for policy mode admission.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MessagingRole {
    /// The root agent or primary session.
    Root,
    /// A retained or active subagent.
    Subagent,
}

impl MessagingRole {
    /// Returns the stable role label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Root => "root",
            Self::Subagent => "subagent",
        }
    }
}

/// The target state supplied by the host.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TargetState {
    /// Target can receive a message in its current context.
    Idle,
    /// Target has active work.
    Busy,
    /// Target is not routable.
    Unavailable,
}

impl TargetState {
    /// Returns the stable target-state label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Busy => "busy",
            Self::Unavailable => "unavailable",
        }
    }
}

/// A family roster member. Identity is host-derived.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RosterMember {
    /// Host-attested member identity.
    pub identity: String,
    /// Family identity to which the member belongs.
    pub family_id: String,
    /// Host-attested member role.
    pub role: MessagingRole,
    /// Host-supplied target state.
    pub state: TargetState,
}

/// A canonical family roster snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FamilyRoster {
    /// Host-attested family identity.
    pub family_id: String,
    /// Members sorted by identity.
    pub members: Vec<RosterMember>,
    /// Domain-separated roster identity.
    pub identity: String,
}

impl FamilyRoster {
    /// Creates a canonical roster and rejects duplicate or cross-family members.
    ///
    /// # Errors
    ///
    /// Returns an error when an identity, family, role, or roster identity is invalid.
    pub fn new(family_id: &str, mut members: Vec<RosterMember>) -> Result<Self, MessagingError> {
        validate_label(family_id)?;
        if members.is_empty() || members.len() > MAX_ROSTER_MEMBERS {
            return Err(MessagingError::InvalidRoster);
        }
        for member in &members {
            validate_label(&member.identity)?;
            validate_label(&member.family_id)?;
            if member.family_id != family_id {
                return Err(MessagingError::CrossFamilyTarget);
            }
        }
        members.sort_by(|left, right| left.identity.cmp(&right.identity));
        if members
            .windows(2)
            .any(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(MessagingError::DuplicateRosterMember);
        }
        let identity = digest(
            MESSAGING_ROSTER_DOMAIN,
            &canonical_roster(family_id, &members),
        );
        Ok(Self {
            family_id: family_id.into(),
            members,
            identity,
        })
    }

    /// Returns one member by host-attested identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the identity is not in this family roster.
    pub fn member(&self, identity: &str) -> Result<&RosterMember, MessagingError> {
        self.members
            .iter()
            .find(|member| member.identity == identity)
            .ok_or(MessagingError::UnknownTarget)
    }

    /// Resolves a family-scoped broadcast, excluding the sender.
    ///
    /// # Errors
    ///
    /// Returns an error when the sender is not in this family roster.
    pub fn broadcast_targets(&self, sender_identity: &str) -> Result<Vec<String>, MessagingError> {
        self.member(sender_identity)?;
        let targets = self
            .members
            .iter()
            .filter(|member| member.identity != sender_identity)
            .map(|member| member.identity.clone())
            .collect::<Vec<_>>();
        if targets.is_empty() || targets.len() > MAX_TARGETS {
            return Err(MessagingError::EmptyBroadcast);
        }
        Ok(targets)
    }

    /// Recomputes the roster identity to detect stale or forged snapshots.
    #[must_use]
    pub fn identity_matches(&self) -> bool {
        digest(
            MESSAGING_ROSTER_DOMAIN,
            &canonical_roster(&self.family_id, &self.members),
        ) == self.identity
    }
}

/// A host-supplied message draft.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageDraft {
    /// Target identity, ignored when `broadcast` is true.
    pub target: Option<String>,
    /// Whether to resolve all other members in the family.
    pub broadcast: bool,
    /// Delivery mode selected by the caller.
    pub mode: DeliveryMode,
    /// Message content, never used as identity authority.
    pub body: String,
    /// A model-claimed sender, rejected when present.
    pub claimed_sender: Option<String>,
}

/// A validated message whose sender is supplied separately by the host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    /// Target identity, if this is not a broadcast.
    pub target: Option<String>,
    /// Whether the message resolves to a family broadcast.
    pub broadcast: bool,
    /// Delivery mode.
    pub mode: DeliveryMode,
    /// Message content held only for host delivery, never receipts.
    pub body: String,
}

impl Message {
    /// Validates a message draft without deriving sender identity from content.
    ///
    /// # Errors
    ///
    /// Returns an error when the draft claims a sender or violates the policy.
    pub fn new(draft: MessageDraft, policy: &MessagingPolicy) -> Result<Self, MessagingError> {
        if draft.claimed_sender.is_some() {
            return Err(MessagingError::SpoofedSenderIdentity);
        }
        if draft.body.is_empty() || draft.body.len() > policy.max_message_bytes {
            return Err(MessagingError::MessageSizeExceeded);
        }
        if draft.broadcast == draft.target.is_some() {
            return Err(MessagingError::InvalidTargetShape);
        }
        Ok(Self {
            target: draft.target,
            broadcast: draft.broadcast,
            mode: draft.mode,
            body: draft.body,
        })
    }
}

/// Delivery mode selected by the caller.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DeliveryMode {
    /// Deliver to idle targets and steer busy targets.
    Auto,
    /// Inject into active work.
    Steer,
    /// Enqueue until the current turn finishes.
    FollowUp,
}

impl DeliveryMode {
    /// Returns the stable mode label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Steer => "steer",
            Self::FollowUp => "follow_up",
        }
    }
}

/// Per-role mode admission policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoleModePolicy {
    /// Role governed by this row.
    pub role: MessagingRole,
    /// Whether auto is admitted.
    pub auto: bool,
    /// Whether steer is admitted.
    pub steer: bool,
    /// Whether follow-up is admitted.
    pub follow_up: bool,
}

impl RoleModePolicy {
    fn allows(self, mode: DeliveryMode) -> bool {
        match mode {
            DeliveryMode::Auto => self.auto,
            DeliveryMode::Steer => self.steer,
            DeliveryMode::FollowUp => self.follow_up,
        }
    }
}

/// A Nickel-exported messaging policy draft.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessagingPolicyDraft {
    /// Reviewed source cohort.
    pub cohort: String,
    /// Reviewed source revision.
    pub source_revision: String,
    /// Maximum UTF-8 body bytes.
    pub max_message_bytes: usize,
    /// Maximum sends represented by one rate counter.
    pub max_rate_count: u32,
    /// Maximum pending messages represented by one queue counter.
    pub max_pending_count: u32,
    /// Per-role mode rows.
    pub role_modes: Vec<RoleModePolicy>,
    /// Required receipt fields.
    pub receipt_fields: Vec<String>,
    /// Required non-claims.
    pub non_claims: Vec<String>,
}

/// An admitted messaging policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessagingPolicy {
    /// Reviewed source cohort.
    pub cohort: String,
    /// Reviewed source revision.
    pub source_revision: String,
    /// Maximum UTF-8 body bytes.
    pub max_message_bytes: usize,
    /// Maximum sends represented by one rate counter.
    pub max_rate_count: u32,
    /// Maximum pending messages represented by one queue counter.
    pub max_pending_count: u32,
    /// Per-role mode rows.
    pub role_modes: Vec<RoleModePolicy>,
    /// Required receipt fields.
    pub receipt_fields: Vec<String>,
    /// Required non-claims.
    pub non_claims: Vec<String>,
    /// Domain-separated policy identity.
    pub identity: String,
}

impl MessagingPolicy {
    /// Validates a generated policy and derives its identity.
    ///
    /// # Errors
    ///
    /// Returns an error when policy fields, modes, non-claims, or identity are invalid.
    pub fn new(draft: MessagingPolicyDraft) -> Result<Self, MessagingError> {
        validate_label(&draft.cohort)?;
        validate_label(&draft.source_revision)?;
        if draft.cohort != REVIEWED_MESSAGING_COHORT
            || draft.source_revision != REVIEWED_MESSAGING_REVISION
            || draft.max_message_bytes == 0
            || draft.max_rate_count == 0
            || draft.max_pending_count == 0
            || draft.role_modes.len() != 2
            || draft.receipt_fields.len() < 7
            || draft.non_claims.len() < 4
            || !draft
                .non_claims
                .iter()
                .any(|claim| claim == MESSAGING_NON_CLAIM)
            || !draft
                .non_claims
                .iter()
                .any(|claim| claim == MESSAGING_ORDERING_NON_CLAIM)
        {
            return Err(MessagingError::PolicyRejected);
        }
        let mut roles = draft
            .role_modes
            .iter()
            .map(|row| row.role)
            .collect::<Vec<_>>();
        roles.sort();
        if roles.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(MessagingError::PolicyRejected);
        }
        for field in draft.receipt_fields.iter().chain(draft.non_claims.iter()) {
            validate_label(field)?;
        }
        let identity_material = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            draft.cohort,
            draft.source_revision,
            draft.max_message_bytes,
            draft.max_rate_count,
            draft.max_pending_count,
            canonical_modes(&draft.role_modes),
            join(&draft.receipt_fields),
            join(&draft.non_claims),
        );
        Ok(Self {
            cohort: draft.cohort,
            source_revision: draft.source_revision,
            max_message_bytes: draft.max_message_bytes,
            max_rate_count: draft.max_rate_count,
            max_pending_count: draft.max_pending_count,
            role_modes: draft.role_modes,
            receipt_fields: draft.receipt_fields,
            non_claims: draft.non_claims,
            identity: digest(MESSAGING_POLICY_DOMAIN, &identity_material),
        })
    }

    fn role_mode(&self, role: MessagingRole) -> Result<RoleModePolicy, MessagingError> {
        self.role_modes
            .iter()
            .copied()
            .find(|row| row.role == role)
            .ok_or(MessagingError::PolicyRejected)
    }
}

/// Builds the reviewed messaging policy used by fixtures.
#[must_use]
///
/// # Panics
///
/// Panics only if the checked-in reviewed policy constants are inconsistent.
pub fn reviewed_messaging_policy() -> MessagingPolicy {
    MessagingPolicy::new(MessagingPolicyDraft {
        cohort: REVIEWED_MESSAGING_COHORT.into(),
        source_revision: REVIEWED_MESSAGING_REVISION.into(),
        max_message_bytes: 4_096,
        max_rate_count: 16,
        max_pending_count: 32,
        role_modes: vec![
            RoleModePolicy {
                role: MessagingRole::Root,
                auto: true,
                steer: true,
                follow_up: true,
            },
            RoleModePolicy {
                role: MessagingRole::Subagent,
                auto: true,
                steer: false,
                follow_up: true,
            },
        ],
        receipt_fields: vec![
            "policy_identity".into(),
            "roster_identity".into(),
            "sender_identity".into(),
            "target_count".into(),
            "mode".into(),
            "variant".into(),
            "counter_identity".into(),
        ],
        non_claims: vec![
            MESSAGING_NON_CLAIM.into(),
            MESSAGING_TRANSPORT_NON_CLAIM.into(),
            MESSAGING_ORDERING_NON_CLAIM.into(),
            "not-message-correctness-or-cooperation".into(),
        ],
    })
    .expect("reviewed messaging policy is valid")
}

/// Host-supplied rate and pending-queue counters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessagingCounters {
    /// Stable identity of the rate counter.
    pub rate_counter_identity: String,
    /// Stable identity of the pending counter.
    pub pending_counter_identity: String,
    /// Current send count in the admitted rate window.
    pub rate_count: u32,
    /// Current pending count for the target family.
    pub pending_count: u32,
}

impl MessagingCounters {
    /// Validates counter identities and minimum values.
    ///
    /// # Errors
    ///
    /// Returns an error when counter identities or counts are invalid.
    pub fn new(
        rate_counter_identity: &str,
        pending_counter_identity: &str,
        rate_count: u32,
        pending_count: u32,
    ) -> Result<Self, MessagingError> {
        validate_label(rate_counter_identity)?;
        validate_label(pending_counter_identity)?;
        if rate_count == u32::MAX || pending_count == u32::MAX {
            return Err(MessagingError::InvalidCounters);
        }
        Ok(Self {
            rate_counter_identity: rate_counter_identity.into(),
            pending_counter_identity: pending_counter_identity.into(),
            rate_count,
            pending_count,
        })
    }
}

/// Limit class that can stop routing.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LimitClass {
    /// Body-size limit.
    MessageSize,
    /// Rate-window limit.
    Rate,
    /// Pending-queue limit.
    PendingQueue,
}

impl LimitClass {
    /// Returns the stable limit label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MessageSize => "message_size",
            Self::Rate => "rate",
            Self::PendingQueue => "pending_queue",
        }
    }
}

/// Checks all limits before routing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LimitDecision {
    /// Counter identities used by the decision.
    pub counter_identities: Vec<String>,
    /// Body byte count checked.
    pub message_bytes: usize,
}

/// Returns the pre-routing limit decision.
///
/// # Errors
///
/// Returns an error when policy or counter validation fails.
pub fn check_message_limits(
    message: &Message,
    counters: &MessagingCounters,
    policy: &MessagingPolicy,
) -> Result<LimitDecision, MessagingError> {
    if message.body.len() > policy.max_message_bytes {
        return Err(MessagingError::LimitExceeded(LimitClass::MessageSize));
    }
    if counters.rate_count >= policy.max_rate_count {
        return Err(MessagingError::LimitExceeded(LimitClass::Rate));
    }
    if counters.pending_count >= policy.max_pending_count {
        return Err(MessagingError::LimitExceeded(LimitClass::PendingQueue));
    }
    Ok(LimitDecision {
        counter_identities: vec![
            counters.rate_counter_identity.clone(),
            counters.pending_counter_identity.clone(),
        ],
        message_bytes: message.body.len(),
    })
}

/// The action selected for one target.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RoutingAction {
    /// Message enters an idle target's context.
    Deliver,
    /// Message steers active work.
    Steer,
    /// Message waits for the current turn.
    QueueAfterTurn,
}

impl RoutingAction {
    /// Returns the stable routing label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deliver => "deliver",
            Self::Steer => "steer",
            Self::QueueAfterTurn => "queue_after_turn",
        }
    }
}

/// A pure routing decision over one family roster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutingDecision {
    /// Host-derived sender identity.
    pub sender_identity: String,
    /// Resolved target identities.
    pub target_identities: Vec<String>,
    /// Requested mode.
    pub mode: DeliveryMode,
    /// Action for the target state and mode.
    pub action: RoutingAction,
    /// Roster identity bound to the decision.
    pub roster_identity: String,
    /// Counter identities bound before routing.
    pub counter_identities: Vec<String>,
}

/// Resolves a message after checking policy, roster, identity, and limits.
///
/// # Errors
///
/// Returns an error when policy, roster, mode, target, or limit validation fails.
#[allow(clippy::too_many_arguments)]
pub fn route_message(
    roster: &FamilyRoster,
    sender_identity: &str,
    message: &Message,
    counters: &MessagingCounters,
    policy: &MessagingPolicy,
) -> Result<RoutingDecision, MessagingError> {
    validate_policy(policy)?;
    if !roster.identity_matches() {
        return Err(MessagingError::StaleRoster);
    }
    let sender = roster.member(sender_identity)?;
    let role_policy = policy.role_mode(sender.role)?;
    if !role_policy.allows(message.mode) {
        return Err(MessagingError::ModeDenied);
    }
    let limits = check_message_limits(message, counters, policy)?;
    let target_identities = if message.broadcast {
        roster.broadcast_targets(sender_identity)?
    } else {
        vec![
            message
                .target
                .as_deref()
                .ok_or(MessagingError::InvalidTargetShape)?
                .into(),
        ]
    };
    for target_identity in &target_identities {
        let target = roster.member(target_identity)?;
        if target.state == TargetState::Unavailable {
            return Err(MessagingError::TargetUnavailable);
        }
        if target.identity == sender_identity {
            return Err(MessagingError::SelfTarget);
        }
    }
    let target = roster.member(&target_identities[0])?;
    let action = match message.mode {
        DeliveryMode::FollowUp => RoutingAction::QueueAfterTurn,
        DeliveryMode::Steer => RoutingAction::Steer,
        DeliveryMode::Auto => match target.state {
            TargetState::Idle => RoutingAction::Deliver,
            TargetState::Busy => RoutingAction::Steer,
            TargetState::Unavailable => return Err(MessagingError::TargetUnavailable),
        },
    };
    Ok(RoutingDecision {
        sender_identity: sender.identity.clone(),
        target_identities,
        mode: message.mode,
        action,
        roster_identity: roster.identity.clone(),
        counter_identities: limits.counter_identities,
    })
}

/// Host attestation class required for a success receipt.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DeliveryAttestationKind {
    /// Host confirms entry into an idle target context.
    ContextEntered,
    /// Host confirms accepted pending storage.
    PendingAccepted,
}

impl DeliveryAttestationKind {
    /// Returns the stable attestation label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContextEntered => "context_entered",
            Self::PendingAccepted => "pending_accepted",
        }
    }
}

/// Host attestation for a delivery effect.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryAttestation {
    /// Stable host-generated attestation identity.
    pub identity: String,
    /// Attestation class.
    pub kind: DeliveryAttestationKind,
}

/// Exactly one of the two admitted delivery receipt variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeliveryVariant {
    /// Message entered an idle target context.
    Delivered {
        /// Host attestation identity.
        attestation_identity: String,
    },
    /// Message was accepted for later delivery.
    Queued {
        /// Host attestation identity.
        attestation_identity: String,
    },
}

impl DeliveryVariant {
    /// Returns the stable receipt variant label.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Delivered { .. } => "delivered",
            Self::Queued { .. } => "queued",
        }
    }
}

/// A receipt draft with untrusted content fields for fail-closed validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessagingReceiptDraft {
    /// Routing decision to bind.
    pub routing: RoutingDecision,
    /// Selected success variant.
    pub variant: DeliveryVariant,
    /// Host attestation.
    pub attestation: DeliveryAttestation,
    /// Optional body material, always rejected.
    pub message_body: Option<String>,
    /// Optional overclaim material, always rejected.
    pub overclaim: Option<String>,
}

/// A validated metadata-only messaging receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessagingReceipt {
    /// Policy identity.
    pub policy_identity: String,
    /// Roster identity.
    pub roster_identity: String,
    /// Sender identity.
    pub sender_identity: String,
    /// Number of resolved targets.
    pub target_count: usize,
    /// Delivery mode.
    pub mode: DeliveryMode,
    /// Delivery variant.
    pub variant: DeliveryVariant,
    /// Counter identities.
    pub counter_identities: Vec<String>,
    /// Required safe non-claims.
    pub non_claims: Vec<String>,
    /// Domain-separated receipt identity.
    pub identity: String,
}

/// Projects a receipt after validating matching host attestation.
///
/// # Errors
///
/// Returns an error when the receipt draft, policy, or host attestation is invalid.
pub fn project_messaging_receipt(
    policy: &MessagingPolicy,
    draft: &MessagingReceiptDraft,
) -> Result<MessagingReceipt, MessagingError> {
    if draft.message_body.is_some() || draft.overclaim.is_some() {
        return Err(MessagingError::ReceiptContainsContent);
    }
    validate_hash(&draft.routing.roster_identity)?;
    validate_label(&draft.routing.sender_identity)?;
    validate_label(&draft.attestation.identity)?;
    if draft.routing.target_identities.is_empty() {
        return Err(MessagingError::ReceiptRejected);
    }
    let expected_kind = match draft.variant {
        DeliveryVariant::Delivered { .. } => DeliveryAttestationKind::ContextEntered,
        DeliveryVariant::Queued { .. } => DeliveryAttestationKind::PendingAccepted,
    };
    if draft.attestation.kind != expected_kind {
        return Err(MessagingError::MissingAttestation);
    }
    let expected_identity = match &draft.variant {
        DeliveryVariant::Delivered {
            attestation_identity,
        }
        | DeliveryVariant::Queued {
            attestation_identity,
        } => attestation_identity,
    };
    if expected_identity != &draft.attestation.identity {
        return Err(MessagingError::MissingAttestation);
    }
    let material = format!(
        "{}|{}|{}|{}|{}|{}|{}",
        policy.identity,
        draft.routing.roster_identity,
        draft.routing.sender_identity,
        draft.routing.target_identities.len(),
        draft.routing.mode.as_str(),
        draft.variant.as_str(),
        draft.attestation.identity
    );
    Ok(MessagingReceipt {
        policy_identity: policy.identity.clone(),
        roster_identity: draft.routing.roster_identity.clone(),
        sender_identity: draft.routing.sender_identity.clone(),
        target_count: draft.routing.target_identities.len(),
        mode: draft.routing.mode,
        variant: draft.variant.clone(),
        counter_identities: draft.routing.counter_identities.clone(),
        non_claims: policy.non_claims.clone(),
        identity: digest(MESSAGING_RECEIPT_DOMAIN, &material),
    })
}

/// Typed fail-closed messaging errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessagingError {
    /// A label is empty or oversized.
    InvalidLabel,
    /// The roster is empty, oversized, or malformed.
    InvalidRoster,
    /// Two roster members have the same identity.
    DuplicateRosterMember,
    /// A member belongs to another family.
    CrossFamilyTarget,
    /// Target identity is not in the admitted roster.
    UnknownTarget,
    /// A broadcast has no target members.
    EmptyBroadcast,
    /// Message shape has both or neither broadcast target fields.
    InvalidTargetShape,
    /// Message text attempted to claim sender identity.
    SpoofedSenderIdentity,
    /// Message body exceeds policy or is empty.
    MessageSizeExceeded,
    /// Generated policy failed closed.
    PolicyRejected,
    /// Counter identities or values are invalid.
    InvalidCounters,
    /// A size, rate, or pending limit was exceeded.
    LimitExceeded(LimitClass),
    /// Roster identity is stale.
    StaleRoster,
    /// Sender role is not allowed to use the selected mode.
    ModeDenied,
    /// Target cannot receive work.
    TargetUnavailable,
    /// Sender attempted to target itself.
    SelfTarget,
    /// A receipt included message content or an overclaim.
    ReceiptContainsContent,
    /// A receipt has no matching host attestation.
    MissingAttestation,
    /// Receipt fields are malformed.
    ReceiptRejected,
}

fn validate_policy(policy: &MessagingPolicy) -> Result<(), MessagingError> {
    if policy.cohort != REVIEWED_MESSAGING_COHORT
        || policy.source_revision != REVIEWED_MESSAGING_REVISION
        || policy.max_message_bytes == 0
        || policy.max_rate_count == 0
        || policy.max_pending_count == 0
        || policy.role_modes.len() != 2
        || policy.receipt_fields.len() < 7
        || policy.non_claims.len() < 4
        || !policy
            .non_claims
            .iter()
            .any(|claim| claim == MESSAGING_NON_CLAIM)
        || !policy
            .non_claims
            .iter()
            .any(|claim| claim == MESSAGING_ORDERING_NON_CLAIM)
    {
        return Err(MessagingError::PolicyRejected);
    }
    let mut roles = policy
        .role_modes
        .iter()
        .map(|row| row.role)
        .collect::<Vec<_>>();
    roles.sort();
    if roles.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(MessagingError::PolicyRejected);
    }
    let material = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        policy.cohort,
        policy.source_revision,
        policy.max_message_bytes,
        policy.max_rate_count,
        policy.max_pending_count,
        canonical_modes(&policy.role_modes),
        join(&policy.receipt_fields),
        join(&policy.non_claims),
    );
    if digest(MESSAGING_POLICY_DOMAIN, &material) != policy.identity {
        return Err(MessagingError::PolicyRejected);
    }
    Ok(())
}

fn canonical_roster(family_id: &str, members: &[RosterMember]) -> String {
    let mut material = String::from(family_id);
    for member in members {
        material.push('|');
        material.push_str(&member.identity);
        material.push('|');
        material.push_str(member.role.as_str());
        material.push('|');
        material.push_str(member.state.as_str());
    }
    material
}

fn canonical_modes(modes: &[RoleModePolicy]) -> String {
    let mut ordered = modes.to_vec();
    ordered.sort_by_key(|row| row.role);
    ordered
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}:{}",
                row.role.as_str(),
                row.auto,
                row.steer,
                row.follow_up
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn validate_label(value: &str) -> Result<(), MessagingError> {
    if value.is_empty() || value.len() > MAX_LABEL_BYTES || value.contains('\n') {
        return Err(MessagingError::InvalidLabel);
    }
    Ok(())
}

fn validate_hash(value: &str) -> Result<(), MessagingError> {
    if value.len() != MESSAGING_BLAKE3_HEX_CHARS
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(MessagingError::ReceiptRejected);
    }
    Ok(())
}

fn digest(domain: &str, material: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[HASH_SEPARATOR]);
    hasher.update(material.as_bytes());
    format!("{}", hasher.finalize().to_hex())
}

fn join(values: &[String]) -> String {
    values.join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAMILY: &str = "family-a";
    const ROOT: &str = "root-a";
    const SUBAGENT: &str = "subagent-a";
    const RATE_COUNTER: &str = "rate-a";
    const PENDING_COUNTER: &str = "pending-a";
    const ATTESTATION: &str = "attestation-a";

    fn policy() -> MessagingPolicy {
        reviewed_messaging_policy()
    }

    fn roster() -> FamilyRoster {
        FamilyRoster::new(
            FAMILY,
            vec![
                RosterMember {
                    identity: ROOT.into(),
                    family_id: FAMILY.into(),
                    role: MessagingRole::Root,
                    state: TargetState::Idle,
                },
                RosterMember {
                    identity: SUBAGENT.into(),
                    family_id: FAMILY.into(),
                    role: MessagingRole::Subagent,
                    state: TargetState::Busy,
                },
            ],
        )
        .expect("roster")
    }

    fn counters() -> MessagingCounters {
        MessagingCounters::new(RATE_COUNTER, PENDING_COUNTER, 0, 0).expect("counters")
    }

    fn message(mode: DeliveryMode, target: Option<&str>, broadcast: bool) -> Message {
        Message::new(
            MessageDraft {
                target: target.map(String::from),
                broadcast,
                mode,
                body: "bounded message".into(),
                claimed_sender: None,
            },
            &policy(),
        )
        .expect("message")
    }

    #[test]
    fn modes_resolve_for_idle_busy_follow_up_and_broadcast() {
        let routed = route_message(
            &roster(),
            ROOT,
            &message(DeliveryMode::Auto, Some(SUBAGENT), false),
            &counters(),
            &policy(),
        )
        .expect("busy auto");
        assert_eq!(routed.action, RoutingAction::Steer);
        let idle = FamilyRoster::new(
            FAMILY,
            vec![
                RosterMember {
                    identity: ROOT.into(),
                    family_id: FAMILY.into(),
                    role: MessagingRole::Root,
                    state: TargetState::Idle,
                },
                RosterMember {
                    identity: SUBAGENT.into(),
                    family_id: FAMILY.into(),
                    role: MessagingRole::Subagent,
                    state: TargetState::Idle,
                },
            ],
        )
        .expect("idle roster");
        let delivered = route_message(
            &idle,
            ROOT,
            &message(DeliveryMode::Auto, Some(SUBAGENT), false),
            &counters(),
            &policy(),
        )
        .expect("idle auto");
        assert_eq!(delivered.action, RoutingAction::Deliver);
        let follow_up = route_message(
            &roster(),
            ROOT,
            &message(DeliveryMode::FollowUp, Some(SUBAGENT), false),
            &counters(),
            &policy(),
        )
        .expect("follow up");
        assert_eq!(follow_up.action, RoutingAction::QueueAfterTurn);
        let broadcast = route_message(
            &roster(),
            ROOT,
            &message(DeliveryMode::Auto, None, true),
            &counters(),
            &policy(),
        )
        .expect("broadcast");
        assert_eq!(broadcast.target_identities, vec![String::from(SUBAGENT)]);
    }

    #[test]
    fn delivered_and_queued_receipts_require_matching_attestation() {
        let routing = route_message(
            &roster(),
            ROOT,
            &message(DeliveryMode::Auto, Some(SUBAGENT), false),
            &counters(),
            &policy(),
        )
        .expect("routing");
        let delivered = project_messaging_receipt(
            &policy(),
            &MessagingReceiptDraft {
                routing: routing.clone(),
                variant: DeliveryVariant::Delivered {
                    attestation_identity: ATTESTATION.into(),
                },
                attestation: DeliveryAttestation {
                    identity: ATTESTATION.into(),
                    kind: DeliveryAttestationKind::ContextEntered,
                },
                message_body: None,
                overclaim: None,
            },
        )
        .expect("delivered receipt");
        assert_eq!(delivered.variant.as_str(), "delivered");
        let queued_routing = RoutingDecision {
            action: RoutingAction::QueueAfterTurn,
            mode: DeliveryMode::FollowUp,
            ..routing
        };
        let queued = project_messaging_receipt(
            &policy(),
            &MessagingReceiptDraft {
                routing: queued_routing,
                variant: DeliveryVariant::Queued {
                    attestation_identity: ATTESTATION.into(),
                },
                attestation: DeliveryAttestation {
                    identity: ATTESTATION.into(),
                    kind: DeliveryAttestationKind::PendingAccepted,
                },
                message_body: None,
                overclaim: None,
            },
        )
        .expect("queued receipt");
        assert_eq!(queued.variant.as_str(), "queued");
    }

    #[test]
    fn negative_identity_roster_mode_limit_and_receipt_inputs_fail_closed() {
        assert_eq!(
            Message::new(
                MessageDraft {
                    target: Some(SUBAGENT.into()),
                    broadcast: false,
                    mode: DeliveryMode::Auto,
                    body: "body".into(),
                    claimed_sender: Some("spoof".into()),
                },
                &policy(),
            ),
            Err(MessagingError::SpoofedSenderIdentity)
        );
        let message = message(DeliveryMode::Auto, Some(SUBAGENT), false);
        let limited = MessagingCounters::new(RATE_COUNTER, PENDING_COUNTER, 16, 0).expect("limit");
        assert_eq!(
            check_message_limits(&message, &limited, &policy()),
            Err(MessagingError::LimitExceeded(LimitClass::Rate))
        );
        assert_eq!(
            route_message(
                &roster(),
                ROOT,
                &message,
                &counters(),
                &MessagingPolicy {
                    role_modes: vec![RoleModePolicy {
                        role: MessagingRole::Root,
                        auto: true,
                        steer: false,
                        follow_up: true,
                    }],
                    ..policy()
                }
            ),
            Err(MessagingError::PolicyRejected)
        );
        let mut forged = roster();
        forged.identity = "bad".into();
        assert_eq!(
            route_message(&forged, ROOT, &message, &counters(), &policy()),
            Err(MessagingError::StaleRoster)
        );
        let routing =
            route_message(&roster(), ROOT, &message, &counters(), &policy()).expect("routing");
        assert_eq!(
            project_messaging_receipt(
                &policy(),
                &MessagingReceiptDraft {
                    routing,
                    variant: DeliveryVariant::Delivered {
                        attestation_identity: ATTESTATION.into(),
                    },
                    attestation: DeliveryAttestation {
                        identity: ATTESTATION.into(),
                        kind: DeliveryAttestationKind::ContextEntered,
                    },
                    message_body: Some("private body".into()),
                    overclaim: Some("exactly once".into()),
                },
            ),
            Err(MessagingError::ReceiptContainsContent)
        );
    }
}
