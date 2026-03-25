alias b := build
alias t := test
alias d := docs
alias doc := docs

# Runs all tests CI would perform before merging a PR.
[arg("verbose", long="verbose", short="v", value="--verbose")]
ci verbose="": (build verbose) (test verbose) test-dotnet lint

# Builds the workspace with all features.
[arg("verbose", long="verbose", short="v", value="--verbose")]
build verbose="":
    cargo build --all-features {{ verbose }}

# Builds the .NET (reverse interop) plugins. Separate step, as .NET output is not reproducible.
build-dotnet-plugins: (_bdp "functions_primitive") (_bdp "functions_behavior") (_bdp "complex") (_bdp "pattern") (_bdp "service_basic") (_bdp "service_async") (_bdp "service_nested")  (_bdp "wire")

# Helper to build a .NET plugin.
_bdp name:
    dotnet build -c Release crates/backend_csharp/tests/reference_plugins/{{ name }}.dll/{{ name }}.csproj -v q 
    cp crates/backend_csharp/tests/reference_plugins/{{ name }}.dll/bin/Release/net10.0/{{ name }}.dll crates/backend_csharp/tests/reference_plugins/_plugins

# Run unit tests, check semantic correctness.
[arg("verbose", long="verbose", short="v", value="--verbose")]
test verbose="" package="":
    cargo nextest run --all-features {{ verbose }}
    cargo test --doc --all-features

# Runs .NET tests.
test-dotnet:
    # Make sure the DLL + Interop files exist
    cargo build -p reference_project
    cargo test --test mod reference_project::interop
    dotnet test crates/backend_csharp/tests/reference_project/reference_project_tests.csproj

# Runs .NET benchmarks.
bench-dotnet:
    # Make sure the DLL + Interop files exist
    cargo build -p reference_project --release
    cargo test --test mod reference_project::interop
    dotnet run -c Release --project crates/backend_csharp/benches/dotnet/dotnet_benchmarks.csproj

# Run linters, check tidiness.
lint:
    cargo fmt --check
    cargo clippy -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
    diff -q crates/core/README.md README.md # Make sure top-level README is up to date.

# Install all required tools, needs `binstall`, see https://github.com/cargo-bins/cargo-binstall
binstall-deps force="":
    cargo binstall cargo-insta --disable-telemetry --no-confirm --secure {{ force }}
    cargo binstall cargo-nextest --disable-telemetry --no-confirm --secure {{ force }}

# Opens cargo docs using nightly for doc feature bubbles.
docs open="":
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features {{ open }}

# Updates the top-level README from the core crate's README (the source of truth).
update-readme:
    cp crates/core/README.md README.md

# Update UI snapshots (.snap and .stderr) files
update-snapshots:
    INSTA_UPDATE=always TRYBUILD=overwrite cargo nextest run --all-features

# Generate 8 random 128-bit IDs in hex format.
ids:
    for i in $(seq 1 8); do od -An -tx1 -N16 /dev/urandom | tr -d ' \n' | sed 's/^/0x/' | tr 'a-f' 'A-F'; echo; done

# Can be used by agents for the current task.
test-agent:
    # Agents: Feel free to update the test logic here for the task at hand.
    cargo test --test mod reference_plugins::service::define_plugins
    dotnet build crates/backend_csharp/tests/reference_plugins/service_basic.dll/service_basic.csproj
    dotnet build crates/backend_csharp/tests/reference_plugins/service_async.dll/service_async.csproj
    dotnet build crates/backend_csharp/tests/reference_plugins/service_nested.dll/service_nested.csproj
    cargo test --test mod reference_plugins::service -p interoptopus_csharp
