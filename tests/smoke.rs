use std::process::Command;

#[test]
fn debug_build_and_help() {
    assert!(
        Command::new("cargo")
            .arg("build")
            .status()
            .expect("build failed")
            .success()
    );

    assert!(
        Command::new("cargo")
            .args(["run", "--", "--help"])
            .status()
            .expect("run failed")
            .success()
    );
}
