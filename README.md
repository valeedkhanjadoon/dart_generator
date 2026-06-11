# dart

A grammar-based fuzzer for Dart runtimes using [fandango-rs](https://github.com/fandango-fuzzer/fandango-rs).

## Grammar

`grammars/dart.fan` defines a Dart grammar derived from the ANTLR v4 `Dart2Parser.g4` grammar, simplified and ported to fandango-rs.

`programs/` contains example generated programs.

## Code walkthrough

`src/lib.rs` is split into three sections:

- **`defs`** — the `#[derive(Fandango)]` struct that compiles the grammar, plus all visitor and constraint definitions.
- **`fuzzer`** — `gen_and_execute_one()`, which pipe programs to `dart`.
- **`test`** — individual tests mirroring the C crate structure, adapted to dart nonterminals.

### Constraints

These constraints are adapted from the C reference crate to illustrate the same visitor patterns on a different grammar.

| Type | What it checks |
|---|---|
| `BooleansMustBeFalseConstraint` | All `BooleanLiteral` nodes must be `false` |
| `OnlyEvenDecimalIntegersConstraint` | All decimal integer literals must start with an even digit |
| `CombinedConstraintVisitor` | Runs both from the start symbol; fitness is their mediant |

### Generation goals

| Type | What it targets |
|---|---|
| `SourceElementGoal { n }` | At least `n` source elements (top-level statements) |
| `EmptyStatementLimit { n }` | No more than `n` empty statements |

## Experiments

All three experiments report an **execution rate** — the fraction of programs `dart` accepts — so results are directly comparable across them.

| Experiment | Setup | Output |
|---|---|---|
| `experiment1` | Grammar only, pure random generation | Per-program execution result; final execution rate over 10 programs |
| `experiment2` | NSGA-II, 100 generations, combined constraints | Fitness per generation; execution rate of the final population (100 programs) |
| `experiment3` | NSGA-II, 100 generations, combined constraints + `SourceElementGoal(5)` + `EmptyStatementLimit(1)` | Fitness per generation; execution rate of the final population (100 programs) |

## Setup

### 1. Clone the repository

```sh
git clone <repo-url>
cd CompilerTesting2026/dart
```

### 2. Install a nightly Rust toolchain

`rust-toolchain.toml` specifies the required nightly channel.
If you have [rustup](https://rustup.rs/) installed, it will be downloaded automatically on first use.
To install it explicitly:

```sh
rustup toolchain install nightly
```

### 3. Install Dart

On macOS:

```sh
brew tap dart-lang/dart
brew install dart
```

On Ubuntu/Debian:

```sh
# 1. Update package index and install prerequisites
sudo apt-get update
sudo apt-get install apt-transport-https curl gnupg

# 2. Download the Google Linux signing key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://dl-ssl.google.com/linux/linux_signing_key.pub | sudo gpg --dearmor -o /etc/apt/keyrings/google-linux-signing-key.gpg

# 3. Add the Dart APT repository
echo 'deb [signed-by=/etc/apt/keyrings/google-linux-signing-key.gpg arch=amd64] https://storage.googleapis.com/download.dartlang.org/linux/debian stable main' | sudo tee /etc/apt/sources.list.y/dart.list

# 4. Install the Dart SDK
sudo apt-get update
sudo apt-get install dart
```

On Windows (using Chocolatey):
```
choco install dart-sdk
```

### 4. Verify the build

```sh
RUSTFLAGS='-Znext-solver' cargo +nightly build
```

The first build will be slow — fandango-rs compiles the grammar into Rust types at build time.
Subsequent incremental builds are much faster.

If you are working inside the `dart/` directory, `.cargo/config.toml` sets the flag automatically and you can also just run:

```sh
cargo +nightly build
```

### 5. Run the tests

```sh
RUSTFLAGS='-Znext-solver' cargo +nightly test -- --nocapture
```

## Running

Requires a nightly Rust toolchain (see `rust-toolchain.toml`) and `dart` on `PATH`.

`.cargo/config.toml` sets `RUSTFLAGS='-Znext-solver'` automatically, so `cargo check` and IDE background checks (e.g. RustRover) pick it up without any manual configuration.

```sh
# Experiment 1: grammar only, collect execution rate
RUSTFLAGS='-Znext-solver' cargo +nightly run --example experiment1

# Experiment 2: NSGA-II + combined constraints, collect execution rate
RUSTFLAGS='-Znext-solver' cargo +nightly run --example experiment2

# Experiment 3: NSGA-II + combined constraints + goals, collect execution rate
RUSTFLAGS='-Znext-solver' cargo +nightly run --example experiment3

# Run individual constraint tests (useful when developing your own)
RUSTFLAGS='-Znext-solver' cargo +nightly test dart_lang_can_generate -- --nocapture
RUSTFLAGS='-Znext-solver' cargo +nightly test booleans_must_be_false_constraint_alone -- --nocapture
RUSTFLAGS='-Znext-solver' cargo +nightly test only_even_decimal_integers_constraint_alone -- --nocapture
```

## Adapting this for your language

See the equivalent section in the C crate README for the general procedure.
For Dart specifically:

- The grammar was derived from ANTLR v4 and then simplified for fandango-rs — the grammar debugging and porting process from class applies directly here.
- Dart is invoked with `--input-type=module` so that `import` statements parse correctly; your runtime may need different flags.