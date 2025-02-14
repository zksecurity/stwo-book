# Quickstart

The fastest way to get started with Stwo is to use the stwo-cairo repository:

```bash
git clone https://github.com/starkware-libs/stwo-cairo
cd stwo-cairo
```

> ⚠️ **Note**: Stwo, the proof system used by Stwo-Cairo, is currently under active development. The API and features may change frequently, and the system is not yet cryptographically sound. Use it for testing and experimentation only.

## Getting Started with Proofs

After running a Cairo program, you'll get four important files:

- `air_public_inputs.json`: Contains publicly verifiable information
- `air_private_inputs.json`: Contains private computation details
- `trace.bin`: Records the program's execution steps
- `memory.bin`: Stores the program's memory state

To generate a proof, use the `adapted_stwo` command:

```bash
cargo run --bin adapted_stwo --release \
--pub_json <path_to_air_public_input> \
--priv_json <path_to_air_private_input> \
--proof_path <path for proof output>
```

For optimal performance, add these flags:

```bash
RUSTFLAGS="-C target-cpu=native -C opt-level=3" --features="std"
```

## Creating Your First Cairo Executable

### Setup Requirements

1. Install Scarb (Cairo's package manager) using [asdf](https://asdf-vm.com/). Use version 2.10.0 or later:

```bash
asdf global scarb 2.10.0
```

or

```bash
asdf global scarb latest:nightly
```

### Creating a Project

1. Start a new project:

```bash
scarb new <project_name>
```

2. Configure your `Scarb.toml` file:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024_07"

[[target.executable]]

[cairo]
enable-gas = false

[dependencies]
cairo_execute = "2.10.0"
```

3. Create a simple program in `src/lib.cairo`:

```rust
#[executable]
fn main(num: u8) -> u8 {
    num
}
```

4. Run your program:

```bash
scarb execute -p my_project --print-program-output --arguments 5
```

## Execution Options

You can run your program in two modes:

- `standalone`: Direct execution and proving (default)
- `bootloader`: Execution resulting in a PIE, or Position Independent Executable. This cannot be proved by itself, must be provided as input to a bootloader program.

Use the `--target` flag to specify your mode:

```bash
scarb execute --target standalone  # or bootloader
```

### Input Formats

Inputs are provided as comma-separated values. Here are some examples:

- For `fn main(num: u8)`: Use `--arguments 1`
- For `fn main(num1: u8, num2: u16)`: Use `--arguments 1,2`
- For `fn main(num1: u8, tuple: (u16, u16))`: Use `--arguments 1,2,3`

More info about serialization of cairo types can be found [here](https://docs.starknet.io/architecture-and-concepts/smart-contracts/serialization-of-cairo-types/)

When using the `--arguments-file` flag, the arguments should be represented as hex strings. For example, 1,2,3 in the above example becomes ["0x1", "0x2", "0x3"].

### Current Limitations

1. Gas tracking must be disabled <- only used for Starknet smart contracts
2. System calls (syscalls) are not supported <- this means no external calls to other contracts are allowed (e.g. corelib functions)
3. Execution steps are currently padded for optimization <- only applies for `standalone` target,

## Using Scarb for Proofs

From Scarb 2.10.0, you can generate proofs directly:

1. Run your program:

```bash
scarb execute
```

2. Generate a proof:

```bash
scarb prove --execution_id 1
```

3. Verify the proof:

```bash
scarb verify <path_to_proof_json>
```

Your proofs will be stored in `./target/execute/<package_name>/execution<N>/proof/`.

You can also verify a proof using scarb:

```bash
scarb verify <path_to_proof_json>
```
