use assert_cmd::Command;
use predicates::str::starts_with;

#[test]
fn test_kbs_binary() {
    // make sure the kbs binary itself works.
    Command::cargo_bin("kbs")
        .unwrap()
        .assert()
        .code(2)
        .stderr(starts_with("Usage: kbs [OPTIONS] <COMMAND>"));
}

#[test]
fn test_kbs_version() {
    // make sure `kbs version` prints a version string.
    Command::cargo_bin("kbs")
        .unwrap()
        .arg("version")
        .assert()
        .stdout(starts_with("v"));
}
