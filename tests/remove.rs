use assert_cmd::Command;
use predicates::str::{is_empty, is_match};
use tempfile::tempdir;

#[test]
fn test_kbs_remove() {
    let temp_dir = tempdir().unwrap();
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/files");

    // initial import should succeed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("test.kubeconfig").to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("localhost.kubeconfig").to_str().unwrap(),
        ])
        .assert()
        .success();

    // both should be listed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\nlocalhost\n$").unwrap());

    // removing one kubeconfig should succeed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "remove",
            "kubernetes.embik.me",
        ])
        .assert()
        .success();

    // only the other kubeconfig should be listed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^localhost\n$").unwrap());
}

#[test]
fn test_kbs_remove_by_selector() {
    let temp_dir = tempdir().unwrap();
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/files");

    // initial import should succeed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("test.kubeconfig").to_str().unwrap(),
            "-l",
            "location=imagination,owner=embik",
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("localhost.kubeconfig").to_str().unwrap(),
            "-l",
            "location=nonexistent,owner=embik",
        ])
        .assert()
        .success();

    // both should be listed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\nlocalhost\n$").unwrap());

    // removing kubeconfig by location=imagination selector should succeed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "remove",
            "-l",
            "location=imagination",
        ])
        .assert()
        .success();

    // only the other kubeconfig should be listed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^localhost\n$").unwrap());

    // removing kubeconfig by owner=embik selector should succeed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "remove",
            "-l",
            "owner=embik",
        ])
        .assert()
        .success();

    // no kubeconfig should exist anymore.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_empty());
}
