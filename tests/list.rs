use assert_cmd::Command;
use predicates::str::is_match;
use tempfile::tempdir;

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
fn test_kbs_list_label_selector_not_equal() {
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

    // initial import should succeed.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "import",
            base_dir.join("test.kubeconfig").to_str().unwrap(),
            "-l",
            "location=narnia,owner=notembik",
            "-n",
            "kubernetes.not.embik.me",
        ])
        .assert()
        .success();

    // assert that only one of the imported kubeconfigs is returned,
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

    // assert that the two kubeconfigs that are not labeled location=imagination
    // are returned.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "location!=imagination",
        ])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\nkubernetes.not.embik.me\n$").unwrap());
}
