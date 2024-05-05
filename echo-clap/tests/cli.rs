use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::stdout;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn pass_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-clap-2")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}

#[test]
fn check_new_line() -> TestResult {
    // let expected = fs::read_to_string("tests/expected/new_line")?;
    // let mut cmd = Command::cargo_bin("echo-clap-2")?;
    //
    // cmd.args(&["Mikołaj", "Dom", "Pałac"])
    //     .assert()
    //     .success()
    //     .stdout(expected);
    //
    // Ok(())

    run_test(&["Mikołaj", "Dom", "Pałac"], "tests/expected/new_line")
}

#[test]
fn check_no_new_line() -> TestResult {
    // let expected = fs::read_to_string("tests/expected/no_new_line")?;
    // let mut cmd = Command::cargo_bin("echo-clap-2")?;
    //
    // cmd.args(&["-n", "Mikołaj", "Dom", "Pałac"])
    //     .assert()
    //     .success()
    //     .stdout(expected);
    //
    // Ok(())
    run_test(&["-n", "Mikołaj", "Dom", "Pałac"], "tests/expected/no_new_line")
}

fn run_test(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echo-clap-2")?;
    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}