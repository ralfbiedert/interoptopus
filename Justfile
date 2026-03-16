# Builds the workspace with all features.
[arg("verbose", long="verbose", short="v", value="--verbose")]
build verbose="":
    cargo build --all-features {{ verbose }}

# Checks unit tests.
[arg("verbose", long="verbose", short="v", value="--verbose")]
test verbose="" package="":
    cargo nextest run --all-features {{ verbose }}

# Runs C# tests via dotnet.
test-dotnet:
    # Make sure the DLL + Interop files exist
    cargo build -p reference_project
    cargo test --test mod reference_project::interop
    # Run .NET tests
    dotnet test crates/backend_csharp/tests/reference_project/reference_project_tests.csproj

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

# Opens cargo docs using nightly for doc feature bubbles.
docs:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps --open

# Updates the top-level README from the core crate's README (the source of truth).
update-readme:
    cp crates/core/README.md README.md

# Can be used by agents for the current task.
test-agent:
    # Agents: Feel free to update the test logic here for the task at hand.
    # cargo test --test mod reference_project::interop
