//! Pure terminal-image planning and the feature-gated host execution shell.
//!
//! Planning consumes caller-supplied capability facts and metadata. Only
//! [`execute_terminal_image_plan`] resolves bytes or writes to a supplied
//! terminal writer.

use std::io::Write;

/// Immutable Rats cohort selected for terminal-image support.
pub const RATS_IMAGE_COHORT_REVISION: &str = "3b498c3d7c248c335d3e7faf12f133a18c87d632";
const BLAKE3_HEX_CHARS: usize = 64;
const RECEIPT_SCHEMA: &str = "animus.terminal-image-receipt.v1";
const RECEIPT_DOMAIN: &str = "animus.terminal-image-receipt.v1\0";
const MAX_SAFE_LABEL_CHARS: usize = 96;
const MIN_IMAGE_BYTES: u64 = 1;
const MIN_DIMENSION: u32 = 1;
const MIN_CELL_COUNT: u16 = 1;
const MIN_OPERATION_COUNT: u16 = 1;
const MAX_PALETTE_COLORS: u16 = 255;
const MAX_WORK_UNITS: u64 = 250_000_000;
const KITTY_RESPONSE_PREFIX: &[u8] = b"\x1b_G";
const KITTY_RESPONSE_SUFFIX: &[u8] = b"\x1b\\";
const KITTY_RESPONSE_OK: &[u8] = b";OK";
const FORBIDDEN_EVIDENCE_TOKENS: &[&str] = &[
    "base64",
    "data:",
    "\\x1b",
    "\u{1b}",
    "/home/",
    "/tmp/",
    "term_program",
    "kitty_window_id",
    "credential",
    "provider_payload",
];

/// Terminal image protocol selected from explicit capability facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalImageProtocol {
    /// Kitty graphics protocol.
    Kitty,
    /// iTerm2 inline-image protocol.
    Iterm2,
    /// Sixel graphics protocol.
    Sixel,
    /// Bounded text fallback.
    Text,
}

impl TerminalImageProtocol {
    const fn label(self) -> &'static str {
        match self {
            Self::Kitty => "kitty",
            Self::Iterm2 => "iterm2",
            Self::Sixel => "sixel",
            Self::Text => "text",
        }
    }
}

/// One operation in an ordered terminal-image lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalImageOperation {
    /// Query a terminal capability through a host-owned adapter.
    Query,
    /// Transmit bounded image content through `rat-image`.
    Transmit,
    /// Place a transmitted image through a host-owned adapter.
    Place,
    /// Replace an existing image through a host-owned adapter.
    Replace,
    /// Delete an existing image through a host-owned adapter.
    Delete,
    /// Perform terminal cleanup through a host-owned adapter.
    Cleanup,
    /// Render bounded alternative text without protocol bytes.
    TextFallback,
}

impl TerminalImageOperation {
    const fn label(self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Transmit => "transmit",
            Self::Place => "place",
            Self::Replace => "replace",
            Self::Delete => "delete",
            Self::Cleanup => "cleanup",
            Self::TextFallback => "text-fallback",
        }
    }
}

/// Support state for one exact terminal-image operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalImageCapabilityState {
    /// The supplied host row supports the operation.
    Supported,
    /// The supplied host row does not support the operation.
    Unsupported,
}

/// Operation-level capability facts for one protocol.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalImageCapabilities {
    /// Protocol described by this row.
    pub protocol: TerminalImageProtocol,
    /// Query support.
    pub query: TerminalImageCapabilityState,
    /// Transmission support.
    pub transmit: TerminalImageCapabilityState,
    /// Placement support.
    pub place: TerminalImageCapabilityState,
    /// Replacement support.
    pub replace: TerminalImageCapabilityState,
    /// Deletion support.
    pub delete: TerminalImageCapabilityState,
    /// Cleanup support.
    pub cleanup: TerminalImageCapabilityState,
}

impl TerminalImageCapabilities {
    fn supports(self, operation: TerminalImageOperation) -> bool {
        let state = match operation {
            TerminalImageOperation::Query => self.query,
            TerminalImageOperation::Transmit => self.transmit,
            TerminalImageOperation::Place => self.place,
            TerminalImageOperation::Replace => self.replace,
            TerminalImageOperation::Delete => self.delete,
            TerminalImageOperation::Cleanup => self.cleanup,
            TerminalImageOperation::TextFallback => {
                return self.protocol == TerminalImageProtocol::Text;
            }
        };
        state == TerminalImageCapabilityState::Supported
    }
}

