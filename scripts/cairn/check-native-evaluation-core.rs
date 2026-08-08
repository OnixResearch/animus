use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;
const EXPECTED_ARGUMENTS: usize = 2;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != EXPECTED_ARGUMENTS {
        eprintln!("usage: check-native-evaluation-core <root>");
        process::exit(FAILURE);
    }
    let root = PathBuf::from(&arguments[1]);
    match validate_repository(&root) {
        Ok(()) => {
            println!("native evaluation core static check: pass");
            process::exit(SUCCESS);
        }
        Err(error) => {
            eprintln!("native evaluation core static check: {error}");
            process::exit(FAILURE);
        }
    }
}

fn validate_repository(root: &Path) -> Result<(), String> {
    let crate_root = read(root, "crates/animus-eval-core/src/lib.rs")?;
    let task = read(root, "crates/animus-eval-core/src/task.rs")?;
    let recipe = read(root, "crates/animus-eval-core/src/recipe.rs")?;
    let episode = read(root, "crates/animus-eval-core/src/episode.rs")?;
    let reward = read(root, "crates/animus-eval-core/src/reward.rs")?;
    let resume = read(root, "crates/animus-eval-core/src/resume.rs")?;
    let receipt = read(root, "crates/animus-eval-core/src/receipt.rs")?;
    let manifest = read(root, "crates/animus-eval-core/Cargo.toml")?;
    let harbor = read(root, "crates/animus-runtime/src/harbor/eval.rs")?;
    validate_source_shapes(SourceShapes {
        crate_root: &crate_root,
        task: &task,
        recipe: &recipe,
        episode: &episode,
        reward: &reward,
        resume: &resume,
        receipt: &receipt,
        manifest: &manifest,
        harbor: &harbor,
    })?;
    for path in [
        "config/native-evaluation-policy.ncl",
        "config/generated/native-evaluation-policy.json",
        "fixtures/native-evaluation/positive.ncl",
        "fixtures/native-evaluation/negative.ncl",
        "docs/native-evaluation-core.md",
    ] {
        let content = read(root, path)?;
        if content.trim().is_empty() {
            return Err(format!("{path} is empty"));
        }
    }
    let negative = read(root, "fixtures/native-evaluation/negative.ncl")?;
    for required in [
        "infinite-global-shuffle",
        "copied-trace-body",
        "shared-judge-completion",
        "nan-reward",
        "torn-resume",
        "raw-prompt-receipt",
        "release-overclaim",
    ] {
        require(&negative, required, "negative fixture")?;
    }
    Ok(())
}

struct SourceShapes<'a> {
    crate_root: &'a str,
    task: &'a str,
    recipe: &'a str,
    episode: &'a str,
    reward: &'a str,
    resume: &'a str,
    receipt: &'a str,
    manifest: &'a str,
    harbor: &'a str,
}

fn validate_source_shapes(source: SourceShapes<'_>) -> Result<(), String> {
    require(source.crate_root, "#![no_std]", "crate root")?;
    require(source.crate_root, "#![forbid(unsafe_code)]", "crate root")?;
    require(source.task, "pub fn plan_taskset", "task core")?;
    require(source.recipe, "pub fn plan_environment", "recipe core")?;
    require(source.recipe, "pub fn plan_recipe", "recipe core")?;
    require(source.recipe, "pub fn reduce_pass_at_n", "recipe core")?;
    require(source.episode, "pub fn reduce_episode", "episode core")?;
    require(
        source.reward,
        "pub fn reduce_reward_dimensions",
        "reward core",
    )?;
    require(source.resume, "pub fn evaluate_resume", "resume core")?;
    require(
        source.receipt,
        "pub fn project_evaluation_receipt",
        "receipt core",
    )?;
    require(
        source.harbor,
        "animus_eval_core::plan_compat_trials",
        "Harbor migration",
    )?;
    require(
        source.harbor,
        "animus_eval_core::reduce_compat_rewards",
        "Harbor migration",
    )?;
    for forbidden in [
        "use std::",
        "std::fs",
        "std::process",
        "std::net",
        "std::env",
        "SystemTime",
        "Instant::now",
        "TcpStream",
        "Command::",
    ] {
        for (name, value) in [
            ("crate root", source.crate_root),
            ("task", source.task),
            ("recipe", source.recipe),
            ("episode", source.episode),
            ("reward", source.reward),
            ("resume", source.resume),
            ("receipt", source.receipt),
        ] {
            if value.contains(forbidden) {
                return Err(format!(
                    "{name} contains prohibited authority token {forbidden}"
                ));
            }
        }
    }
    for forbidden_dependency in [
        "tokio",
        "reqwest",
        "hyper",
        "serde",
        "python",
        "harbor",
        "docker",
        "kubernetes",
    ] {
        if source
            .manifest
            .to_ascii_lowercase()
            .contains(forbidden_dependency)
        {
            return Err(format!(
                "core manifest contains prohibited dependency {forbidden_dependency}"
            ));
        }
    }
    Ok(())
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn require(source: &str, marker: &str, owner: &str) -> Result<(), String> {
    if !source.contains(marker) {
        return Err(format!("{owner} is missing {marker}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_complete_authority_free_shapes() {
        let shared = "#![no_std]\n#![forbid(unsafe_code)]";
        let result = validate_source_shapes(SourceShapes {
            crate_root: shared,
            task: "pub fn plan_taskset",
            recipe: "pub fn plan_environment pub fn plan_recipe pub fn reduce_pass_at_n",
            episode: "pub fn reduce_episode",
            reward: "pub fn reduce_reward_dimensions",
            resume: "pub fn evaluate_resume",
            receipt: "pub fn project_evaluation_receipt",
            manifest: "[dependencies]\nblake3 = {}",
            harbor: "animus_eval_core::plan_compat_trials animus_eval_core::reduce_compat_rewards",
        });
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn rejects_host_authority_and_missing_migration() {
        let result = validate_source_shapes(SourceShapes {
            crate_root: "#![no_std]\n#![forbid(unsafe_code)]",
            task: "use std::fs; pub fn plan_taskset",
            recipe: "pub fn plan_environment pub fn plan_recipe pub fn reduce_pass_at_n",
            episode: "pub fn reduce_episode",
            reward: "pub fn reduce_reward_dimensions",
            resume: "pub fn evaluate_resume",
            receipt: "pub fn project_evaluation_receipt",
            manifest: "[dependencies]\ntokio = {}",
            harbor: "not migrated",
        });
        assert!(result.is_err());
    }
}
