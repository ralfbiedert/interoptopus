#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

cp "$PROJECT_ROOT"/backends/c/tests/output/my_header.h "$PROJECT_ROOT"/backends/c/tests/output/my_header.h.expected
cp "$PROJECT_ROOT"/backends/cpython_cffi/tests/output/reference_project.py "$PROJECT_ROOT"/backends/cpython_cffi/tests/output/reference_project.py.expected
cp "$PROJECT_ROOT"/backends/cpython/tests/output/reference_project.py "$PROJECT_ROOT"/backends/cpython/tests/output/reference_project.py.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output_safe/Interop.cs "$PROJECT_ROOT"/backends/csharp/tests/output_safe/Interop.cs.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output_safe/Interop.common.cs "$PROJECT_ROOT"/backends/csharp/tests/output_safe/Interop.common.cs.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output_unsafe/Interop.cs "$PROJECT_ROOT"/backends/csharp/tests/output_unsafe/Interop.cs.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output_unsafe/Interop.common.cs "$PROJECT_ROOT"/backends/csharp/tests/output_unsafe/Interop.common.cs.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output_unity/Assets/Interop.cs "$PROJECT_ROOT"/backends/csharp/tests/output_unity/Assets/Interop.cs.expected
cp "$PROJECT_ROOT"/backends/csharp/tests/output_unity/Assets/Interop.common.cs "$PROJECT_ROOT"/backends/csharp/tests/output_unity/Assets/Interop.common.cs.expected