/// Closed image formats admitted by the terminal payload shell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalImageFormat {
    /// PNG input.
    Png,
    /// JPEG input.
    Jpeg,
    /// GIF input.
    Gif,
    /// WebP input.
    WebP,
}

/// Safe host-owned lookup handle. It is not a path, URL, or provider identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalImageHandle(String);

impl TerminalImageHandle {
    /// Validates a safe lookup handle.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalImageError::UnsafeHandle`] for an empty, oversized,
    /// or non-token handle.
    pub fn try_new(value: &str) -> Result<Self, TerminalImageError> {
        if value.is_empty()
            || value.chars().count() > MAX_SAFE_LABEL_CHARS
            || !value.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '_' | '-')
            })
        {
            return Err(TerminalImageError::UnsafeHandle);
        }
        Ok(Self(value.to_owned()))
    }

    /// Returns the safe handle value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Lowercase BLAKE3 identity for admitted image content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalImageDigest(String);

impl TerminalImageDigest {
    /// Validates one lowercase BLAKE3 digest.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalImageError::MalformedDigest`] unless the value is
    /// exactly one lowercase 64-character BLAKE3 hex digest.
    pub fn try_new(value: &str) -> Result<Self, TerminalImageError> {
        if value.len() != BLAKE3_HEX_CHARS
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(TerminalImageError::MalformedDigest);
        }
        Ok(Self(value.to_owned()))
    }

    /// Computes an identity at the host boundary.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(blake3::hash(bytes).to_hex().to_string())
    }

    /// Returns the lowercase digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Safe metadata used by the pure planner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalImageMetadata {
    /// Host-owned lookup handle.
    pub handle: TerminalImageHandle,
    /// BLAKE3 identity supplied by the admitting host.
    pub content_digest: TerminalImageDigest,
    /// Exact encoded byte count.
    pub byte_count: u64,
    /// Declared source width in pixels.
    pub source_width: u32,
    /// Declared source height in pixels.
    pub source_height: u32,
    /// Admitted image format.
    pub format: TerminalImageFormat,
    /// Bounded transient alternative text. Receipts omit this value.
    pub alt_text: String,
}

/// Terminal-cell geometry supplied by the host layout reducer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalImageGeometry {
    /// Left cell coordinate.
    pub column: u16,
    /// Top cell coordinate.
    pub row: u16,
    /// Width in terminal cells.
    pub width: u16,
    /// Height in terminal cells.
    pub height: u16,
}

/// Named limits exported from terminal-image policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalImagePolicy {
    /// Maximum encoded image bytes.
    pub max_image_bytes: u64,
    /// Maximum prepared protocol bytes.
    pub max_prepared_bytes: u64,
    /// Maximum bytes accepted from one Kitty capability response.
    pub max_capability_response_bytes: u16,
    /// Maximum source width.
    pub max_source_width: u32,
    /// Maximum source height.
    pub max_source_height: u32,
    /// Maximum source pixels.
    pub max_source_pixels: u64,
    /// Maximum decoded bytes passed to upstream limits.
    pub max_decoded_bytes: u64,
    /// Maximum terminal width.
    pub max_terminal_width: u16,
    /// Maximum terminal height.
    pub max_terminal_height: u16,
    /// Maximum terminal cells.
    pub max_terminal_cells: u64,
    /// Maximum Kitty chunks.
    pub max_chunks: u64,
    /// Maximum ordered operations.
    pub max_operations: u16,
    /// Maximum lifecycle entries retained by the caller.
    pub max_lifecycle_entries: u16,
    /// Maximum alternative-text characters.
    pub max_alt_text_chars: u16,
    /// Whether visible bounded text fallback is allowed.
    pub allow_text_fallback: bool,
}

/// Requested lifecycle transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalImageIntent {
    /// Render a fresh image.
    Render,
    /// Replace a prior safe image identity.
    Replace {
        /// Existing safe terminal image identity.
        prior_image_id: String,
    },
    /// Delete the current image.
    Delete,
    /// Complete a pending cleanup obligation.
    Cleanup,
}

/// Inputs to pure terminal-image planning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalImagePlanInput {
    /// Safe image metadata.
    pub image: TerminalImageMetadata,
    /// Selected protocol capability row.
    pub capabilities: TerminalImageCapabilities,
    /// Layout geometry.
    pub geometry: TerminalImageGeometry,
    /// Named policy.
    pub policy: TerminalImagePolicy,
    /// Requested transition.
    pub intent: TerminalImageIntent,
    /// Current bounded lifecycle entry count.
    pub lifecycle_entries: u16,
}

