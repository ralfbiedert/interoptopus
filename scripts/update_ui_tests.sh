#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

cp "$PROJECT_ROOT"/backends/c/tests/output/my_header.h "$PROJECT_ROOT"/backends/c/tests/output/my_header.h.expected
cp "$PROJECT_ROOT"/backends/cpython_cffi/tests/output/reference_project.py "$PROJECT_ROOT"/backends/cpython_cffi/tests/output/reference_project.py.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output/Interop.cs "$PROJECT_ROOT"/backends/csharp/tests/output/Interop.cs.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output/Interop.common.cs "$PROJECT_ROOT"/backends/csharp/tests/output/Interop.common.cs.expected
