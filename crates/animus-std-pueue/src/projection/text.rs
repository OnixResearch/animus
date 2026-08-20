use super::PueueProjectionError;

pub(super) fn tail_lines(value: &str, max_lines: u32) -> Result<String, PueueProjectionError> {
    let max_lines =
        usize::try_from(max_lines).map_err(|_error| PueueProjectionError::BoundExceeded)?;
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.len().saturating_sub(max_lines);
    Ok(lines[start..].join("\n"))
}

pub(super) fn tail_bytes(value: &str, max_bytes: u32) -> Result<String, PueueProjectionError> {
    let max_bytes =
        usize::try_from(max_bytes).map_err(|_error| PueueProjectionError::BoundExceeded)?;
    if value.len() <= max_bytes {
        return Ok(String::from(value));
    }
    let mut start = value.len().saturating_sub(max_bytes);
    while !value.is_char_boundary(start) {
        start = start.saturating_add(1);
    }
    Ok(String::from(&value[start..]))
}

pub(super) fn count_lines(value: &str) -> Result<u32, PueueProjectionError> {
    let count = if value.is_empty() {
        0
    } else {
        value.lines().count()
    };
    u32::try_from(count).map_err(|_error| PueueProjectionError::BoundExceeded)
}
