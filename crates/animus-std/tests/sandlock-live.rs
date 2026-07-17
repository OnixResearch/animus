//! Opt-in real-Linux behavior checks for the pinned Sandlock cohort.

#![cfg(all(target_os = "linux", feature = "sandlock-host"))]

#[path = "sandlock_live/fixture.rs"]
mod fixture;
#[path = "sandlock_live/runner.rs"]
mod runner;

#[tokio::test(flavor = "current_thread")]
#[ignore = "requires explicit ANIMUS_SANDLOCK_LINUX_LIVE=1 host qualification"]
async fn real_linux_denials_clean_environment_cow_and_resource_limit_are_observed() {
    if std::env::var("ANIMUS_SANDLOCK_LINUX_LIVE").as_deref() != Ok("1") {
        return;
    }
    runner::run_all().await;
}
