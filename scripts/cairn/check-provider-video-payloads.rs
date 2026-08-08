use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;
const REQUIRED_FILES: [&str; 11] = [
    "crates/animus-adapters/src/multimodal/motion.rs",
    "crates/animus-adapters/src/multimodal/compose.rs",
    "crates/animus-adapters/src/multimodal/inputgate.rs",
    "crates/animus-adapters/src/multimodal/wire.rs",
    "config/provider-video-payload-policy.ncl",
    "config/generated/provider-video-payload-policy.json",
    "fixtures/provider-video-payload/positive.ncl",
    "fixtures/provider-video-payload/negative.ncl",
    "fixtures/provider-video-payload/google-gemini-v1beta-files.ncl",
    "docs/provider-video-payloads.md",
    "README.md",
];
const VIDEO_MARKERS: [&str; 13] = [
    "struct VideoPart",
    "struct VideoMetadata",
    "enum VideoMediaType",
    "enum VideoSourceTrust",
    "trait VideoPayloadService",
    "struct VideoPayloadHandle",
    "enum VideoLossClass",
    "struct VideoReceipt",
    "fn validate_video_metadata",
    "MAX_VIDEO_BYTE_COUNT",
    "MAX_VIDEO_DURATION_MS",
    "provider-video-receipt-v1",
    "blake3::hash",
];
const COMPOSE_MARKERS: [&str; 7] = [
    "ContentPart::Video",
    "VideoOutcome",
    "max_video_count",
    "max_aggregate_media_bytes",
    "video_count_budget_exceeded",
    "aggregate_media_budget_exceeded",
    "DuplicateVideoIdentity",
];
const CAPABILITY_MARKERS: [&str; 7] = [
    "InputModality::Video",
    "VideoInputCapabilitiesDraft",
    "service_available",
    "projection_available",
    "OptionalOmissionAllowed",
    "OmittedByPolicy",
    "fn admits",
];
const WIRE_MARKERS: [&str; 8] = [
    "serialize_provider_native_video_fixture",
    "MissingVideoService",
    "LossyGenericConversion",
    "UnrepresentableVideoField",
    "resolve_video_payload",
    "ensure_video_handle_matches",
    "ProviderNativePart::Video",
    "LiveProviderCallRequired",
];
const POLICY_MARKERS: [&str; 10] = [
    "max_descriptor_bytes",
    "max_prompt_video_bytes",
    "max_aggregate_media_bytes",
    "max_duration_ms",
    "max_frame_rate_millihz",
    "max_video_count",
    "exact_provider_profiles",
    "generic_json_video_allowed = false",
    "require_injected_payload_service = true",
    "metadata-only-v1",
];
const FORBIDDEN_DEPENDENCIES: [&str; 8] = [
    "reqwest", "tokio", "image =", "ffmpeg", "gstreamer", "opencv", "mime_guess", "base64 =",
];
const FORBIDDEN_CORE_EFFECTS: [&str; 8] = [
    "std::fs", "std::net", "std::process", "std::env", "File::open", "Command::new",
    "TcpStream", "SystemTime",
];

struct Inputs {
    video: String,
    compose: String,
    capability: String,
    wire: String,
    policy: String,
    provider_fixture: String,
    docs: String,
    readme: String,
    cargo: String,
}

fn parse_root(args: &[String]) -> Result<PathBuf, String> {
    match args {
        [] => Ok(PathBuf::from(".")),
        [flag, root] if flag == "--root" => Ok(PathBuf::from(root)),
        _ => Err("usage: check-provider-video-payloads [--root PATH]".to_string()),
    }
}

fn read_inputs(root: &Path) -> Result<Inputs, String> {
    for relative in REQUIRED_FILES {
        if !root.join(relative).is_file() {
            return Err(format!("missing required file: {relative}"));
        }
    }
    let read = |relative: &str| {
        fs::read_to_string(root.join(relative))
            .map_err(|error| format!("cannot read {relative}: {error}"))
    };
    Ok(Inputs {
        video: read("crates/animus-adapters/src/multimodal/motion.rs")?,
        compose: read("crates/animus-adapters/src/multimodal/compose.rs")?,
        capability: read("crates/animus-adapters/src/multimodal/inputgate.rs")?,
        wire: read("crates/animus-adapters/src/multimodal/wire.rs")?,
        policy: read("config/provider-video-payload-policy.ncl")?,
        provider_fixture: read("fixtures/provider-video-payload/google-gemini-v1beta-files.ncl")?,
        docs: read("docs/provider-video-payloads.md")?,
        readme: read("README.md")?,
        cargo: read("crates/animus-adapters/Cargo.toml")?,
    })
}

