# Generate a Rust-based external for PureData

This is a cargo generate template to create a PureData external written entirely in Rust.

It uses [puredata-rust](https://github.com/x37v/puredata-rust).

## Usage:

You need to have [cargo-generate](https://github.com/cargo-generate/cargo-generate) and [cargo-make](https://github.com/sagiegurari/cargo-make) installed to run these commands.

Install them using cargo before procedeeding:
    
```bash
cargo install cargo-generate
cargo install cargo-make
```

### Generating a new project

```bash
cargo generate madskjeldgaard/puredata-rust-template
cd <project-name>
```

### Building

```bash
cargo make
```

### Running the plugin in PureData

At the root of the project is a .pd file you can open to test the plugin.

Run it using cargo make:

```bash
cargo make run
```

### Making a Deken package

The make file also includes a task to create a Deken package to be used in the [PureData externals database](https://deken.puredata.info/).

```bash
cargo make package
```

### Upload to Deken

```bash
cargo make upload
```
