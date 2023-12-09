use assert_cmd::Command;
use predicates::str::{contains, is_empty, is_match, starts_with};
use tempfile::tempdir;

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

#[test]
fn test_kbs_import() {
    let temp_dir = tempdir().unwrap();
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/files");

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
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\n$").unwrap());
}

#[test]
fn test_kbs_import_duplicate_error() {
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

    // assert that import worked.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\n$").unwrap());

    // second import should fail.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("test.kubeconfig").to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(contains("kubeconfig kubernetes.embik.me already exists"));
}

#[test]
fn test_kbs_import_name_override() {
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
            "-n",
            "kubernetes.beckers.dev",
        ])
        .assert()
        .success();

    // assert that import worked and shows up with the adequate name override.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.beckers.dev\n$").unwrap());
}

#[test]
fn test_kbs_import_with_labels() {
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
            "location=nonexistent,owner=embik",
        ])
        .assert()
        .success();

    // assert that import worked and labels show up in table view.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-o",
            "table",
        ])
        .assert()
        .success()
        .stdout(
            is_match(
                "^NAME( +)\tLABELS( +)\nkubernetes.embik.me( +)\tlocation=nonexistent,owner=embik\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_kbs_import_with_short_flag() {
    let temp_dir = tempdir().unwrap();
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/files");

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("test.kubeconfig").to_str().unwrap(),
            "-s",
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes\n$").unwrap());
}

#[test]
fn test_kbs_import_with_short_flag_localhost() {
    let temp_dir = tempdir().unwrap();
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/files");

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("localhost.kubeconfig").to_str().unwrap(),
            "-s",
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^localhost\n$").unwrap());
}

#[test]
fn test_kbs_list_label_selector() {
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
            "location=nonexistent,owner=embik",
        ])
        .assert()
        .success();

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
            "-n",
            "kubernetes.beckers.dev",
        ])
        .assert()
        .success();

    // assert that only one of the two imported kubeconfigs work,
    // namely the one matching the label.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "location=imagination",
        ])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.beckers.dev\n$").unwrap());
}

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
