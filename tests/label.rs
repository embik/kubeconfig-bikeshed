use assert_cmd::Command;
use predicates::str::is_match;
use tempfile::tempdir;

#[test]
fn test_kbs_label_add() {
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

    // add a label to the imported kubeconfig.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "label",
            "--name",
            "kubernetes.embik.me",
            "new-label=new-value",
        ])
        .assert()
        .success();

    // assert that imported kubeconfig shows up under new label.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "new-label=new-value",
        ])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\n$").unwrap());
}

#[test]
fn test_kbs_label_add_by_selector() {
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
            "to-be-labeled=true",
        ])
        .assert()
        .success();

    // add a label to the imported kubeconfig.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "label",
            "--selector",
            "to-be-labeled=true",
            "new-label=new-value",
        ])
        .assert()
        .success();

    // assert that imported kubeconfig shows up under new label.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "new-label=new-value",
        ])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\n$").unwrap());
}

#[test]
fn test_kbs_label_remove() {
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
            "new-label=new-value,old-label=old-value",
        ])
        .assert()
        .success();

    // assert that imported kubeconfig shows up under new label.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "new-label=new-value",
        ])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\n$").unwrap());

    // remove label from the imported kubeconfig.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "label",
            "--name",
            "kubernetes.embik.me",
            "new-label-",
        ])
        .assert()
        .success();

    // assert that imported kubeconfig no longer shows up under removed label.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "new-label=new-value",
        ])
        .assert()
        .success()
        .stdout(is_match("^$").unwrap());

    // assert that imported kubeconfig still shows up under old label.
    Command::cargo_bin("kbs")
        .unwrap()
        .args(&[
            "-c",
            temp_dir.path().to_str().unwrap(),
            "list",
            "-l",
            "old-label=old-value",
        ])
        .assert()
        .success()
        .stdout(is_match("^kubernetes.embik.me\n$").unwrap());
}