/// Pure ordered effect plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalImagePlan {
    /// Selected protocol.
    pub protocol: TerminalImageProtocol,
    /// Safe image metadata.
    pub image: TerminalImageMetadata,
    /// Geometry admitted by policy.
    pub geometry: TerminalImageGeometry,
    /// Ordered operations.
    pub operations: Vec<TerminalImageOperation>,
    /// Whether failure after allocation requires cleanup.
    pub cleanup_required: bool,
    /// Policy used to admit the plan.
    pub policy: TerminalImagePolicy,
}

/// Stable fail-closed planner and shell errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalImageError {
    /// A safe handle was malformed.
    UnsafeHandle,
    /// A BLAKE3 identity was malformed.
    MalformedDigest,
    /// Image metadata was empty or inconsistent.
    InvalidMetadata,
    /// Policy contained an invalid zero or inconsistent bound.
    InvalidPolicy,
    /// Image bytes exceeded policy.
    ImageBytesExceeded,
    /// Source dimensions exceeded policy.
    SourceGeometryExceeded,
    /// Terminal geometry exceeded policy.
    TerminalGeometryExceeded,
    /// Lifecycle state exceeded policy.
    LifecycleExceeded,
    /// Alternative text exceeded policy.
    AltTextExceeded,
    /// The exact protocol operation is unsupported.
    UnsupportedOperation,
    /// Text fallback is denied.
    FallbackDenied,
    /// Byte resolution failed.
    ByteResolutionFailed,
    /// Resolved bytes differed from admitted metadata.
    ByteIdentityMismatch,
    /// Upstream preparation failed.
    PayloadPreparationFailed,
    /// Terminal writing failed or was partial.
    TerminalWriteFailed,
    /// Host lifecycle operation failed.
    LifecycleOperationFailed,
    /// A capability response exceeded its explicit bound.
    CapabilityResponseExceeded,
    /// A capability response was malformed or negative.
    MalformedCapabilityResponse,
    /// Receipt content contained forbidden data.
    EvidenceLeak,
}

/// Parses one bounded Kitty capability response into a content-free support state.
///
/// # Errors
///
/// Returns [`TerminalImageError::CapabilityResponseExceeded`] when the response
/// exceeds `max_bytes`, or [`TerminalImageError::MalformedCapabilityResponse`]
/// when it is not one complete positive Kitty graphics response.
pub fn parse_kitty_capability_response(
    response: &[u8],
    max_bytes: u16,
) -> Result<TerminalImageCapabilityState, TerminalImageError> {
    if response.len() > usize::from(max_bytes) {
        return Err(TerminalImageError::CapabilityResponseExceeded);
    }
    if response.starts_with(KITTY_RESPONSE_PREFIX)
        && response.ends_with(KITTY_RESPONSE_SUFFIX)
        && response
            .windows(KITTY_RESPONSE_OK.len())
            .any(|window| window == KITTY_RESPONSE_OK)
    {
        Ok(TerminalImageCapabilityState::Supported)
    } else {
        Err(TerminalImageError::MalformedCapabilityResponse)
    }
}

/// Produces a deterministic bounded image plan without host effects.
///
/// # Errors
///
/// Returns a typed [`TerminalImageError`] when metadata, policy, geometry,
/// lifecycle state, capability rows, or fallback policy do not admit the plan.
pub fn plan_terminal_image(
    input: TerminalImagePlanInput,
) -> Result<TerminalImagePlan, TerminalImageError> {
    validate_policy(input.policy)?;
    validate_metadata(&input.image, input.policy)?;
    validate_geometry(input.geometry, input.policy)?;
    if input.lifecycle_entries >= input.policy.max_lifecycle_entries {
        return Err(TerminalImageError::LifecycleExceeded);
    }

    let operations = operations_for_intent(&input.intent, input.capabilities);
    let operations = match operations {
        Ok(value) => value,
        Err(TerminalImageError::UnsupportedOperation)
            if matches!(input.intent, TerminalImageIntent::Render)
                && input.policy.allow_text_fallback =>
        {
            vec![TerminalImageOperation::TextFallback]
        }
        Err(error) => return Err(error),
    };
    let operation_count =
        u16::try_from(operations.len()).map_err(|_| TerminalImageError::UnsupportedOperation)?;
    if operation_count < MIN_OPERATION_COUNT || operation_count > input.policy.max_operations {
        return Err(TerminalImageError::UnsupportedOperation);
    }
    let protocol = if operations == [TerminalImageOperation::TextFallback] {
        TerminalImageProtocol::Text
    } else {
        input.capabilities.protocol
    };
    let cleanup_required = operations.iter().any(|operation| {
        matches!(
            operation,
            TerminalImageOperation::Transmit
                | TerminalImageOperation::Place
                | TerminalImageOperation::Replace
        )
    });
    Ok(TerminalImagePlan {
        protocol,
        image: input.image,
        geometry: input.geometry,
        operations,
        cleanup_required,
        policy: input.policy,
    })
}

