use assert_cmd::Command;
use predicates::str::{is_empty, is_match};
use tempfile::tempdir;

#[test]
fn test_kbs_prune() {
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

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "prune",
            "--dry-run=false",
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_empty());
}

#[test]
fn test_kbs_prune_dry_run() {
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

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "prune"])
        .assert()
        .success()
        .stderr(is_match("\'kubernetes.embik.me\' should be pruned").unwrap());

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\n$").unwrap());
}

#[test]
fn test_kbs_prune_label_selector() {
    let temp_dir = tempdir().unwrap();
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/files");

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

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("test.kubeconfig").to_str().unwrap(),
            "-n",
            "kubernetes.beckers.dev",
            "-l",
            "location=fantasy,owner=embik",
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.beckers.dev\nkubernetes.embik.me\n$").unwrap());

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "prune",
            "-l",
            "location=nonexistent",
            "--dry-run=false",
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.beckers.dev\n$").unwrap());
}
