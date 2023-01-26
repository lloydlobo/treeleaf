# treeleaf

[![ci rust](https://github.com/lloydlobo/treeleaf/actions/workflows/rust.yml/badge.svg)](https://github.com/lloydlobo/treeleaf/actions/workflows/rust.yml)
[![Version info](https://img.shields.io/crates/v/trealeaf.svg)](https://crates.io/crates/trealeaf)

> Tree diagrams for the teriminal.
> Visualize mermaid-like data on the command-line.

<!-- [API documentation](https://docs.rs/treeleaf) -->

**Demo**: Creating the classic hello world tree:

![treeleaf](https://i.imgur.com/z19OYxE.gif)

NOTE: Only binary tree-like diagrams available for now.

## Features

* Constant visual feedback about the creation progress and current estimates.
* Support for arbitrary shell commands.
* Interactive tree creation acroos optional runs.
* Cache-clearing commands can be set up before each drafting run.
* Export results to various formats: CSV, JSON, Markdown, AsciiDoc.
* Cross-platform

## Usage

### `tree` emulation for your docs

```bash
$ mkdir -p foo/bar/baz
$ touch foo/foo.md foo/bar/foobar.md foo/bar/baz/foobarbaz.md
$ ./treeleaf foo

foo
├── foo.md
└── bar
    ├── baz
    │   └── foobarbaz.md
    └── foobar.md
```

### Docker

#### Docker Pull Command

```shell
docker pull lloydlobo/treeleaf
```

### Example

An example program is provided under the "examples" directory to mimic the `tree(1)`
linux program

```bash
$ cargo run --example treeleaf target
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/treeleaf target`

foo
├── hello
│   world
└── goodbye
    world
    ├── foo
    |   └── bar
    |       └── baz
    ├── foo
    |   ├── bar
    |   └── baz
    ├── bar
    ├── hello
    │   world
    ├── goodbye
    └── world
```

### Exporting results

#### Markdown

You can use the `--export-markdown <file>` option to create tables like the following:

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `find . -iregex '.*[0-9]\.jpg$'` | 2.275 ± 0.046 | 2.243 | 2.397 |9.9 ± 0.22 |
| `find . -iname '*[0-9].jpg'` | 1.427 ± 0.026 | 1.405 | 1.468 | 6.14 ± 0.13 |
| `fd -HI '.*[0-9]\.jpg$'` | 0.232 ± 0.002 | 0.230 | 0.236 | 1.00 |

#### JSON

The JSON output is useful if you want to analyze
the benchmark results in more detail. The [`scripts/`](https://github.com/lloydlobo/treeleaf/tree/master/scripts)
folder includes a lot of helpful Python programs to further analyze
benchmark results and create helpful visualizations, like a histogram
of runtimes or a whisker plot to compare multiple benchmarks:

| ![histogram](doc/histogram.png) | ![whisker](doc/whisker.png) |
|---:|---:|

### Detailed benchmark flowchart

The following chart explains the execution order of various timing runs when
using options like `--warmup`, `--prepare <cmd>`, `--setup <cmd>` or `--cleanup <cmd>`:

![exececution-order](doc/execution-order.png)

## Installation

### With cargo (Linux, macOS, Windows)

Treeleaf can be installed from source via [cargo](https://doc.rust-lang.org/cargo/):

```shell
cargo install --locked treeleaf
```

Make sure that you use Rust 1.60 or higher.

### From binaries (Linux, macOS, Windows)

Download the corresponding archive from the [Release page](https://github.com/lloydlobo/treeleaf/releases).

## Related Crates

* [`termtree`](https://crates.io/crates/termtree): treeleaf was forked from this.
* [`treeline`](https://crates.io/crates/treeline)
* [`tree_decorator`](https://crates.io/crates/tree_decorator)
* [`xtree`](https://crates.io/crates/xtree)
* [`ptree`](https://crates.io/crates/ptree)

## License

Treeleaf is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
