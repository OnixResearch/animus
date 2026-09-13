//! Exact smoke grammar over the locked argument lexer.

const MODEL_FLAG: &str = "model";
const CREDENTIAL_FLAG: &str = "credential-stdin";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Admitted {
    pub(super) model: String,
    pub(super) has_credential_stdin: bool,
}

pub(super) fn parse(
    raw: &clap_lex::RawArgs,
    cursor: &mut clap_lex::ArgCursor,
) -> Result<Admitted, String> {
    let mut count_cursor = raw.cursor();
    let argument_count = Iterator::count(raw.remaining(&mut count_cursor));
    let mut model = None;
    let mut has_credential_stdin = false;
    for _argument in 0..argument_count {
        let Some(argument) = raw.next(cursor) else {
            break;
        };
        let Some((Ok(flag), None)) = argument.to_long() else {
            return Err(String::from("unsupported_argument"));
        };
        match flag {
            MODEL_FLAG => {
                let value = raw
                    .next(cursor)
                    .ok_or_else(|| String::from("missing_model_value"))?;
                let text = value
                    .to_value()
                    .map_err(|_error| String::from("unsupported_argument"))?;
                if model.replace(String::from(text)).is_some() {
                    return Err(String::from("duplicate_model"));
                }
            }
            CREDENTIAL_FLAG => {
                if has_credential_stdin {
                    return Err(String::from("duplicate_credential_stdin"));
                }
                has_credential_stdin = true;
            }
            _unsupported => return Err(String::from("unsupported_argument")),
        }
    }
    if !has_credential_stdin {
        return Err(String::from("missing_credential_stdin"));
    }
    assert!(has_credential_stdin);
    assert!(raw.peek(cursor).is_none());
    Ok(Admitted {
        model: model.ok_or_else(|| String::from("missing_model"))?,
        has_credential_stdin,
    })
}

#[cfg(test)]
mod tests;
