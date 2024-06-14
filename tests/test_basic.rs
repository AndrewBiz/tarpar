use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn it_checks_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tarpar")?;

    cmd.arg("test/file/does_not/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed reading file test/file/does_not/exist",
    ));
    Ok(())
}

#[test]
fn it_exits_if_file_is_not_xml() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("sample")?;

    const DATA: &str = r##"just simple text file"##;

    input_file.write_str(DATA)?;

    let mut cmd = Command::cargo_bin("tarpar")?;
    cmd.arg(input_file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(r##"Failed parsing"##));
    Ok(())
}

#[test]
fn it_exits_if_file_is_not_drawio() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("sample")?;

    const DATA: &str = r##"
<my_xml host="Electron">
    <object name="L1 diagram">
    </object>
</my_xml>"##;

    input_file.write_str(DATA)?;

    let mut cmd = Command::cargo_bin("tarpar")?;
    cmd.arg(input_file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(r##"File is not drawio"##));
    Ok(())
}
