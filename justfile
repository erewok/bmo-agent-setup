# Default recipe: list available recipes
default:
    just --list

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run all tests across all crates
test:
    cargo test --all

# Check the project for errors without producing output
check:
    just fmt-check
    just lint
    cargo check --all

# Format all source files in place
fmt:
    cargo fmt --all

# Check formatting without making changes (CI-safe)
fmt-check:
    cargo fmt --all -- --check

# Run Clippy lints and treat all warnings as errors
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Remove build artifacts
clean:
    cargo clean

# Run the release binary, writing output to the given directory
run output="./claude-code-env":
    cargo run --release -- --output {{output}}

# Run the full CI pipeline: format check, lint, release build, tests
ci: fmt-check lint build-release test
