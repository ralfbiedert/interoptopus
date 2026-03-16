# Builds the workspace with all features.
[arg("verbose", long="verbose", short="v", value="--verbose")]
build verbose="":
    cargo build --all-features {{verbose}}

# Checks unit tests.
[arg("verbose", long="verbose", short="v", value="--verbose")]
test verbose="" package="":
    cargo nextest run --all-features {{verbose}}

# Checks extra linting
lint:
    cargo fmt --check
    cargo clippy -- -D warnings

# Runs all tests CI would perform before merging a PR.
[arg("verbose", long="verbose", short="v", value="--verbose")]
ci verbose="": (build verbose) (test verbose) lint

# Install all required tools, needs `binstall`, see https://github.com/cargo-bins/cargo-binstall
binstall-deps:
    cargo binstall cargo-insta --disable-telemetry --no-confirm --secure
    cargo binstall cargo-nextest --disable-telemetry --no-confirm --secure

# Can be used by agents for the current task.
test-agent:
    # Agents: Feel free to update the test logic here for the task at hand.
    # cargo test TODO
    cargo test --test mod reference_project::interop
