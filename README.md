# Generate a Rust-based external for PureData

This is a cargo generate template to create a PureData external written entirely in Rust.

It uses [puredata-rust](https://github.com/x37v/puredata-rust).

## Usage:

```bash
cargo generate madskjeldgaard/puredata-rust-template
cd <project-name>
cargo make
```

At the root of the project is a .pd file you can open to test the plugin.

## Dependencies

- Cargo
- Cargo-Make
- Cargo-Generate

Install these dependencies with:

```bash
cargo install cargo-make
cargo install cargo-generate
```
