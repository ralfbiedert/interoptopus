# Builds the workspace with all features.
[arg("verbose", long="verbose", short="v", value="--verbose")]
build verbose="":
    cargo build --all-features {{ verbose }}

# Run unit tests, check semantic correctness.
[arg("verbose", long="verbose", short="v", value="--verbose")]
test verbose="" package="":
    cargo nextest run --all-features {{ verbose }}
    cargo test --doc --all-features

# Runs C# tests via dotnet.
test-dotnet:
    # Make sure the DLL + Interop files exist
    cargo build -p reference_project
    cargo test --test mod reference_project::interop
    # Run .NET tests
    dotnet test crates/backend_csharp/tests/reference_project/reference_project_tests.csproj

# Run linters, check tidiness.
lint:
    cargo fmt --check
    cargo clippy -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
    diff -q crates/core/README.md README.md # Make sure top-level README is up to date.

# Runs all tests CI would perform before merging a PR.
[arg("verbose", long="verbose", short="v", value="--verbose")]
ci verbose="": (build verbose) (test verbose) lint

# Install all required tools, needs `binstall`, see https://github.com/cargo-bins/cargo-binstall
binstall-deps:
    cargo binstall cargo-insta --disable-telemetry --no-confirm --secure --force
    cargo binstall cargo-nextest --disable-telemetry --no-confirm --secure --force

# Opens cargo docs using nightly for doc feature bubbles.
docs open="":
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps {{ open }}

# Updates the top-level README from the core crate's README (the source of truth).
update-readme:
    cp crates/core/README.md README.md

# Generate 8 random 128-bit IDs in hex format.
ids:
    for i in $(seq 1 8); do od -An -tx1 -N16 /dev/urandom | tr -d ' \n' | sed 's/^/0x/' | tr 'a-f' 'A-F'; echo; done

# Can be used by agents for the current task.
test-agent:
    # Agents: Feel free to update the test logic here for the task at hand.
    # cargo test --test mod reference_project::interop
