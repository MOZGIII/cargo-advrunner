# cargo-advrunner

An advanced configurable runner for cargo.

## Usage

1. Install `cargo-advrunner`

   ```shell
   cargo install cargo-advrunner
   ```

2. Add the `.cargo/config.toml` entry.

   ```toml
   runner = "cargo-advrunner"
   ```

3. Add the `advrunner.toml` config file.

   ```toml
   [test]
   command = "<command to use for cargo test>"

   [run]
   command = "<command to use for cargo run>"
   ```

## Examples

### WASM

Configure `advrunner.toml` to run using `wasm-server-runner`, test using `wasm-bindgen-test-runner`:

```toml
[test]
command = "wasm-bindgen-test-runner"

[run]
command = "wasm-server-runner"
```

Set the runner to only be used for WASM at `.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
runner = "cargo-advrunner"
```
