use std::collections::BTreeMap;

use animus_adapter_rlm_kernel::{HostOperationClass, HostRequest};

const BLAKE3_HEX_CHARS: usize = 64;
const MAX_SAFE_LABEL_CHARS: usize = 128;
const BINDING_SET_DOMAIN: &str = "animus.rlm-host.binding-set.v1";
const RESULT_DOMAIN: &str = "animus.rlm-host.result.v1";

/// Stable route class used by construction and observations.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RouteClass {
    /// Capability-relative file read.
    FileRead,
    /// Revision-gated workspace edit.
    FileEdit,
    /// Exact bounded process profile.
    Shell,
    /// Existing tool-executor kit.
    Tool,
    /// Existing child-session service.
    Subagent,
    /// Existing context resolver.
    ContextRead,
    /// Explicit Steel kernel owner.
    KernelClear,
}

impl RouteClass {
    /// Returns the stable route label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileRead => "file_read",
            Self::FileEdit => "file_edit",
            Self::Shell => "shell",
            Self::Tool => "tool",
            Self::Subagent => "subagent",
            Self::ContextRead => "context_read",
            Self::KernelClear => "kernel_clear",
        }
    }

    /// Maps an RLM operation to its exact route.
    #[must_use]
    pub const fn for_operation(operation: HostOperationClass) -> Self {
        match operation {
            HostOperationClass::FileRead => Self::FileRead,
            HostOperationClass::FileEdit => Self::FileEdit,
            HostOperationClass::Shell => Self::Shell,
            HostOperationClass::Tool => Self::Tool,
            HostOperationClass::Subagent => Self::Subagent,
            HostOperationClass::ContextRead => Self::ContextRead,
            HostOperationClass::KernelClear => Self::KernelClear,
        }
    }
}

/// Validated BLAKE3 binding identity.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BindingIdentity(String);

impl BindingIdentity {
    /// Validates one lowercase BLAKE3 identity.
    ///
    /// # Errors
    ///
    /// Returns [`ConstructionError::InvalidIdentity`] for malformed text.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ConstructionError> {
        let value = value.as_ref();
        if is_blake3(value) {
            Ok(Self(value.to_owned()))
        } else {
            Err(ConstructionError::InvalidIdentity)
        }
    }

    /// Returns validated identity text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Stable binding and scope for one route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteBinding {
    route: RouteClass,
    identity: BindingIdentity,
    scope_identity: BindingIdentity,
    generation: u64,
}

impl RouteBinding {
    /// Builds one exact route binding.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed identities or a zero generation.
    pub fn new(
        route: RouteClass,
        identity: impl AsRef<str>,
        scope_identity: impl AsRef<str>,
        generation: u64,
    ) -> Result<Self, ConstructionError> {
        if generation == 0 {
            return Err(ConstructionError::InvalidGeneration);
        }
        Ok(Self {
            route,
            identity: BindingIdentity::new(identity)?,
            scope_identity: BindingIdentity::new(scope_identity)?,
            generation,
        })
    }

    /// Returns the route class.
    #[must_use]
    pub const fn route(&self) -> RouteClass {
        self.route
    }

    /// Returns the exact capability-pack identity.
    #[must_use]
    pub const fn identity(&self) -> &BindingIdentity {
        &self.identity
    }

    /// Returns the exact admitted scope identity.
    #[must_use]
    pub const fn scope_identity(&self) -> &BindingIdentity {
        &self.scope_identity
    }

    /// Returns the expected current generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

/// Complete route binding set with a stable aggregate identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingSet {
    bindings: BTreeMap<RouteClass, RouteBinding>,
    identity: BindingIdentity,
}

impl BindingSet {
    /// Validates unique route bindings and computes their aggregate identity.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty set or duplicate route.
    pub fn new(
        bindings: impl IntoIterator<Item = RouteBinding>,
    ) -> Result<Self, ConstructionError> {
        let mut by_route = BTreeMap::new();
        for binding in bindings {
            let route = binding.route();
            if by_route.insert(route, binding).is_some() {
                return Err(ConstructionError::DuplicateBinding(route));
            }
        }
        if by_route.is_empty() {
            return Err(ConstructionError::EmptyBindings);
        }
        let mut hasher = blake3::Hasher::new();
        frame(&mut hasher, BINDING_SET_DOMAIN.as_bytes());
        for (route, binding) in &by_route {
            frame(&mut hasher, route.as_str().as_bytes());
            frame(&mut hasher, binding.identity().as_str().as_bytes());
            frame(&mut hasher, binding.scope_identity().as_str().as_bytes());
            frame(&mut hasher, &binding.generation().to_le_bytes());
        }
        let identity = BindingIdentity::new(hasher.finalize().to_hex().as_str())?;
        Ok(Self {
            bindings: by_route,
            identity,
        })
    }

    /// Returns the binding for one route.
    #[must_use]
    pub fn get(&self, route: RouteClass) -> Option<&RouteBinding> {
        self.bindings.get(&route)
    }

