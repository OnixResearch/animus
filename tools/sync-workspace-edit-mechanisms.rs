#!/usr/bin/env -S nix shell github:nix-community/fenix/092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain nixpkgs/20535e48e12c86043b577b8518234ff5dbb26957#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const EXPECTED_ARGUMENT_COUNT: usize = 4;
const SELF_TEST_ARGUMENT_COUNT: usize = 2;
const SELF_TEST_FLAG: &str = "--self-test";
const CAP_ROOT_REVISION: &str = "8991e2572578c16e590c2f1c0915db6c46c5d9cb";
const PUBLICATION_REVISION: &str = "c57f01d9d80f27d4893961d75a31bf9c95d1f6c8";
const CAP_ROOT_DIRECTORY: &str = "vendor/cap-root-core-selected";
const PUBLICATION_DIRECTORY: &str = "vendor/durable-file-publication-selected";
const CAP_ROOT_ENTRY_COUNT: usize = 4;
const PUBLICATION_ENTRY_COUNT: usize = 8;

const CAP_ROOT_ENTRIES: &[(&str, &str)] = &[
    ("Cargo.toml", "Cargo.toml"),
    (
        "crates/cap-root-core/Cargo.toml",
        "crates/cap-root-core/Cargo.toml",
    ),
    (
        "crates/cap-root-core/src/lib.rs",
        "crates/cap-root-core/src/lib.rs",
    ),
    ("LICENSE", "LICENSE"),
];

const PUBLICATION_ENTRIES: &[(&str, &str)] = &[
    ("Cargo.toml", "Cargo.toml"),
    ("src/lib.rs", "src/lib.rs"),
    ("src/core.rs", "src/core.rs"),
    ("src/corpus.rs", "src/corpus.rs"),
    ("src/shell.rs", "src/shell.rs"),
    ("LICENSE", "LICENSE"),
    ("LICENSE-APACHE", "LICENSE-APACHE"),
    ("LICENSE-MIT", "LICENSE-MIT"),
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct ImportEntry {
    revision: &'static str,
    source_path: &'static str,
    destination: PathBuf,
}

fn family_plan(
    destination_root: &Path,
    destination_directory: &str,
    revision: &'static str,
    entries: &'static [(&'static str, &'static str)],
    expected_count: usize,
) -> Result<Vec<ImportEntry>, String> {
    if destination_root.as_os_str().is_empty() {
        return Err(String::from("destination root must be non-empty"));
    }
    if entries.len() != expected_count {
        return Err(String::from("workspace-edit import inventory count drifted"));
    }
    let destination = destination_root.join(destination_directory);
    Ok(entries
        .iter()
        .map(|(source_path, destination_name)| ImportEntry {
            revision,
            source_path,
            destination: destination.join(destination_name),
        })
        .collect())
}

fn import_plan(destination_root: &Path) -> Result<(Vec<ImportEntry>, Vec<ImportEntry>), String> {
    let cap_root = family_plan(
        destination_root,
        CAP_ROOT_DIRECTORY,
        CAP_ROOT_REVISION,
        CAP_ROOT_ENTRIES,
        CAP_ROOT_ENTRY_COUNT,
    )?;
    let publication = family_plan(
        destination_root,
        PUBLICATION_DIRECTORY,
        PUBLICATION_REVISION,
        PUBLICATION_ENTRIES,
        PUBLICATION_ENTRY_COUNT,
    )?;
    Ok((cap_root, publication))
}

fn read_git_object(
    repository: &Path,
    revision: &str,
    source_path: &str,
) -> Result<Vec<u8>, String> {
    let object = format!("{revision}:{source_path}");
    let output = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("show")
        .arg(&object)
        .output()
        .map_err(|error| format!("cannot run git show for {object}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git show failed for {object}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output.stdout)
}

fn execute_import(source_repository: &Path, plan: &[ImportEntry]) -> Result<(), String> {
    if !source_repository.join(".git").exists() {
        return Err(String::from("source is not a Git repository"));
    }
    for entry in plan {
        let bytes = read_git_object(source_repository, entry.revision, entry.source_path)?;
        let parent = entry
            .destination
            .parent()
            .ok_or_else(|| String::from("destination has no parent"))?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create destination parent: {error}"))?;
        fs::write(&entry.destination, bytes)
            .map_err(|error| format!("cannot write imported file: {error}"))?;
        println!(
            "{}@{}:{} -> {}",
            source_repository.display(),
            entry.revision,
            entry.source_path,
            entry.destination.display()
        );
    }
    Ok(())
}

fn self_test() -> Result<(), String> {
    let destination = Path::new("/tmp/animus-workspace-edit-mechanisms");
    let (cap_root, publication) = import_plan(destination)?;
    assert_eq!(cap_root.len(), CAP_ROOT_ENTRY_COUNT);
    assert_eq!(publication.len(), PUBLICATION_ENTRY_COUNT);
    assert!(
        cap_root
            .iter()
            .chain(publication.iter())
            .all(|entry| entry.destination.starts_with(destination))
    );
    assert!(import_plan(Path::new("")).is_err());
    println!("workspace-edit mechanism import self-test passed");
    Ok(())
}

fn run(arguments: &[String]) -> Result<(), String> {
    if arguments.len() == SELF_TEST_ARGUMENT_COUNT && arguments[1] == SELF_TEST_FLAG {
        return self_test();
    }
    if arguments.len() != EXPECTED_ARGUMENT_COUNT {
        return Err(String::from(
            "usage: sync-workspace-edit-mechanisms.rs <cap-root-repository> <durable-publication-repository> <animus-root>",
        ));
    }
    let cap_root_repository = Path::new(&arguments[1]);
    let publication_repository = Path::new(&arguments[2]);
    let destination_root = Path::new(&arguments[3]);
    let (cap_root, publication) = import_plan(destination_root)?;
    execute_import(cap_root_repository, &cap_root)?;
    execute_import(publication_repository, &publication)
}

fn main() -> ExitCode {
    let arguments = env::args().collect::<Vec<_>>();
    match run(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
