# kubeconfig-bikeshed

There are few topics in software engineering that have been [bikeshedded](https://en.wiktionary.org/wiki/bikeshed) as much as personal workflow tools. The Cloud Native space is not exempted from that. The way of managing your kubeconfigs to use with `kubectl` (and other command line tools) is a highly personal choice.

kubeconfig-bikeshed - `kbs` - is an opinionated tool to manage your kubeconfigs for accessing many Kubernetes APIs. Instead of managing a single kubeconfig file with many contexts, `kbs` maintains many kubeconfig files that can be switched between.

`kbs` is a personal project with two objectives:

1. Write tooling for my own workflows.
1. Learn Rust.

The core idea of this tool is that the context name should be meaningful. Not all Kubernetes deployment tools generate "good" context names. And _meaningful_ for me often meant renaming the context to the DNS name hosting the Kubernetes API. Therefore, `kbs` automatically discovers the server name from a kubeconfig that is being imported, and uses that to identify any conflicts with existing kubeconfigs and to set a meaningful context name.

There is also a blog post available that gets into details of the initial version and the motivation to start this project: [Bikeshedding Kubeconfig Management](https://marvin.beckers.dev/blog/bikeshedding-kubeconfig-management/).

## Installation

### Homebrew

`kbs` is available from my [tap](https://github.com/embik/homebrew-tap):

```sh
$ brew tap embik/tap
$ brew install kubeconfig-bikeshed
```

### Others (via `cargo`)

kubeconfig-bikeshed is also directly available from [crates.io](https://crates.io) and can be installed via `cargo` (if a working Rust toolchain is installed):

```sh
$ cargo install kubeconfig-bikeshed
```

This will install the `kbs` binary. Make sure `$HOME/.cargo/bin` is in your `PATH`.

## Setup

After installing `kbs`, a few things can be set up for smoother usage of it. To change between contexts and namespaces, it is recommended to also install [kubectx](https://github.com/ahmetb/kubectx).

### Autocompletion

`kbs` can generate shell autocompletion for many available shells via the `kbs shell completion` command. Specific instructions might differ by shell. For example, to install the `zsh` autocompletion, add the following snippet to your `.zshrc`:

```sh
source <(kbs shell completion zsh)
```

### `KUBECONFIG` Magic

One of the most important features of a kubeconfig manager might be the ability to set the environment variable `KUBECONFIG` to point to a kubeconfig file of your choice. Unfortunately, the `kbs` binary on its own cannot provide that feature as it cannot set environment variables for the active shell.

To remedy that, `kbs` optionally provides shell "magic" that replaces the `kbs` binary in your shell with a function that can export `KUBECONFIG`. This requires `fzf` to be installed. Shell magic is supported for a subset of shells, the currently supported shells are:

- `zsh`
- `bash`

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
  use     Use a kubeconfig by name and print shell snippet to source
  shell   Print various shell related scripts
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Enable verbose (debug) logging
  -h, --help     Print help
```

## Alternatives

- [konf-go](https://github.com/SimonTheLeg/konf-go) by my colleague Simon, if you are looking for a more mature solution written in Go.
- [kubecm](https://github.com/sunny0826/kubecm) is probably the "top dog" of kubeconfig managers.
