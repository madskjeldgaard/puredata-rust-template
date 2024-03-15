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

Please note: All build commands use the debug profile by default. For DSP, this is EXTREMELY slow. Therefore, make sure to use the release profile correctly when you need to have an optimized version of your external.

For debug builds: 

```bash
cargo make build
```

```bash
cargo make --profile release build
```

### Running the external in PureData

At the root of the project is a .pd file you can open to test the external.

Run it using cargo make:

```bash
cargo make run
```

If you want to run the release version:

```bash
cargo make --profile release run
```

### Install the external

If you have [plugdata](https://plugdata.org/) installed (on MacOS), this will install the external to the plugdata externals folder, otherwise the Pd-externals folder in your home directory. On Linux it will simply copy it to `~/pd-externals`

```bash
cargo make --profile release install
```

### Making a Deken package

The make file also includes a task to create a Deken package to be used in the [Pure Data externals database](https://deken.puredata.info/).

```bash
cargo make --profile release package
```

### Upload to Deken

```bash
cargo make --profile release upload
```
