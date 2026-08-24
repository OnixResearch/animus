use std::collections::BTreeMap;

/// Untrusted MCP `readOnlyHint` classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReadOnlyHint {
    /// The descriptor declares `readOnlyHint: true`.
    DeclaredTrue,
    /// The descriptor declares `readOnlyHint: false`.
    DeclaredFalse,
    /// The descriptor omits the hint.
    Missing,
    /// The descriptor uses an invalid value type.
    Malformed,
    /// The descriptor uses an unsupported annotation shape.
    Unknown,
}

/// Pure exposure input for one validated MCP descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpExposureDescriptor<'a> {
    /// Exact configured server identity.
    pub server_identity: &'a str,
    /// Preserved model-facing tool name.
    pub tool_name: &'a str,
    /// Exact validated schema identity.
    pub schema_identity: &'a str,
    /// Exact operation class.
    pub operation_class: &'a str,
    /// Bound root session identity.
    pub session_identity: &'a str,
    /// Bound server generation.
    pub generation: u64,
    /// Untrusted descriptor annotation.
    pub read_only_hint: ReadOnlyHint,
}

/// Exact host and Basalt authority fact for one mutating tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExactMutationAuthority<'a> {
    /// Exact configured server identity.
    pub server_identity: &'a str,
    /// Exact preserved tool name.
    pub tool_name: &'a str,
    /// Exact schema identity.
    pub schema_identity: &'a str,
    /// Exact operation class.
    pub operation_class: &'a str,
    /// Exact session identity.
    pub session_identity: &'a str,
    /// Whether the host admits the operation.
    pub host_admitted: bool,
    /// Whether Basalt admits the operation.
    pub basalt_admitted: bool,
}

/// One non-MCP name already present in the model-facing catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltInToolSource<'a> {
    /// Preserved model-facing name.
    pub tool_name: &'a str,
    /// Safe source label for diagnostics.
    pub source_label: &'a str,
}

/// Reason one descriptor was not exposed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExposureDenial {
    /// No exact mutation authority admitted the descriptor.
    AuthorityDenied,
    /// The descriptor does not belong to the current session or generation.
    StaleBinding,
    /// More than one source declared the model-facing name.
    NameCollision,
}

/// One denied descriptor and its bounded reason.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedExposure {
    /// Exact preserved tool name.
    pub tool_name: String,
    /// Safe configured source label.
    pub source_label: String,
    /// Denial classification.
    pub denial: ExposureDenial,
}

/// Collision diagnostic that names every conflicting source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExposureCollision {
    /// Exact colliding model-facing name.
    pub tool_name: String,
    /// Deterministically sorted safe source labels.
    pub source_labels: Vec<String>,
}

/// One tool admitted to the model-facing MCP catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExposedMcpTool {
    /// Exact configured server identity.
    pub server_identity: String,
    /// Preserved model-facing tool name.
    pub tool_name: String,
    /// Exact schema identity.
    pub schema_identity: String,
    /// Exact server generation.
    pub generation: u64,
    /// Whether exact mutation authority, rather than metadata, admitted the tool.
    pub mutation_authorized: bool,
}

/// Safe exposure receipt with an explicit non-claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpExposureReceipt {
    /// Number of exposed tools.
    pub exposed_count: usize,
    /// Number of denied descriptors.
    pub rejected_count: usize,
    /// Descriptor hints remain untrusted metadata.
    pub hint_is_untrusted_metadata: bool,
    /// The receipt never claims side-effect freedom.
    pub claims_side_effect_freedom: bool,
}

/// Pure root exposure result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpExposureCatalog {
    /// Current root session identity.
    pub session_identity: String,
    /// Current server generation.
    pub generation: u64,
    /// Deterministically ordered exposed tools.
    pub exposed: Vec<ExposedMcpTool>,
    /// Deterministically ordered rejected descriptors.
    pub rejected: Vec<RejectedExposure>,
    /// Collision diagnostics.
    pub collisions: Vec<ExposureCollision>,
    /// Aggregate-only non-claim receipt.
    pub receipt: McpExposureReceipt,
}

/// Pure child-session authority fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChildToolAuthority<'a> {
    /// Exact preserved tool name.
    pub tool_name: &'a str,
    /// Exact schema identity.
    pub schema_identity: &'a str,
    /// Whether the child profile admits the tool.
    pub admitted: bool,
}