    /// Returns the aggregate binding-set identity.
    #[must_use]
    pub const fn identity(&self) -> &BindingIdentity {
        &self.identity
    }

    /// Returns the number of explicitly bound routes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns whether the set has no routes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

/// Current route scope derived without effects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteScope {
    /// BLAKE3 identity of the complete effect-bearing request.
    pub identity: BindingIdentity,
}

impl RouteScope {
    /// Derives one domain-separated scope identity from the typed request.
    ///
    /// # Errors
    ///
    /// Returns an identity error only if internal BLAKE3 encoding fails validation.
    pub fn for_request(request: &HostRequest) -> Result<Self, ConstructionError> {
        let mut hasher = blake3::Hasher::new();
        frame(&mut hasher, b"animus.rlm-host.request-scope.v1");
        frame(&mut hasher, request.operation_class().as_str().as_bytes());
        match request {
            HostRequest::FileRead { target } => frame(&mut hasher, target.as_bytes()),
            HostRequest::FileEdit {
                target,
                replacement,
            } => {
                frame(&mut hasher, target.as_bytes());
                frame(&mut hasher, replacement.as_bytes());
            }
            HostRequest::Shell { command } => frame(&mut hasher, command.as_bytes()),
            HostRequest::Tool { name, payload } => {
                frame(&mut hasher, name.as_bytes());
                frame(&mut hasher, payload.as_bytes());
            }
            HostRequest::Subagent(child) => frame_child_scope(&mut hasher, child),
            HostRequest::ContextRead { key } => frame(&mut hasher, key.as_bytes()),
            HostRequest::KernelClear => frame(&mut hasher, b"explicit"),
        }
        Ok(Self {
            identity: BindingIdentity::new(hasher.finalize().to_hex().as_str())?,
        })
    }
}

/// Named executor bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_field_names)]
pub struct ExecutorLimits {
    /// Maximum file bytes admitted for one visible read.
    pub max_file_read_bytes: u64,
    /// Maximum context bytes admitted before application budgeting.
    pub max_context_bytes: u32,
    /// Maximum context lines admitted before application budgeting.
    pub max_context_lines: u32,
    /// Maximum model-visible result characters returned to the bridge.
    pub max_result_chars: u32,
    /// Maximum status observations before child collection.
    pub max_child_status_checks: u32,
}

impl ExecutorLimits {
    /// Validates that every bound is positive.
    ///
    /// # Errors
    ///
    /// Returns [`ConstructionError::InvalidLimit`] when any bound is zero.
    pub const fn validate(self) -> Result<Self, ConstructionError> {
        if self.max_file_read_bytes == 0
            || self.max_context_bytes == 0
            || self.max_context_lines == 0
            || self.max_result_chars == 0
            || self.max_child_status_checks == 0
        {
            Err(ConstructionError::InvalidLimit)
        } else {
            Ok(self)
        }
    }
}

/// Executor construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstructionError {
    /// Binding set was empty.
    EmptyBindings,
    /// Route appeared more than once.
    DuplicateBinding(RouteClass),
    /// Required identity was not lowercase BLAKE3 hex.
    InvalidIdentity,
    /// Route generation was zero.
    InvalidGeneration,
    /// One named limit was zero.
    InvalidLimit,
    /// A pack and route binding disagreed.
    PackBindingMismatch(RouteClass),
}

impl core::fmt::Display for ConstructionError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::EmptyBindings => "RLM host binding set is empty",
            Self::DuplicateBinding(_) => "RLM host route binding is duplicated",
            Self::InvalidIdentity => "RLM host identity is malformed",
            Self::InvalidGeneration => "RLM host generation must be positive",
            Self::InvalidLimit => "RLM host limit must be positive",
            Self::PackBindingMismatch(_) => "RLM host pack does not match its route binding",
        })
    }
}

impl std::error::Error for ConstructionError {}

/// Stable host dispatch failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchFailureClass {
    /// Required pack or exact service is absent.
    Unavailable,
    /// Current authority or scope denied the effect.
    Denied,
    /// Seal, generation, binding, revision, or target facts drifted.
    Stale,
    /// Cancellation stopped the effect.
    Cancelled,
    /// Deadline expired.
    TimedOut,
    /// Input or owner result was malformed.
    Malformed,
    /// A named bound was exceeded.
    BoundExceeded,
    /// Existing route owner reported a conflict.
    Conflict,
    /// Injected owner failed safely.
    OwnerFailure,
}

impl DispatchFailureClass {
    /// Returns the stable failure label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::Denied => "denied",
            Self::Stale => "stale",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Malformed => "malformed",
            Self::BoundExceeded => "bound_exceeded",
            Self::Conflict => "conflict",
            Self::OwnerFailure => "owner_failure",
        }
    }
}

/// Safe typed dispatch failure without private request or result bodies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchFailure {
    /// Failed route.
    pub route: RouteClass,
    /// Stable failure class.
    pub class: DispatchFailureClass,
    /// Safe bounded source class.
    source_class: String,
}

