# Github Stats

## Example

```rust
cargo run -- --stars 100 --template template.md --filter projectname
```

## Usage

```
gh-stats 0.1.0
Github Stats

USAGE:
    gh-stats [OPTIONS] --template <template>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <filter>...     Filter repositories (regex support)
    -s, --stars <stars>          The minimum number of stars required to display a project [default: 10]
    -t, --template <template>    Template file
```
