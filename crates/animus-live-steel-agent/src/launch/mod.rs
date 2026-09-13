//! Exact positional grammar for the opt-in binary.

const USAGE: &str = "usage: animus-live-steel-agent <store-root> <prompt> < credential";

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Admitted {
    pub(super) store_root: std::path::PathBuf,
    pub(super) prompt: String,
}

pub(super) fn parse(raw: &clap_lex::RawArgs) -> Result<Admitted, &'static str> {
    let mut cursor = raw.cursor();
    let _program = raw.next_os(&mut cursor);
    let store = raw.next(&mut cursor).ok_or(USAGE)?;
    let prompt = raw.next(&mut cursor).ok_or(USAGE)?;
    if raw.next(&mut cursor).is_some() {
        return Err(USAGE);
    }
    let store = store.to_value().map_err(|_error| USAGE)?;
    let prompt = prompt.to_value().map_err(|_error| USAGE)?;
    assert!(raw.peek(&cursor).is_none());
    Ok(Admitted {
        store_root: std::path::PathBuf::from(store),
        prompt: String::from(prompt),
    })
}

#[cfg(test)]
mod tests;