impl DispatchFailure {
    /// Builds one safe failure.
    #[must_use]
    pub fn new(route: RouteClass, class: DispatchFailureClass, source_class: &str) -> Self {
        let bounded = source_class
            .chars()
            .filter(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
            .take(MAX_SAFE_LABEL_CHARS)
            .collect::<String>();
        Self {
            route,
            class,
            source_class: if bounded.is_empty() {
                String::from("unspecified")
            } else {
                bounded
            },
        }
    }

    /// Returns the safe source failure label.
    #[must_use]
    pub fn source_class(&self) -> &str {
        self.source_class.as_str()
    }
}

impl core::fmt::Display for DispatchFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "RLM host {} failed: {}",
            self.route.as_str(),
            self.class.as_str()
        )
    }
}

impl std::error::Error for DispatchFailure {}

/// Metadata-only observation for one successful route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchObservation {
    /// Route class.
    pub route: RouteClass,
    /// Exact binding identity used after current recheck.
    pub binding_identity: BindingIdentity,
    /// Current generation used for this effect.
    pub generation: u64,
    /// Model-visible result character count.
    pub result_chars: u32,
    /// Domain-separated BLAKE3 identity of the full owner result.
    pub result_identity: BindingIdentity,
    /// Safe owner receipt or output identity.
    pub owner_identity: BindingIdentity,
}

/// Inputs for one bounded host result.
pub struct RlmHostResultDraft<'a> {
    /// Exact route binding.
    pub binding: &'a RouteBinding,
    /// Model-visible value candidate.
    pub value: &'a str,
    /// Safe owner identity.
    pub owner_identity: BindingIdentity,
    /// Maximum visible characters.
    pub max_result_chars: u32,
}

/// Bounded model-visible result plus metadata-only observation.
#[derive(Clone, Eq, PartialEq)]
pub struct RlmHostResult {
    value: String,
    observation: DispatchObservation,
}

impl RlmHostResult {
    /// Bounds one owner result and builds its observation.
    ///
    /// # Errors
    ///
    /// Returns [`DispatchFailureClass::BoundExceeded`] for oversized results.
    pub fn new(route: RouteClass, draft: RlmHostResultDraft<'_>) -> Result<Self, DispatchFailure> {
        let binding = draft.binding;
        let value = draft.value.trim().to_owned();
        let owner_identity = draft.owner_identity;
        let max_result_chars = draft.max_result_chars;
        if value.is_empty() || value.contains('\0') {
            return Err(DispatchFailure::new(
                route,
                DispatchFailureClass::Malformed,
                "invalid_result",
            ));
        }
        let result_chars = u32::try_from(value.chars().count()).unwrap_or(u32::MAX);
        if result_chars > max_result_chars {
            return Err(DispatchFailure::new(
                route,
                DispatchFailureClass::BoundExceeded,
                "result_chars",
            ));
        }
        let mut hasher = blake3::Hasher::new();
        frame(&mut hasher, RESULT_DOMAIN.as_bytes());
        frame(&mut hasher, route.as_str().as_bytes());
        frame(&mut hasher, value.as_bytes());
        let result_identity =
            BindingIdentity::new(hasher.finalize().to_hex().as_str()).map_err(|_error| {
                DispatchFailure::new(route, DispatchFailureClass::Malformed, "result_hash")
            })?;
        Ok(Self {
            value,
            observation: DispatchObservation {
                route,
                binding_identity: binding.identity().clone(),
                generation: binding.generation(),
                result_chars,
                result_identity,
                owner_identity,
            },
        })
    }

    /// Returns model-visible result text.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    /// Returns metadata-only execution facts.
    #[must_use]
    pub const fn observation(&self) -> &DispatchObservation {
        &self.observation
    }
}

impl core::fmt::Debug for RlmHostResult {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RlmHostResult")
            .field("value", &"<model-visible>")
            .field("observation", &self.observation)
            .finish()
    }
}

fn frame_child_scope(
    hasher: &mut blake3::Hasher,
    child: &animus_adapter_rlm_kernel::ChildSpawnRequest,
) {
    frame(hasher, child.profile().as_bytes());
    frame(hasher, child.context_slice().as_bytes());
    frame(hasher, &child.depth().to_le_bytes());
    frame(hasher, &child.fan_out().to_le_bytes());
    frame(hasher, &child.token_budget().to_le_bytes());
    frame(hasher, &child.wall_time_ms().to_le_bytes());
    frame(hasher, &child.result_char_budget().to_le_bytes());
    frame(hasher, &[u8::from(child.ambient_discovery())]);
    frame(hasher, &[u8::from(child.cross_session())]);
}

pub(crate) fn is_blake3(value: &str) -> bool {
    value.len() == BLAKE3_HEX_CHARS
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn frame(hasher: &mut blake3::Hasher, value: &[u8]) {
    hasher.update(&u64::try_from(value.len()).unwrap_or(u64::MAX).to_le_bytes());
    hasher.update(value);
}
