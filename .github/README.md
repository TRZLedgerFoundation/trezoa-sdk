# trezoa-sdk

[![Apache-2.0 license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Security](https://img.shields.io/badge/Security-Policy-brightgreen)](SECURITY.md)
[![Trezoa crate](https://img.shields.io/crates/v/trezoa-sdk.svg)](https://crates.io/crates/trezoa-sdk)
[![Trezoa documentation](https://docs.rs/trezoa-sdk/badge.svg)](https://docs.rs/trezoa-sdk)

Rust SDK for the Trezoa blockchain, used by on-chain programs and the Trezoa validator.

## Overview

Use the Trezoa SDK Crate to write client side applications in Rust. If writing on-chain programs, use the Trezoa Program Crate https://crates.io/crates/trezoa-program instead.

More information about Trezoa is available in the [Trezoa documentation](https://docs.trezoa.xyz).

The [Trezoa Program Library](https://github.com/TRZLedgerFoundation/trezoa-program-library) provides examples of how to use this crate.

## Features

- ⚡ **High Performance** - Optimized for Trezoa's parallel execution
- 🔒 **Type Safe** - Comprehensive type safety for blockchain operations
- 🛠️ **Developer Friendly** - Intuitive APIs and excellent documentation
- 🧪 **Well Tested** - Extensive test suite ensuring reliability
- 🌐 **Cross Platform** - Works on all major platforms

## Building

### 1. Install rustc, cargo and rustfmt.
```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

### 2. Download the source code.
```bash
git clone https://github.com/TRZLedgerFoundation/trezoa-sdk.git
cd trezoa-sdk
```

When building the master branch, please make sure you are using the version specified in the repo's `rust-toolchain.toml` by running:

```bash
rustup show
```

This command will download the toolchain if it is missing in the system.

### 3. Test.
```bash
cargo test
```

## Testing

### Basic testing
Run the test suite:
```bash
cargo test
```

Alternatively, there is a helper script:
```bash
./scripts/test-stable.sh
```

### Formatting
Format code for rustfmt check:
```bash
./cargo nightly fmt --all
```

The check can be run with a helper script:
```bash
./scripts/check-fmt.sh
```

### Clippy / Linting
To check the clippy lints:
```bash
./scripts/check-clippy.sh
```

### Benchmarking
Run the benchmarks:
```bash
./scripts/test-bench.sh
```

### Code coverage
To generate code coverage statistics:
```bash
./scripts/test-coverage.sh
```

```bash
$ open target/cov/lcov-local/index.html
```

Code coverage requires `llvm-tools-preview` for the configured nightly toolchain. To install the component, run the command output by the script if it fails to find the component:

```bash
rustup component add llvm-tools-preview --toolchain=<NIGHTLY_TOOLCHAIN>
```

## License

Licensed under [Apache License, Version 2.0](LICENSE).

---

Still have questions? Ask us on [Stack Exchange](https://trezoa.xyz/sse)
