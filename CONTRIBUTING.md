# Contributing

Thank you for considering to contribute to kubeconfig-bikeshed! Please see the following information regarding contributions.

## License

kubeconfig-bikeshed is licensed under [Apache-2.0](./LICENSE).

## Developer Certificate of Origin (DCO)

By contributing to kubeconfig-bikeshed you agree to the Developer Certificate of Origin (DCO), which has originally been created by the Linux Foundation for the Linux kernel developer community. It certifies that you have a legal right to ma.

To sign your work, just add a line like this at the end of your commit message (this can also be done with the `--signoff` / `-S` flag):

```
Signed-off-by: Joe Example <joe@example.com>
```

By doing so you certify that you make a contribution in accordance with the [DCO](./DCO) file in this repository.

## Conventional Commits

This repository uses [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) starting from May 25, 2024. [sidero/conform](https://github.com/siderolabs/conform) is used to validate and enforce it.

It is possible to set up a commit hook to run `conform` on commits. This is not necessary to contribute as conventional commits can be verified with any compatible tool.

```sh
cat <<EOF | tee .git/hooks/commit-msg
#!/bin/sh

conform enforce --commit-msg-file \$1
EOF
chmod +x .git/hooks/commit-msg
```
