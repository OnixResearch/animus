//! Print one bounded native evaluation conformance summary.

use animus_eval_conformance::{
    parse_conformance_profile, render_conformance_summary, run_evaluation_conformance,
};

const PROFILE: &[u8] =
    include_bytes!("../../../config/generated/evaluation-composition-conformance.json");
const FAILURE: i32 = 1;

fn main() {
    let profile = match parse_conformance_profile(PROFILE) {
        Ok(profile) => profile,
        Err(error) => {
            eprintln!("conformance profile blocked: {error:?}");
            std::process::exit(FAILURE);
        }
    };
    match run_evaluation_conformance(&profile) {
        Ok(summary) => print!("{}", render_conformance_summary(&summary)),
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                eprintln!(
                    "family={} expected={} observed={}",
                    diagnostic.family, diagnostic.expected, diagnostic.observed
                );
            }
            std::process::exit(FAILURE);
        }
    }
}
