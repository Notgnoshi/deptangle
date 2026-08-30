# Developer Guide

This project is not really intended to be used as a library, so the MSRV is the latest stable
toolchain, and we aspire to keep all dependencies up-to-date.

It's a "usual" Rust project, so you can use `cargo` as normal:

```sh
cargo build
cargo run --bin depconv -- ...
# nextest is preferred over 'cargo test' for better speed; both will work
cargo nextest run
cargo fmt -- --config group_imports=StdExternalCrate,imports_granularity=Module
cargo clippy --all-targets --all-features

# For updating or auditing dependencies
cargo install cargo-edit
cargo upgrade --dry-run --incompatible
```

Because this project mixes Rust tools with some scripts, to facilitate installation, an `install`
script is provided that installs both:

```sh
./install --prefix ~/.local
./install --uninstall --prefix ~/.local
```

This project values tests, but does not prioritize complete test coverage. Tests are code that needs
to be reviewed and maintained just as well as any other, so this project prioritizes test quality
over quantity. When adding a new test, consider if that test is actually valuable and justifies its
maintenance cost. The same goes for comments and documentation: don't just say what the code does,
but why it does it; the audience of comments and documentation is not the developer at the time of
writing, but the maintainer in several years.

To make a release, add an entry to the `CHANGELOG.md` file and then merge a PR that bumps the
version in the workspace `Cargo.toml`. The CI pipeline will do the rest. Not every change needs
mentioned in the changelog. Keep the focus on changes that are relevant to the end user.

This is a virtual Cargo workspace:

* `./crates/deptangle-graph` - the graph data model
* `./crates/deptangle-io` - graph serialization and deserialization, as well as auto format
  detection
* `./crates/deptangle-cli` - provides the CLI binaries and integration tests. The `main.rs` files
  for each binary is intended to be as thin as possible, delegating to the other library crates.
* `./crates/deptangle-minpath` - a library for shortening file paths much the same way an IDE might
  use in its editor tabs.
* `./crates/deptangle-ops` - a library for performing analysis and transformations on graphs
* `./crates/deptangle-test` - a test fixture library for use in integration tests
