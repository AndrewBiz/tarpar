use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tarpar")?;

    cmd.arg("test/file/does_not/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed reading file test/file/does_not/exist",
    ));

    Ok(())
}
