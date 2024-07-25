# RISC Zero Rust Starter Template

Welcome to the RISC Zero Rust Starter Template! This template is intended to
give you a starting point for building a project using the RISC Zero zkVM.
Throughout the template (including in this README), you'll find comments
labelled `TODO` in places where you'll need to make changes. To better
understand the concepts behind this template, check out the [zkVM
Overview][zkvm-overview].

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following
command:

```bash
cargo run
```

This is an empty template, and so there is no expected output (until you modify
the code).

### Executing the project locally in development mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="[executor]=info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```