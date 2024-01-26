# kubeconfig-bikeshed

kubeconfig-bikeshed - `kbs` - is an opinionated tool to manage your kubeconfigs for accessing many Kubernetes APIs. Instead of managing a single kubeconfig file with many contexts, `kbs` maintains many kubeconfig files that can easily be switched between.

See the demo below for a typical `kbs` workflow (this assumes that all optional shell integration is set up):

![kbs demo](./docs/kbs.gif)

## Why `kbs`?

`kbs` can help restoring order to a scattered and inconsistent collection of kubeconfigs. It does so by enforcing standardized naming, by taking ownership of its kubeconfigs and by allowing to apply bulk operations like purging outdated kubeconfigs.

`kbs` maintains a kubeconfig "data store", which by default is located in `~/.config/kbs` (`kbs` is aware of `XDG_CONFIG_HOME` and looks for the store directory in `$XDG_CONFIG_HOME/kbs` if it is set). For kubeconfigs imported into its store, `kbs` allows to set and update labels (just like on Kubernetes objects) that can be used to run bulk operations.

When it comes to naming standards, the core idea of `kbs` is that not all Kubernetes deployment tools generate "good" context names. Therefore, `kbs` automatically discovers the server name from a kubeconfig that is being imported, and uses that to identify any conflicts with existing kubeconfigs and to set a meaningful context name. This can be overridden, but `kbs` expects **all names to be valid DNS-style names**.

There is also a blog post that gets into details of the initial version and the motivation to start this project: [Bikeshedding Kubeconfig Management](https://marvin.beckers.dev/blog/bikeshedding-kubeconfig-management/).

## Installation

kubeconfig-bikeshed can be installed as a standalone binary called `kbs`. Optionally, a `kbs` shell function that overrides the binary call can be loaded into some shells to allow selecting an active kubeconfig (see [Setup](#Setup)).

### Options

`kbs` can currently be installed via [Homebrew](https://brew.sh), `cargo` or downloaded from the [releases page](https://github.com/embik/kubeconfig-bikeshed/releases).

#### Homebrew

`kbs` is available from my [tap](https://github.com/embik/homebrew-tap):

```sh
$ brew tap embik/tap
$ brew install kubeconfig-bikeshed
```

#### Cargo

kubeconfig-bikeshed is also directly available from [crates.io](https://crates.io) and can be installed via `cargo` (if a working Rust toolchain is installed):

```sh
$ cargo install kubeconfig-bikeshed
```

This will install the `kbs` binary. Make sure `$HOME/.cargo/bin` is in your `PATH`.

### Setup

After installing `kbs`, a few things can be set up for smoother usage of it. To change between contexts and namespaces, it is recommended to also install [fubectl](https://github.com/kubermatic/fubectl).

#### Autocompletion

`kbs` can generate shell autocompletion for many available shells via the `kbs shell completion` command. Specific instructions might differ by shell. For example, to install the `zsh` autocompletion, add the following snippet to your `.zshrc`:

```sh
source <(kbs shell completion zsh)
```

#### "Magic" Shell Functions

One of the most important features of a kubeconfig manager might be the ability to set the environment variable `KUBECONFIG` to point to a kubeconfig file of your choice. Unfortunately, the `kbs` binary on its own cannot provide that feature as it cannot set environment variables for the active shell.

To remedy that, `kbs` optionally provides shell "magic" that replaces the `kbs` binary in your shell with a function that can export `KUBECONFIG`. This requires `fzf` to be installed. Shell magic is supported for a subset of shells, the currently supported shells are:

- `zsh`
- `bash`

Specific instructions might differ by shell as well, e.g. to install the `zsh` magic you can add the following snippet to your `.zshrc`:

```sh
source <(kbs shell magic zsh)
```

#### Restore Last Active Kubeconfig

To start new shells with the last selected ("active") kubeconfig, add the following snippet (or similar, depending on your shell) to your login shell configuration (e.g. `.zshrc`):

```sh
eval $(kbs use -)
```

## Usage

To select a kubeconfig from the `kbs` data store, simply run `kbs` (if shell integration is all set up). This will offer a selection via `fzf` and export the `KUBECONFIG` environment variable.

Full set of commands for `kbs` below.

```sh
Usage: kbs [OPTIONS] <COMMAND>

Commands:
  import   Import a kubeconfig into data store [aliases: i]
  list     List available kubeconfigs [aliases: ls]
  use      Use a kubeconfig by name and print shell snippet to source [aliases: u]
  shell    Print various shell related scripts [aliases: sh]
  remove   Remove kubeconfig from data store [aliases: rm, delete]
  version  Print version [aliases: v]
  label    Manage labels on kubeconfigs in the data store [aliases: l]
  prune    Remove kubeconfigs for Kubernetes API servers that are no longer accessible [aliases: p]
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose                  Enable verbose (debug) logging
  -c, --config-dir <config-dir>  Directory to use for configuration and data store. Defaults to ~/.config/kbs or $XDG_CONFIG_DIR/kbs
  -h, --help                     Print help
```

### Importing Kubeconfigs

`kbs import` allows to _import_ a kubeconfig already existing on the local filesystem (e.g. because it was downloaded via a third-party tool or a web interface) into the kbs "data store".

The command takes a couple of flags to alter behaviour of the import process. When a kubeconfig has multiple severs configured, passing a `--name` might be necessary as `kbs` cannot determine a name automatically.

### Updating Kubeconfig Labels

`kbs label` allows setting new labels or updating existing labels on a kubeconfig identified by name or by label selector. Labels can be passed as `key=value` pairs, separated by comma.

Existing label values can only be updated if `--overwrite` is set, mimicking `kubectl` behaviour.

### Removing Kubeconfigs

`kbs remove` allows deleting kubeconfigs by name (or label selector) from the `kbs` data store.

### Pruning Kubeconfigs

`kbs prune` is a helpful command in case removing kubeconfigs by hand or label selector proves cumbersome. 

`kbs` will iterate over all kubeconfigs matched by the label selector passed to `kbs prune` (if none is passed, all kubeconfigs will be checked) and attempt to fetch the Kubernetes API server version from the remote server as a way to "ping" it. If that connection fails for whatever reason, `kbs prune` will consider the kubeconfig in question stale.

By default, this command runs in "dry mode", which means it will not delete any kubeconfigs (as this is a destructive action potentially elevated by temporary networking problems). To actually prune kubeconfigs, pass `--dry-run=false` to the command.

## Alternatives

- [konf-go](https://github.com/SimonTheLeg/konf-go) by my colleague Simon, if you are looking for a more mature solution written in Go.
- [kubecm](https://github.com/sunny0826/kubecm) is probably the "top dog" of kubeconfig managers.
