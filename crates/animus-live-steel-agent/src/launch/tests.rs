#[test]
fn positional_values_keep_spaces_empty_strings_and_flag_spellings() {
    for (store, prompt) in [
        ("store root", "a prompt with spaces"),
        ("--", "--help"),
        ("--model=value", "--credential-stdin"),
        ("", ""),
    ] {
        let raw = clap_lex::RawArgs::new(["agent", store, prompt]);
        let admitted = super::parse(&raw).expect("exact positional grammar");
        assert_eq!(admitted.store_root, std::path::PathBuf::from(store));
        assert_eq!(admitted.prompt, prompt);
    }
}

#[test]
fn missing_and_extra_values_return_only_the_existing_usage() {
    for values in [
        vec!["agent"],
        vec!["agent", "private-store"],
        vec!["agent", "private-store", "private-prompt", "extra"],
    ] {
        let raw = clap_lex::RawArgs::new(values);
        assert_eq!(super::parse(&raw), Err(super::USAGE));
    }
}

#[cfg(unix)]
#[test]
fn malformed_encoding_returns_usage_instead_of_panicking() {
    use std::os::unix::ffi::OsStringExt;
    let raw = clap_lex::RawArgs::new([
        std::ffi::OsString::from("agent"),
        std::ffi::OsString::from_vec(vec![u8::MAX]),
        std::ffi::OsString::from("private-prompt"),
    ]);
    assert_eq!(super::parse(&raw), Err(super::USAGE));
}
