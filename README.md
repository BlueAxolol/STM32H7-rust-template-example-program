# STM32H753ZI program template

#### This template includes all files and configs necessary to print "Hello, Rust" every second over terminal (probe-rs).

## Dependencies:

### 1. `flip-link`:

```bash
cargo install flip-link
```

### 2. `probe-rs`:

Install **probe-rs** by following the instructions at <https://probe.rs/docs/getting-started/installation/> or installing the vs-code extension.

### 3. [`cargo-generate`]:

```bash
cargo install cargo-generate
```

[`cargo-generate`]: https://crates.io/crates/cargo-generate

> *Note:* You can also just click on `Use this template` instead of using `cargo-generate`.

## Setup

### 1. Initialize the project template
#### Linux / Mac
```bash
cargo generate \
  --git https://github.com/BlueAxolol/STM32H7-rust-template \
  --branch main \
  --name my-app
```

#### Windows CMD
```cmd
cargo generate ^
  --git https://github.com/BlueAxolol/STM32H7-rust-template ^
  --branch main ^
  --name my-app
```

#### Windows PowerShell
```powershell
cargo generate `
  --git https://github.com/BlueAxolol/STM32H7-rust-template `
  --branch main `
  --name my-app
```

> *Note:* change `my-app` to your project name.\
> *Note:* you can also run `cargo generate --git https://github.com/BlueAxolol/STM32H7-rust-template`

There now should be a new folder `my-app` with the template files in it.

### 2. Change authors
#### 2.1 `Cargo.toml`:
In `Cargo.toml` change authors to your name and email
```toml
authors = ["Your Name <youremail@example.com>"]
```

### 3. Run the template

#### 3.1 Connect your STM32

#### 3.2 Run the Program with:
##### in vscode terminal:
```bash
cargo run
```
##### in cmd
```bash
cd patch/to/your/program

cargo run
```

The programm should flash and probe-rs debug output should be running. You should see `Hello, Rust` be printed every second.

### 4. Run Tests

#### 4.1 Connect your STM32

#### 4.2 Run Tests with:
```bash
cargo test --test integration --target thumbv7em-none-eabihf
```

The Tests should flash and you should see:\
``(1/2) running `led_toggles`...``\
``(2/2) running `button_press`...``\
`all tests passed!`

### 5. Write Tests

> *Note:* Only write hardware tests for actual hardware, if you want to check logic you can write normal tests.

#### 5.1 Single tests

In `integration.rs` remove the TestArgs struct and the `#[init]` function and make sure only one `#[test]` function is there.\
Now simply write your test in the `#[test]` function.

#### 5.2 Multiple tests

In `integration.rs` you can see how to write multiple hardware tests. In the `TestArgs` Struct, data gets stored and then called on in the tests.\
The `#[init]` function runs once before the tests and stores variables in the `TestArgs` Struct.