fn validate_policy(policy: TerminalImagePolicy) -> Result<(), TerminalImageError> {
    let valid = policy.max_image_bytes >= MIN_IMAGE_BYTES
        && policy.max_prepared_bytes >= policy.max_image_bytes
        && policy.max_capability_response_bytes >= MIN_CELL_COUNT
        && policy.max_source_width >= MIN_DIMENSION
        && policy.max_source_height >= MIN_DIMENSION
        && policy.max_source_pixels >= u64::from(policy.max_source_width)
        && policy.max_decoded_bytes >= policy.max_image_bytes
        && policy.max_terminal_width >= MIN_CELL_COUNT
        && policy.max_terminal_height >= MIN_CELL_COUNT
        && policy.max_terminal_cells >= u64::from(policy.max_terminal_width)
        && policy.max_chunks >= u64::from(MIN_OPERATION_COUNT)
        && policy.max_operations >= MIN_OPERATION_COUNT
        && policy.max_lifecycle_entries >= MIN_OPERATION_COUNT
        && policy.max_alt_text_chars >= MIN_CELL_COUNT;
    if valid {
        Ok(())
    } else {
        Err(TerminalImageError::InvalidPolicy)
    }
}

fn validate_metadata(
    image: &TerminalImageMetadata,
    policy: TerminalImagePolicy,
) -> Result<(), TerminalImageError> {
    if image.byte_count < MIN_IMAGE_BYTES
        || image.source_width < MIN_DIMENSION
        || image.source_height < MIN_DIMENSION
    {
        return Err(TerminalImageError::InvalidMetadata);
    }
    if image.byte_count > policy.max_image_bytes {
        return Err(TerminalImageError::ImageBytesExceeded);
    }
    let source_pixels = u64::from(image.source_width)
        .checked_mul(u64::from(image.source_height))
        .ok_or(TerminalImageError::SourceGeometryExceeded)?;
    if image.source_width > policy.max_source_width
        || image.source_height > policy.max_source_height
        || source_pixels > policy.max_source_pixels
    {
        return Err(TerminalImageError::SourceGeometryExceeded);
    }
    if image.alt_text.chars().count() > usize::from(policy.max_alt_text_chars) {
        return Err(TerminalImageError::AltTextExceeded);
    }
    Ok(())
}

fn validate_geometry(
    geometry: TerminalImageGeometry,
    policy: TerminalImagePolicy,
) -> Result<(), TerminalImageError> {
    if geometry.width < MIN_CELL_COUNT || geometry.height < MIN_CELL_COUNT {
        return Err(TerminalImageError::TerminalGeometryExceeded);
    }
    let cells = u64::from(geometry.width)
        .checked_mul(u64::from(geometry.height))
        .ok_or(TerminalImageError::TerminalGeometryExceeded)?;
    if geometry.width > policy.max_terminal_width
        || geometry.height > policy.max_terminal_height
        || cells > policy.max_terminal_cells
    {
        return Err(TerminalImageError::TerminalGeometryExceeded);
    }
    Ok(())
}

