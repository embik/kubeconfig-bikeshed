# kubeconfig-bikeshed

There are few topics in software engineering that have been [bikeshedded](https://en.wiktionary.org/wiki/bikeshed) as much as personal workflow tools. The Cloud Native space is not exempted from that. The way of managing your kubeconfigs to use with `kubectl` (and other command line tools) is a highly personal choice.

kubeconfig-bikeshed - `kbs` - is an opinionated tool to manage your kubeconfigs for accessing many Kubernetes APIs. Instead of managing a single kubeconfig file with many contexts, `kbs` maintains many kubeconfig files that can be switched between.

`kbs` is a personal project with two objectives:

1. Write tooling for my own workflows.
1. Learn Rust.

The core idea of this tool is that the context name should be meaningful after a kubeconfig has been imported. And _meaningful_ for me often meant to rename the context to the DNS name hosting the Kubernetes API. Therefore, `kbs` automatically discovers the server name from a kubeconfig that is being imported, and uses that to identify any conflicts with existing kubeconfigs and to set a meaningful context name.

## Installation

`kbs` is currently not available from any repositories. To install, `git`, `rustc` and `cargo` are required. If you have those available, run:

```sh
git clone git@github.com:embik/kubeconfig-bikeshed.git \
    && cd kubeconfig-bikeshed \
    && cargo install --path .
```

Make sure `$HOME/.cargo/bin` is in your `PATH`.

### Autocompletion

`kbs` can generate shell autocompletion for many available shells via the `kbs shell completion` command. Specific instructions might differ by shell. For example, to install the `zsh` autocompletion, add the following snippet to your `.zshrc`:

```sh
source <(kbs shell completion zsh)
```

### `KUBECONFIG` Magic

One of the most important features of a kubeconfig manager might be the ability to set the environment variable `KUBECONFIG` to point to a kubeconfig file of your choice. Unfortunately, the `kbs` binary on its own cannot provide that feature as it cannot set environment variables for the active shell.

To remedy that, `kbs` optionally provides shell "magic" that replaces the `kbs` binary in your shell with a function that can export `KUBECONFIG`. This requires `fzf` to be installed. Shell magic is supported for a subset of shells, the currently supported shells are:

- `zsh`

Specific instructions might differ by shell as well, e.g. to install the `zsh` magic you can add the following snippet to your `.zshrc`:

```sh
source <(kbs shell magic zsh)
```

## Usage

```
Usage: kbs [OPTIONS] [COMMAND]

Commands:
  import  Import a kubeconfig into store
  list    List available kubeconfigs
  path    Print full path to a specific kubeconfig in kbs store
  shell   Print various shell related scripts
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Enable verbose (debug) logging
  -h, --help     Print help
```

## Alternatives

- [konf-go](https://github.com/SimonTheLeg/konf-go) by my colleague Simon, if you are looking for a more mature solution written in Go.
