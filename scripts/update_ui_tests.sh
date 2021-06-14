#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

cp "$PROJECT_ROOT"/README.md "$PROJECT_ROOT"/interoptopus

cd "$PROJECT_ROOT"/interoptopus_proc && cargo publish
cd "$PROJECT_ROOT"/interoptopus && cargo publish
cd "$PROJECT_ROOT"/interoptopus_reference_project && cargo publish
cd "$PROJECT_ROOT"/interoptopus_backend_csharp && cargo publish
cd "$PROJECT_ROOT"/interoptopus_backend_c && cargo publish
cd "$PROJECT_ROOT"/interoptopus_backend_cpython_cffi && cargo publish
