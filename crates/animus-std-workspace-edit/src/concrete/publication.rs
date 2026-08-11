struct StageNames<'a, T> {
    service: &'a mut T,
}

impl<T: crate::concrete::StageNameService> durable_file_publication::shell::StageNameSource
    for StageNames<'_, T>
{
    fn next_stage_leaf(&mut self) -> String {
        self.service.next_stage_leaf()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct MechanismInput<'a, 'request> {
    pub(crate) admitted: &'a cap_root_core::RootRelativePath,
    pub(crate) request: &'a crate::concrete::HostRequest<'request>,
    pub(crate) bytes: &'a [u8],
}

fn request_for_mechanism(
    request: &crate::concrete::HostRequest<'_>,
    leaf: &str,
    bytes: usize,
) -> durable_file_publication::core::PublicationRequest {
    durable_file_publication::core::PublicationRequest {
        destination_leaf: String::from(leaf),
        payload_bytes: bytes,
        maximum_payload_bytes: crate::model::MAX_SOURCE_BYTES,
        collision_attempt_limit: request.collision_attempt_limit,
        final_mode: request.final_mode,
        replacement: durable_file_publication::core::ReplacementMode::Replace,
        durability: if request.require_durability {
            durable_file_publication::core::DurabilityMode::DurabilityRequired
        } else {
            durable_file_publication::core::DurabilityMode::VisibilityOnly
        },
    }
}

pub(crate) fn publish_with_io(
    io: &mut impl durable_file_publication::shell::PublicationIo,
    names: &mut impl crate::concrete::StageNameService,
    input: MechanismInput<'_, '_>,
) -> Result<durable_file_publication::core::Classification, crate::concrete::HostError> {
    let leaf = input
        .admitted
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or(crate::concrete::HostError::PathDenied)?;
    let request = request_for_mechanism(input.request, leaf, input.bytes.len());
    let mut names = StageNames { service: names };
    durable_file_publication::shell::publish_one_file(io, &mut names, &request, input.bytes)
        .map_err(|_| crate::concrete::HostError::PublicationFailed)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) fn publish_capability_relative(
    root: &cap_std::fs::Dir,
    names: &mut impl crate::concrete::StageNameService,
    input: MechanismInput<'_, '_>,
) -> Result<durable_file_publication::core::Classification, crate::concrete::HostError> {
    let directory = match input.admitted.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => root
            .open(parent)
            .map_err(|_| crate::concrete::HostError::ReadFailed)?
            .into_std(),
        Some(_) | None => root
            .open(std::path::Path::new("."))
            .map_err(|_| crate::concrete::HostError::ReadFailed)?
            .into_std(),
    };
    let mut io = durable_file_publication::shell::LinuxDirectory::from_open_directory(directory)
        .map_err(|_| crate::concrete::HostError::PublicationFailed)?;
    publish_with_io(&mut io, names, input)
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub(crate) fn publish_capability_relative(
    _root: &cap_std::fs::Dir,
    _names: &mut impl crate::concrete::StageNameService,
    _input: MechanismInput<'_, '_>,
) -> Result<durable_file_publication::core::Classification, crate::concrete::HostError> {
    Err(crate::concrete::HostError::UnsupportedPlatform)
}
