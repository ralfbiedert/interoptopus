#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

cp "$PROJECT_ROOT"/interoptopus_backend_c/tests/output/my_header.h "$PROJECT_ROOT"/interoptopus_backend_c/tests/output/my_header.h.expected
cp "$PROJECT_ROOT"/interoptopus_backend_cpython_cffi/tests/output/reference_project.py "$PROJECT_ROOT"/interoptopus_backend_cpython_cffi/tests/output/reference_project.py.expected
cp "$PROJECT_ROOT"/interoptopus_backend_csharp/tests/output/Interop.cs "$PROJECT_ROOT"/interoptopus_backend_csharp/tests/output/Interop.cs.expected
