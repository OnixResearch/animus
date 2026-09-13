const MODEL: &str = "reviewed-model";

#[test]
fn both_orders_and_literal_model_values_preserve_the_grammar() {
    for values in [
        vec!["--model", MODEL, "--credential-stdin"],
        vec!["--credential-stdin", "--model", MODEL],
        vec!["--model", "--", "--credential-stdin"],
        vec!["--model", "--credential-stdin", "--credential-stdin"],
        vec!["--model", "", "--credential-stdin"],
    ] {
        let raw = clap_lex::RawArgs::new(values);
        let parsed = super::parse(&raw, &mut raw.cursor()).expect("accepted smoke grammar");
        assert!(parsed.has_credential_stdin);
    }
}

#[test]
fn the_program_name_is_not_a_model_flag() {
    let raw = clap_lex::RawArgs::new(["--model", "--model", MODEL, "--credential-stdin"]);
    let mut cursor = raw.cursor();
    let program = raw.next_os(&mut cursor).expect("program argument");
    assert_eq!(program, "--model");
    let parsed = super::parse(&raw, &mut cursor).expect("arguments after the program name");
    assert_eq!(parsed.model, MODEL);
}

#[test]
fn refused_inputs_preserve_the_safe_issue_and_precedence() {
    let cases: &[(&[&str], &str)] = &[
        (&[], "missing_credential_stdin"),
        (&["--credential-stdin"], "missing_model"),
        (&["--model"], "missing_model_value"),
        (&["--model", MODEL], "missing_credential_stdin"),
        (&["--model", MODEL, "--model"], "missing_model_value"),
        (&["--model", MODEL, "--model", MODEL], "duplicate_model"),
        (
            &["--credential-stdin", "--credential-stdin"],
            "duplicate_credential_stdin",
        ),
        (
            &["--model=reviewed-model", "--credential-stdin"],
            "unsupported_argument",
        ),
        (&["--credential-stdin=yes"], "unsupported_argument"),
        (&["--"], "unsupported_argument"),
        (&["-m", MODEL], "unsupported_argument"),
        (&["private-argument-text"], "unsupported_argument"),
        (&["--help"], "unsupported_argument"),
    ];
    for (values, expected) in cases {
        let raw = clap_lex::RawArgs::new(values.iter().copied());
        let error = super::parse(&raw, &mut raw.cursor()).expect_err("rejected smoke grammar");
        assert_eq!(error, *expected);
        assert!(!error.contains("private-argument-text"));
    }
}

#[cfg(unix)]
#[test]
fn malformed_encoding_is_rejected_without_a_panic_or_echo() {
    use std::os::unix::ffi::OsStringExt;
    let invalid = std::ffi::OsString::from_vec(vec![u8::MAX]);
    let raw = clap_lex::RawArgs::new([
        std::ffi::OsString::from("--model"),
        invalid,
        std::ffi::OsString::from("--credential-stdin"),
    ]);
    assert_eq!(
        super::parse(&raw, &mut raw.cursor()).expect_err("invalid encoding"),
        "unsupported_argument"
    );
}
