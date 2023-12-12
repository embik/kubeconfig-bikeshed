use assert_cmd::Command;
use predicates::str::{contains, is_match};
use tempfile::tempdir;

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
fn test_kbs_import_mixed_kubeconfig() {
    let temp_dir = tempdir().unwrap();
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/files");

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("mixed.kubeconfig").to_str().unwrap(),
        ])
        .assert()
        .failure();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("mixed.kubeconfig").to_str().unwrap(),
            "--name",
            "kubernetes.embik.me",
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
