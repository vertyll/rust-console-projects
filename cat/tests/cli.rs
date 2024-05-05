use std::fs;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn fail_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("cat")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));

    Ok(())
}

#[test]
fn test_nonblank_number_lines() -> TestResult {
    let expected = fs::read_to_string(
        "./tests/expected/number_non_blank_lines_expected"
    )?;

    let mut cmd = Command::cargo_bin("cat")?;

    cmd.args(&["-b", "tests/expected/number_non_blank_lines"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}