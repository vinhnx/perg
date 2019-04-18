use std::process::Command; // run programs
use assert_cmd::prelude::*; // add methods on commands
use predicates::prelude::*; // used for writing assertion

#[test]
fn file_does_not_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("foobar")
        .arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

use tempfile::NamedTempFile;
use std::io::{self, Write};

#[test]
fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;
    let mut cmd = Command::main_binary()?;
    cmd.arg("test")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));
    Ok(())
}