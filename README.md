# Generate a Rust-based external for Pure Data

This is a cargo generate template to create a Pure Data external written entirely in Rust.

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

### Running the external in Pure Data

At the root of the project is a .pd file you can open to test the external.

Run it using cargo make:

```bash
cargo make run
```

### Install the external

If you have PlugData installed (on MacOS), this will install the external to the PlugData externals folder, otherwise the Pd-externals folder in your home directory. On Linux it will simply copy it to `~/pd-externals`

```bash
cargo make install
```

### Making a Deken package

The make file also includes a task to create a Deken package to be used in the [Pure Data externals database](https://deken.puredata.info/).

```bash
cargo make package
```

### Upload to Deken

```bash
cargo make upload
```