/// Complete borrowed input for one root exposure reduction.
#[derive(Clone, Copy, Debug)]
pub struct McpExposureRequest<'a> {
    /// Current root session identity.
    pub session_identity: &'a str,
    /// Current server generation.
    pub generation: u64,
    /// Validated descriptor facts.
    pub descriptors: &'a [McpExposureDescriptor<'a>],
    /// Current exact mutation-authority facts.
    pub mutation_authorities: &'a [ExactMutationAuthority<'a>],
    /// Existing model-facing catalog names.
    pub built_in_sources: &'a [BuiltInToolSource<'a>],
}

/// Computes fail-closed MCP exposure without performing effects.
#[must_use]
// r[impl animus.mcp.exposure]
// r[impl animus.mcp.authority]
// r[impl animus.mcp.collision]
pub fn reduce_mcp_exposure(request: McpExposureRequest<'_>) -> McpExposureCatalog {
    let collisions = collisions(request.descriptors, request.built_in_sources);
    let mut exposed = Vec::new();
    let mut rejected = Vec::new();
    for descriptor in request.descriptors {
        let source_label = descriptor.server_identity;
        let denial = if descriptor.session_identity != request.session_identity
            || descriptor.generation != request.generation
        {
            Some(ExposureDenial::StaleBinding)
        } else if collisions
            .iter()
            .any(|collision| collision.tool_name == descriptor.tool_name)
        {
            Some(ExposureDenial::NameCollision)
        } else if descriptor.read_only_hint == ReadOnlyHint::DeclaredTrue
            || exact_authority(descriptor, request.mutation_authorities)
        {
            None
        } else {
            Some(ExposureDenial::AuthorityDenied)
        };
        if let Some(denial) = denial {
            rejected.push(RejectedExposure {
                tool_name: String::from(descriptor.tool_name),
                source_label: String::from(source_label),
                denial,
            });
        } else {
            exposed.push(ExposedMcpTool {
                server_identity: String::from(descriptor.server_identity),
                tool_name: String::from(descriptor.tool_name),
                schema_identity: String::from(descriptor.schema_identity),
                generation: descriptor.generation,
                mutation_authorized: descriptor.read_only_hint != ReadOnlyHint::DeclaredTrue,
            });
        }
    }
    exposed.sort_by(|left, right| left.tool_name.cmp(&right.tool_name));
    rejected.sort_by(|left, right| {
        left.tool_name
            .cmp(&right.tool_name)
            .then_with(|| left.source_label.cmp(&right.source_label))
    });
    McpExposureCatalog {
        session_identity: String::from(request.session_identity),
        generation: request.generation,
        receipt: McpExposureReceipt {
            exposed_count: exposed.len(),
            rejected_count: rejected.len(),
            hint_is_untrusted_metadata: true,
            claims_side_effect_freedom: false,
        },
        exposed,
        rejected,
        collisions,
    }
}

/// Projects root exposure into a child without permitting broadening.
#[must_use]
// r[impl animus.mcp.child]
pub fn project_child_mcp_exposure(
    root: &McpExposureCatalog,
    current_generation: u64,
    child_authorities: &[ChildToolAuthority<'_>],
) -> Vec<ExposedMcpTool> {
    if current_generation != root.generation {
        return Vec::new();
    }
    root.exposed
        .iter()
        .filter(|tool| {
            child_authorities.iter().any(|authority| {
                authority.admitted
                    && authority.tool_name == tool.tool_name
                    && authority.schema_identity == tool.schema_identity
            })
        })
        .cloned()
        .collect()
}

fn exact_authority(
    descriptor: &McpExposureDescriptor<'_>,
    authorities: &[ExactMutationAuthority<'_>],
) -> bool {
    authorities.iter().any(|authority| {
        authority.host_admitted
            && authority.basalt_admitted
            && authority.server_identity == descriptor.server_identity
            && authority.tool_name == descriptor.tool_name
            && authority.schema_identity == descriptor.schema_identity
            && authority.operation_class == descriptor.operation_class
            && authority.session_identity == descriptor.session_identity
    })
}

fn collisions(
    descriptors: &[McpExposureDescriptor<'_>],
    built_in_sources: &[BuiltInToolSource<'_>],
) -> Vec<ExposureCollision> {
    let mut sources = BTreeMap::<&str, Vec<String>>::new();
    for descriptor in descriptors {
        sources
            .entry(descriptor.tool_name)
            .or_default()
            .push(String::from(descriptor.server_identity));
    }
    for built_in in built_in_sources {
        sources
            .entry(built_in.tool_name)
            .or_default()
            .push(String::from(built_in.source_label));
    }
    sources
        .into_iter()
        .filter_map(|(tool_name, mut source_labels)| {
            source_labels.sort();
            source_labels.dedup();
            (source_labels.len() > 1).then(|| ExposureCollision {
                tool_name: String::from(tool_name),
                source_labels,
            })
        })
        .collect()
}