fn operations_for_intent(
    intent: &TerminalImageIntent,
    capabilities: TerminalImageCapabilities,
) -> Result<Vec<TerminalImageOperation>, TerminalImageError> {
    let requested = match intent {
        TerminalImageIntent::Render => match capabilities.protocol {
            TerminalImageProtocol::Kitty => vec![
                TerminalImageOperation::Query,
                TerminalImageOperation::Transmit,
                TerminalImageOperation::Place,
            ],
            TerminalImageProtocol::Iterm2 | TerminalImageProtocol::Sixel => {
                vec![TerminalImageOperation::Transmit]
            }
            TerminalImageProtocol::Text => vec![TerminalImageOperation::TextFallback],
        },
        TerminalImageIntent::Replace { prior_image_id } => {
            validate_lifecycle_id(prior_image_id)?;
            vec![TerminalImageOperation::Replace]
        }
        TerminalImageIntent::Delete => vec![TerminalImageOperation::Delete],
        TerminalImageIntent::Cleanup => vec![TerminalImageOperation::Cleanup],
    };
    if requested
        .iter()
        .all(|operation| capabilities.supports(*operation))
    {
        Ok(requested)
    } else {
        Err(TerminalImageError::UnsupportedOperation)
    }
}

fn validate_lifecycle_id(value: &str) -> Result<(), TerminalImageError> {
    if value.is_empty()
        || value.chars().count() > MAX_SAFE_LABEL_CHARS
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        Err(TerminalImageError::InvalidMetadata)
    } else {
        Ok(())
    }
}

/// Host-owned byte resolver. Implementations retain filesystem, network, cache,
/// and provider authority.
pub trait TerminalImageByteResolver {
    /// Resolves one approved handle under the supplied byte bound.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the handle cannot be resolved within bounds.
    fn resolve(
        &mut self,
        handle: &TerminalImageHandle,
        max_bytes: u64,
    ) -> Result<Vec<u8>, TerminalImageError>;
}

/// Host-owned service for operations that `rat-image` does not encode.
pub trait TerminalImageLifecycleService {
    /// Executes one metadata-only lifecycle operation.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the exact operation is denied, unsupported,
    /// or fails.
    fn execute(
        &mut self,
        operation: TerminalImageOperation,
        image_digest: &TerminalImageDigest,
        geometry: TerminalImageGeometry,
    ) -> Result<(), TerminalImageError>;
}

/// One metadata-only operation outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalImageOperationStatus {
    /// Operation completed.
    Succeeded,
    /// Operation failed before the full plan completed.
    Failed,
    /// Cleanup remained incomplete.
    CleanupIncomplete,
}

/// Host execution report. It retains no raw or prepared content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalImageExecution {
    /// Outcomes in attempted order.
    pub outcomes: Vec<(TerminalImageOperation, TerminalImageOperationStatus)>,
    /// Final status.
    pub final_status: TerminalImageOperationStatus,
    /// Exact prepared bytes written for transmit operations.
    pub prepared_bytes_written: u64,
}

/// Executes an admitted plan through explicit services and an explicit writer.
///
/// `rat-image` owns content inspection, bounded protocol preparation, and short
/// write handling for transmit operations. This shell never opens a terminal or
/// reads process environment.
pub fn execute_terminal_image_plan<R, L, W>(
    plan: &TerminalImagePlan,
    resolver: &mut R,
    lifecycle: &mut L,
    writer: &mut W,
) -> TerminalImageExecution
where
    R: TerminalImageByteResolver,
    L: TerminalImageLifecycleService,
    W: Write,
{
    let mut outcomes = Vec::with_capacity(plan.operations.len());
    let mut prepared_bytes_written = 0_u64;
    for operation in &plan.operations {
        let result = if *operation == TerminalImageOperation::Transmit {
            execute_transmit(plan, resolver, writer).map(|written| {
                prepared_bytes_written = written;
            })
        } else if *operation == TerminalImageOperation::TextFallback {
            writer
                .write_all(plan.image.alt_text.as_bytes())
                .and_then(|()| writer.write_all(b"\n"))
                .and_then(|()| writer.flush())
                .map_err(|_| TerminalImageError::TerminalWriteFailed)
        } else {
            lifecycle.execute(*operation, &plan.image.content_digest, plan.geometry)
        };
        if result.is_err() {
            outcomes.push((*operation, TerminalImageOperationStatus::Failed));
            let final_status = attempt_cleanup(plan, lifecycle, &mut outcomes);
            return TerminalImageExecution {
                outcomes,
                final_status,
                prepared_bytes_written,
            };
        }
        outcomes.push((*operation, TerminalImageOperationStatus::Succeeded));
    }
    TerminalImageExecution {
        outcomes,
        final_status: TerminalImageOperationStatus::Succeeded,
        prepared_bytes_written,
    }
}

