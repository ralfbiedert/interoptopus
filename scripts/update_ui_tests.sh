#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

cp "$PROJECT_ROOT"/interoptopus_backend_c/tests/output/my_header.h.generated "$PROJECT_ROOT"/interoptopus_backend_c/tests/output/my_header.h
cp "$PROJECT_ROOT"/interoptopus_backend_cpython_cffi/tests/output/reference_project.py.generated "$PROJECT_ROOT"/interoptopus_backend_cpython_cffi/tests/output/reference_project.py
cp "$PROJECT_ROOT"/interoptopus_backend_csharp/tests/output/Interop.cs.generated "$PROJECT_ROOT"/interoptopus_backend_csharp/tests/output/Interop.cs
