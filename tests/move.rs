use assert_cmd::Command;
use predicates::str::is_match;
use tempfile::tempdir;

#[test]
fn test_kbs_move() {
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
            "move",
            "kubernetes.embik.me",
            "k8s.embik.me",
        ])
        .assert()
        .success();

    Command::cargo_bin("kbs")
        .unwrap()
        .args(&["-c", temp_dir.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(is_match("^k8s.embik.me\n$").unwrap());
}

#[test]
fn test_kbs_move_nonexistent() {
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
            "move",
            "ubernetes.embik.me",
            "k8s.embik.me",
        ])
        .assert()
        .failure();
}

#[test]
fn test_kbs_move_labels() {
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
            "environment=test",
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
            "move",
            "kubernetes.embik.me",
            "k8s.embik.me",
        ])
        .assert()
        .success();

    // make sure the labels were moved as well
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "environment=test",
        ])
        .assert()
        .success()
        .stdout(is_match("^k8s.embik.me\n$").unwrap());
}