fn execute_transmit<R: TerminalImageByteResolver, W: Write>(
    plan: &TerminalImagePlan,
    resolver: &mut R,
    writer: &mut W,
) -> Result<u64, TerminalImageError> {
    let bytes = resolver
        .resolve(&plan.image.handle, plan.policy.max_image_bytes)
        .map_err(|_| TerminalImageError::ByteResolutionFailed)?;
    let byte_count =
        u64::try_from(bytes.len()).map_err(|_| TerminalImageError::ImageBytesExceeded)?;
    if byte_count != plan.image.byte_count
        || TerminalImageDigest::from_bytes(&bytes) != plan.image.content_digest
    {
        return Err(TerminalImageError::ByteIdentityMismatch);
    }
    let profile = rats_limits(plan)?;
    let protocol = rats_protocol(plan.protocol)?;
    let prepared = rat_image::prepare_terminal_image(
        rat_image::CheckedTerminalImageRequest {
            data: &bytes,
            protocol,
            bounds: rat_image::TerminalImageBounds::new(
                rat_image::TerminalCellWidth::new(plan.geometry.width),
                rat_image::TerminalCellHeight::new(plan.geometry.height),
            ),
        },
        &profile,
    )
    .map_err(|_| TerminalImageError::PayloadPreparationFailed)?;
    let outcome = rat_image::write_prepared_terminal_image(writer, &prepared)
        .map_err(|_| TerminalImageError::TerminalWriteFailed)?;
    Ok(outcome.bytes_written)
}

fn rats_limits(
    plan: &TerminalImagePlan,
) -> Result<rat_image::TerminalImageLimits, TerminalImageError> {
    let formats = [rats_format(plan.image.format)];
    let protocols = [rats_protocol(plan.protocol)?];
    rat_image::TerminalImageLimits::try_new(
        rat_image::TerminalImageLimitInput {
            max_encoded_bytes: plan.policy.max_image_bytes,
            max_terminal_width: plan.policy.max_terminal_width,
            max_terminal_height: plan.policy.max_terminal_height,
            max_terminal_cells: plan.policy.max_terminal_cells,
            max_source_width: plan.policy.max_source_width,
            max_source_height: plan.policy.max_source_height,
            max_source_pixels: plan.policy.max_source_pixels,
            max_decoded_bytes: plan.policy.max_decoded_bytes,
            max_prepared_bytes: plan.policy.max_prepared_bytes,
            max_chunks: plan.policy.max_chunks,
            max_palette_colors: MAX_PALETTE_COLORS,
            max_work_units: MAX_WORK_UNITS,
        },
        &protocols,
        &formats,
    )
    .map_err(|_| TerminalImageError::InvalidPolicy)
}

const fn rats_protocol(
    protocol: TerminalImageProtocol,
) -> Result<rat_image::ImageProtocol, TerminalImageError> {
    match protocol {
        TerminalImageProtocol::Kitty => Ok(rat_image::ImageProtocol::Kitty),
        TerminalImageProtocol::Iterm2 => Ok(rat_image::ImageProtocol::Iterm2),
        TerminalImageProtocol::Sixel => Ok(rat_image::ImageProtocol::Sixel),
        TerminalImageProtocol::Text => Err(TerminalImageError::UnsupportedOperation),
    }
}

const fn rats_format(format: TerminalImageFormat) -> rat_image::TerminalImageFormat {
    match format {
        TerminalImageFormat::Png => rat_image::TerminalImageFormat::Png,
        TerminalImageFormat::Jpeg => rat_image::TerminalImageFormat::Jpeg,
        TerminalImageFormat::Gif => rat_image::TerminalImageFormat::Gif,
        TerminalImageFormat::WebP => rat_image::TerminalImageFormat::WebP,
    }
}

fn attempt_cleanup<L: TerminalImageLifecycleService>(
    plan: &TerminalImagePlan,
    lifecycle: &mut L,
    outcomes: &mut Vec<(TerminalImageOperation, TerminalImageOperationStatus)>,
) -> TerminalImageOperationStatus {
    if !plan.cleanup_required {
        return TerminalImageOperationStatus::Failed;
    }
    if lifecycle
        .execute(
            TerminalImageOperation::Cleanup,
            &plan.image.content_digest,
            plan.geometry,
        )
        .is_ok()
    {
        outcomes.push((
            TerminalImageOperation::Cleanup,
            TerminalImageOperationStatus::Succeeded,
        ));
        TerminalImageOperationStatus::Failed
    } else {
        outcomes.push((
            TerminalImageOperation::Cleanup,
            TerminalImageOperationStatus::CleanupIncomplete,
        ));
        TerminalImageOperationStatus::CleanupIncomplete
    }
}

