# 🌟 Github Stats

**🌈 This project graduated from [mre-early-access](https://github.com/mre-early-access) on December 26, 2020 and is now public thanks to
[my generous sponsors](https://github.com/sponsors/mre).**

## Examples

#### Get stats for the current user:

```rust
cargo run -- --stars 100 --template template.md --filter projectname
```

#### Get stats for specific user:

```rust
cargo run -- mre --stars 100 --template template.md --filter projectname
```

## Installation

```
cargo install --git https://github.com/mre/gh-stats --branch=main
```

(We have to wait for hubcaps 0.6.3+ for publication on crates.io.)

## Usage

```
gh-stats 0.1.0
Github Stats

USAGE:
    gh-stats [FLAGS] [OPTIONS] --template <template> [--] [user]

FLAGS:
    -h, --help         Prints help information
    -V, --version      Prints version information
        --with-orgs    Include organization repositories

OPTIONS:
    -f, --filter <filter>...     Filter repositories (regex support)
    -o, --output <output>        Output file
    -s, --stars <stars>          The minimum number of stars required to display a project [default: 0]
    -t, --template <template>    Template file

ARGS:
    <user>    Github username for stats (default: own user)
```
