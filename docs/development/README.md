# Development

## New Releases

Requirements to release:

- Be logged into crates.io.
- Have [cargo release](https://github.com/crate-ci/cargo-release) installed.

To cut a new release, do the following steps:

1. Bump `version` in [Cargo.toml](./Cargo.toml) to the next version.
1. Run `cargo check` so Cargo.lock gets updated.
1. Commit changes and commit as `cargo: bump to version <version>`.
1. Run `cargo release --sign-tag --sign-commit` to check if release is functional.
1. Run `cargo release --sign-tag --sign-commit --execute` to push release to crates.io and GitHub.
1. Go to [Tags](https://github.com/embik/kubeconfig-bikeshed/tags) and create release from it.