/// Metadata-only deterministic receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalImageReceipt {
    /// Receipt schema.
    pub schema: &'static str,
    /// Selected Rats cohort.
    pub rats_revision: &'static str,
    /// Selected protocol label.
    pub protocol: &'static str,
    /// Content identity.
    pub content_digest: String,
    /// Encoded input byte count.
    pub image_bytes: u64,
    /// Prepared bytes written.
    pub prepared_bytes_written: u64,
    /// Geometry.
    pub geometry: TerminalImageGeometry,
    /// Ordered operation labels.
    pub operations: Vec<&'static str>,
    /// Final status.
    pub final_status: &'static str,
    /// Deterministic receipt identity.
    pub receipt_digest: String,
    /// Explicit non-claims.
    pub non_claims: [&'static str; 4],
}

/// Projects a deterministic metadata-only receipt.
#[must_use]
pub fn project_terminal_image_receipt(
    plan: &TerminalImagePlan,
    execution: &TerminalImageExecution,
) -> TerminalImageReceipt {
    let operations: Vec<&'static str> = execution
        .outcomes
        .iter()
        .map(|(operation, _)| operation.label())
        .collect();
    let final_status = match execution.final_status {
        TerminalImageOperationStatus::Succeeded => "succeeded",
        TerminalImageOperationStatus::Failed => "failed",
        TerminalImageOperationStatus::CleanupIncomplete => "cleanup-incomplete",
    };
    let receipt_digest = receipt_identity(plan, execution, final_status);
    TerminalImageReceipt {
        schema: RECEIPT_SCHEMA,
        rats_revision: RATS_IMAGE_COHORT_REVISION,
        protocol: plan.protocol.label(),
        content_digest: plan.image.content_digest.as_str().to_owned(),
        image_bytes: plan.image.byte_count,
        prepared_bytes_written: execution.prepared_bytes_written,
        geometry: plan.geometry,
        operations,
        final_status,
        receipt_digest,
        non_claims: [
            "no live-terminal or emulator-parity claim",
            "no image-decoder correctness or sandbox claim",
            "no video, audio, provider-availability, or persistence claim",
            "no production-readiness or release-eligibility claim",
        ],
    }
}

fn receipt_identity(
    plan: &TerminalImagePlan,
    execution: &TerminalImageExecution,
    final_status: &str,
) -> String {
    let mut hasher = blake3::Hasher::new();
    update_field(&mut hasher, RECEIPT_DOMAIN.as_bytes());
    update_field(&mut hasher, RATS_IMAGE_COHORT_REVISION.as_bytes());
    update_field(&mut hasher, plan.protocol.label().as_bytes());
    update_field(&mut hasher, plan.image.content_digest.as_str().as_bytes());
    update_field(&mut hasher, &plan.image.byte_count.to_le_bytes());
    update_field(&mut hasher, &execution.prepared_bytes_written.to_le_bytes());
    update_field(&mut hasher, &plan.geometry.column.to_le_bytes());
    update_field(&mut hasher, &plan.geometry.row.to_le_bytes());
    update_field(&mut hasher, &plan.geometry.width.to_le_bytes());
    update_field(&mut hasher, &plan.geometry.height.to_le_bytes());
    for (operation, status) in &execution.outcomes {
        update_field(&mut hasher, operation.label().as_bytes());
        update_field(&mut hasher, format!("{status:?}").as_bytes());
    }
    update_field(&mut hasher, final_status.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn update_field(hasher: &mut blake3::Hasher, field: &[u8]) {
    hasher.update(&u64::try_from(field.len()).unwrap_or(u64::MAX).to_le_bytes());
    hasher.update(field);
}

/// Rejects receipt candidates that contain terminal content or unsafe host data.
///
/// # Errors
///
/// Returns [`TerminalImageError::EvidenceLeak`] when the candidate contains a
/// forbidden terminal, host, provider, credential, path, or payload token.
pub fn validate_terminal_image_evidence(text: &str) -> Result<(), TerminalImageError> {
    let normalized = text.to_ascii_lowercase();
    if FORBIDDEN_EVIDENCE_TOKENS
        .iter()
        .any(|token| normalized.contains(token))
    {
        Err(TerminalImageError::EvidenceLeak)
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[path = "pixeltests.rs"]
mod tests;
