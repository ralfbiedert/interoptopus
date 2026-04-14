alias b := build
alias t := test
alias d := docs
alias doc := docs

# Runs all tests CI would perform before merging a PR.
[arg("verbose", long="verbose", short="v", value="--verbose")]
ci verbose="": (build verbose) (test verbose) test-dotnet test-c lint

# Builds the workspace with all features.
[arg("verbose", long="verbose", short="v", value="--verbose")]
build verbose="":
    cargo build --all-features {{ verbose }}

# Builds the .NET (reverse interop) plugins. Separate step, as .NET output is not reproducible.
build-dotnet-plugins: (_bdp_ref "functions_primitive") (_bdp_ref "functions_behavior") (_bdp_ref "complex") (_bdp_ref "pattern") (_bdp_ref "service_basic") (_bdp_ref "service_async") (_bdp_ref "service_async_cancel") (_bdp_ref "service_nested") (_bdp_ref "wire") (_bdp_p "exceptions") (_bdp_p "memory")

# Helper to build a .NET `reference-plugins` plugin.
_bdp_ref name:
    dotnet build -c Release crates/backend_csharp/tests/reference_plugins/{{ name }}.dll/{{ name }}.csproj -v q 
    cp crates/backend_csharp/tests/reference_plugins/{{ name }}.dll/bin/Release/net10.0/{{ name }}.dll crates/backend_csharp/tests/reference_plugins/_plugins

# Helper to build a .NET `plugins` plugin.
_bdp_p name:
    dotnet build -c Release crates/backend_csharp/tests/backend_plugins/{{ name }}.dll/{{ name }}.csproj -v q
    cp crates/backend_csharp/tests/backend_plugins/{{ name }}.dll/bin/Release/net10.0/{{ name }}.dll crates/backend_csharp/tests/backend_plugins/_plugins

# Run unit tests, check semantic correctness.
[arg("verbose", long="verbose", short="v", value="--verbose")]
test verbose="" package="":
    cargo nextest run --all-features {{ verbose }}
    cargo test --doc --all-features

# Runs .NET tests.
test-dotnet:
    # Make sure the DLL + Interop files exist
    cargo build -p reference_project  --all-features
    cargo test --test mod reference_project::interop  --all-features
    dotnet test crates/backend_csharp/tests/reference_project/Tests/Tests.csproj
    # hello_world example
    cargo build -p hello_world
    cargo test -p hello_world -- generate_csharp_bindings
    dotnet test examples/hello_world/bindings_csharp/hello_world.csproj

# Runs C/C++ tests (gtest).
test-c: (_test_c "hello_world" "examples/hello_world/bindings_c") (_test_c "reference_project_c" "crates/backend_c/tests/reference_project")

# Helper: cmake configure + build + ctest. The --config / -C Debug flags are
# needed for multi-config generators (MSVC) and are harmless on single-config ones.
_test_c rust_crate dir:
    cargo build -p {{ rust_crate }}
    cmake -S {{ dir }} -B {{ dir }}/build -DCMAKE_BUILD_TYPE=Debug -DRUST_LIB_DIR={{`pwd`}}/target/debug
    cmake --build {{ dir }}/build --config Debug
    ctest --test-dir {{ dir }}/build --output-on-failure -C Debug

# Runs .NET benchmarks.
bench-dotnet:
    # Make sure the DLL + Interop files exist
    cargo build -p reference_project --release  --all-features
    cargo test --test mod reference_project::interop  --all-features
    dotnet run -c Release --project crates/backend_csharp/benches/dotnet/dotnet_benchmarks.csproj

# Runs C/C++ benchmarks (Google Benchmark).
bench-c:
    cargo build -p reference_project_c --release
    cargo test -p interoptopus_c --test mod reference_project::interop
    cmake -S crates/backend_c/benches -B crates/backend_c/benches/build -DCMAKE_BUILD_TYPE=Release -DRUST_LIB_DIR={{`pwd`}}/target/release
    cmake --build crates/backend_c/benches/build --config Release
    ctest --test-dir crates/backend_c/benches/build -V -C Release

# Run linters, check tidiness.
lint:
    cargo fmt --check
    cargo clippy -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
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
    # find . -name "*.snap" -delete
    INSTA_UPDATE=always TRYBUILD=overwrite cargo nextest run --all-features

# Generate 8 random 128-bit IDs in hex format.
ids:
    for i in $(seq 1 8); do od -An -tx1 -N16 /dev/urandom | tr -d ' \n' | sed 's/^/0x/' | tr 'a-f' 'A-F'; echo; done

# Can be used by agents for the current task.
test-agent:
    # Agents: Feel free to update the test logic here for the task at hand.
    cargo test -p interoptopus_csharp --test mod reference_project::interop
