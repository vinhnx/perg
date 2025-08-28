use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_basic_search() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "hello world").unwrap();
    writeln!(file, "goodbye world").unwrap();
    writeln!(file, "no match here").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("world")
       .arg(file_path)
       .assert()
       .success()
       .stdout(predicate::str::contains("hello world"))
       .stdout(predicate::str::contains("goodbye world"))
       .stdout(predicate::str::diff("no match here").not().normalize());
}

#[test]
fn test_line_numbers() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "first line").unwrap();
    writeln!(file, "hello world").unwrap();
    writeln!(file, "third line").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("-n")
       .arg("world")
       .arg(file_path)
       .assert()
       .success()
       .stdout(predicate::str::contains("2:hello world"));
}

#[test]
fn test_case_insensitive() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello World").unwrap();
    writeln!(file, "HELLO WORLD").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("-i")
       .arg("hello")
       .arg(file_path)
       .assert()
       .success()
       .stdout(predicate::str::contains("Hello World"))
       .stdout(predicate::str::contains("HELLO WORLD"));
}

#[test]
fn test_invert_match() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "hello world").unwrap();
    writeln!(file, "goodbye").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("-v")
       .arg("world")
       .arg(file_path)
       .assert()
       .success()
       .stdout(predicate::str::diff("hello world").not().normalize())
       .stdout(predicate::str::contains("goodbye"));
}

#[test]
fn test_files_with_matches() {
    let temp_dir = TempDir::new().unwrap();
    let file1_path = temp_dir.path().join("file1.txt");
    let file2_path = temp_dir.path().join("file2.txt");

    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "hello world").unwrap();

    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "no matches").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("-l")
       .arg("-r")  // Need recursive flag to search directory contents
       .arg("world")
       .arg(temp_dir.path())
       .assert()
       .success()
       .stdout(predicate::str::contains("file1.txt"))
       .stdout(predicate::str::diff("file2.txt").not().normalize());
}

#[test]
fn test_files_without_match() {
    let temp_dir = TempDir::new().unwrap();
    let file1_path = temp_dir.path().join("file1.txt");
    let file2_path = temp_dir.path().join("file2.txt");

    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "hello world").unwrap();

    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "no matches").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("-L")
       .arg("-r")  // Need recursive flag to search directory contents
       .arg("world")
       .arg(temp_dir.path())
       .assert()
       .success()
       .stdout(predicate::str::diff("file1.txt").not().normalize())
       .stdout(predicate::str::contains("file2.txt"));
}

#[test]
fn test_recursive_search() {
    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir).unwrap();

    let file1_path = temp_dir.path().join("file1.txt");
    let file2_path = sub_dir.join("file2.txt");

    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "hello world").unwrap();

    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "hello universe").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("-r")
       .arg("-H")
       .arg("hello")
       .arg(temp_dir.path())
       .assert()
       .success()
       .stdout(predicate::str::contains("file1.txt:hello world"))
       .stdout(predicate::str::contains("subdir/file2.txt:hello universe"));
}

#[test]
fn test_regex_search() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "hello world").unwrap();
    writeln!(file, "hi world").unwrap();
    writeln!(file, "goodbye").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("h[ei]")
       .arg(file_path)
       .assert()
       .success()
       .stdout(predicate::str::contains("hello world"))
       .stdout(predicate::str::contains("hi world"))
       .stdout(predicate::str::diff("goodbye").not().normalize());
}

#[test]
fn test_no_matches() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "hello world").unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("nonexistent")
       .arg(file_path)
       .assert()
       .success()
       .stdout(predicate::str::is_empty());
}

#[test]
fn test_file_not_found() {
    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("pattern")
       .arg("nonexistent_file.txt")
       .assert()
       .failure()
       .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn test_invalid_regex() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let mut cmd = Command::cargo_bin("perg").unwrap();
    cmd.arg("[invalid")
       .arg(file_path)
       .assert()
       .failure()
       .stderr(predicate::str::contains("Regex error"));
}