fn require_markers(label: &str, text: &str, markers: &[&str]) -> Result<(), String> {
    let missing: Vec<&str> = markers
        .iter()
        .copied()
        .filter(|marker| !text.contains(marker))
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("{label} misses markers: {}", missing.join(", ")))
    }
}

fn reject_markers(label: &str, text: &str, markers: &[&str]) -> Result<(), String> {
    let present: Vec<&str> = markers
        .iter()
        .copied()
        .filter(|marker| text.contains(marker))
        .collect();
    if present.is_empty() {
        Ok(())
    } else {
        Err(format!("{label} contains forbidden markers: {}", present.join(", ")))
    }
}

fn validate(inputs: &Inputs) -> Result<(), String> {
    require_markers("video contract", &inputs.video, &VIDEO_MARKERS)?;
    require_markers("assembly", &inputs.compose, &COMPOSE_MARKERS)?;
    require_markers("capability", &inputs.capability, &CAPABILITY_MARKERS)?;
    require_markers("provider projection", &inputs.wire, &WIRE_MARKERS)?;
    require_markers("Nickel policy", &inputs.policy, &POLICY_MARKERS)?;
    require_markers(
        "provider fixture",
        &inputs.provider_fixture,
        &["google-gemini-v1beta-files-fixture", "fileData", "not-live-api-validation"],
    )?;
    require_markers(
        "documentation",
        &inputs.docs,
        &["Image support does not imply video support", "Generic JSON conversion", "does not prove"],
    )?;
    require_markers("README", &inputs.readme, &["docs/provider-video-payloads.md"])?;
    reject_markers("video core", &inputs.video, &FORBIDDEN_CORE_EFFECTS)?;
    reject_markers("video projection", &inputs.wire, &FORBIDDEN_CORE_EFFECTS)?;
    reject_markers("adapter dependencies", &inputs.cargo, &FORBIDDEN_DEPENDENCIES)?;
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    validate(&read_inputs(&parse_root(args)?)?)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => {
            println!("provider video payload static check: pass");
            ExitCode::from(SUCCESS)
        }
        Err(error) => {
            eprintln!("provider video payload static check: fail: {error}");
            ExitCode::from(FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Inputs, validate};

    fn complete() -> Inputs {
        Inputs {
            video: "struct VideoPart struct VideoMetadata enum VideoMediaType enum VideoSourceTrust trait VideoPayloadService struct VideoPayloadHandle enum VideoLossClass struct VideoReceipt fn validate_video_metadata MAX_VIDEO_BYTE_COUNT MAX_VIDEO_DURATION_MS provider-video-receipt-v1 blake3::hash".to_string(),
            compose: "ContentPart::Video VideoOutcome max_video_count max_aggregate_media_bytes video_count_budget_exceeded aggregate_media_budget_exceeded DuplicateVideoIdentity".to_string(),
            capability: "InputModality::Video VideoInputCapabilitiesDraft service_available projection_available OptionalOmissionAllowed OmittedByPolicy fn admits".to_string(),
            wire: "serialize_provider_native_video_fixture MissingVideoService LossyGenericConversion UnrepresentableVideoField resolve_video_payload ensure_video_handle_matches ProviderNativePart::Video LiveProviderCallRequired".to_string(),
            policy: "max_descriptor_bytes max_prompt_video_bytes max_aggregate_media_bytes max_duration_ms max_frame_rate_millihz max_video_count exact_provider_profiles generic_json_video_allowed = false require_injected_payload_service = true metadata-only-v1".to_string(),
            provider_fixture: "google-gemini-v1beta-files-fixture fileData not-live-api-validation".to_string(),
            docs: "Image support does not imply video support. Generic JSON conversion does not prove provider acceptance.".to_string(),
            readme: "docs/provider-video-payloads.md".to_string(),
            cargo: "blake3 = default-features = false".to_string(),
        }
    }

    #[test]
    fn accepts_complete_safe_surface() {
        assert_eq!(validate(&complete()), Ok(()));
    }

    #[test]
    fn rejects_missing_or_effectful_surface() {
        let mut inputs = complete();
        inputs.video = "std::fs".to_string();
        assert!(validate(&inputs).is_err());
    }
}
