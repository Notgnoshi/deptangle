# deptangle

![lint workflow](https://github.com/Notgnoshi/deptangle/actions/workflows/lint.yml/badge.svg?event=push)
![code coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/Notgnoshi/0e15edf83d41c5b3cace2ff71f4d2f53/raw/deptangle-coverage.json)

Tools to interrogate and detangle dependency graphs

## Table of contents

* [depconv](#depconv) -- convert dependency graphs between formats

# Philosophy

Rather than build an infinitely flexible, does-everything-and-more graph toolkit, these are targeted
tools to solve specific dependency graph problems I frequently encounter.

All tools operate on stdin/stdout in addition to files, and are designed to be chained together with
pipes. Any ancillary output is emitted on stderr.

# How to use

You can install the tools with

```sh
cargo install --path crates/deptangle-cli --root ~/.local/
```

You can also just experiment with the tools by

```sh
cargo run --release --bin depconv -- ...
```

You likely want a release build for large graphs.

# Tools

## depconv

Convert dependency graphs between formats. Input and output formats are auto-detected from file
extensions or by probing the file content when `--input-format`/`--output-format` are not specified.
Defaults to DOT output when no output format can be inferred.

```sh
$ echo -e "a Node A\nb Node B\n#\na b depends on" | depconv --output-format dot
digraph {
    a [label="Node A"];
    b [label="Node B"];
    a -> b [label="depends on"];
}

$ cargo tree --depth 1 | depconv --output-format tgf
deptangle v0.1.0
clap v4.6.6
...
#
deptangle v0.1.0 clap v4.6.6
...
```

### Supported formats

| Format         | `--input-format` | `--output-format` | Description                                                                     |
| -------------- | :--------------: | :---------------: | ------------------------------------------------------------------------------- |
| DOT (GraphViz) |       yes        |        yes        | `digraph` / `graph` syntax. Parses cmake, ninja, bitbake, and ad-hoc DOT output |
| Mermaid        |       yes        |        yes        | `flowchart` / `graph` graph types                                               |
| TGF            |       yes        |        yes        | Trivial Graph Format                                                            |
| Depfile        |       yes        |        yes        | Makefile `.d` depfile                                                           |
| Tree           |       yes        |        yes        | `tree` CLI output, both ascii and unicode output formats                        |
| Pathlist       |       yes        |        yes        | One path per line; hierarchy inferred from `/` separators                       |
| Cargo tree     |       yes        |        no         | `cargo tree` output                                                             |
| Cargo metadata |       yes        |        no         | `cargo metadata --format-version=1` JSON                                        |

### What's preserved across formats

Not every format can represent the same information. The table below shows what each format
preserves when parsing (P) and emitting (E):

| Format         | Labels | Node type |  Attrs  | Edge labels | Subgraphs |
| -------------- | :----: | :-------: | :-----: | :---------: | :-------: |
| DOT            |  P+E   |    P+E    |   P+E   |     P+E     |    P+E    |
| Mermaid        |  P+E   |  partial  | partial |     P+E     |    P+E    |
| TGF            |  P+E   |    --     |   --    |     P+E     |    --     |
| Depfile        |   --   |    --     |   --    |     --      |    --     |
| Tree           |  P+E   |    --     |   --    |     --      |    --     |
| Pathlist       |  P+E   |    --     |   --    |     --      |    --     |
| Cargo tree     |   P    |     P     |    P    |     --      |    --     |
| Cargo metadata |   P    |     P     |    P    |     --      |    --     |

Converting from a rich format (DOT, cargo metadata) to a simpler one (TGF, depfile) silently drops
unsupported attributes. Converting in the other direction preserves graph topology but cannot
recover lost metadata.
