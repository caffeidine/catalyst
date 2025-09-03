# Installation

There are several ways to install Catalyst on your system.

## Using Cargo

The recommended way to install Catalyst is through Cargo, Rust's package manager. If you already have Rust installed, you can install Catalyst with:

```bash
cargo install catalyst
```

This will download, compile, and install the latest version of Catalyst from crates.io.

## Building from Source

If you prefer to build Catalyst from source, or if you want to use the latest development version, you can clone the repository and build it yourself:

```bash
# Clone the repository
git clone https://github.com/caffeidine/catalyst.git
cd catalyst

# Build and install
cargo install --path .
```

## Verifying Installation

To verify that Catalyst has been installed correctly, run:

```bash
catalyst --version
```

You should see output showing the version number of Catalyst.

## Next Steps

Now that you have Catalyst installed, start with the [Quickstart](./tutorials/quickstart.md) or jump into [Your First Test](./getting-started/first_test.md).
