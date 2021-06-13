#
# Tests all generated bindings work. Should be run before releasing a new version.
#
# Needs the following commands available:
# - cargo
# - dotnet
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

_YELLOW='\033[1;33m'
_GREEN='\033[0;32m'
_RED='\033[0;31m'
_NC='\033[0m' # No Color


function abort() {
    echo ""
    echo -e "${_RED}Error executing previous command.${_NC}."
    echo ""
    exit 1
}

cd "$PROJECT_ROOT"


# RUST
cargo build || abort
cargo test || abort


# C#
cd "$PROJECT_ROOT"/examples/complex/bindings/csharp
dotnet build || abort
dotnet test || abort

cd "$PROJECT_ROOT"/interoptopus_backend_csharp/tests/output/
dotnet build || abort


# C
cd "$PROJECT_ROOT"/examples/complex/bindings/c
# How do I do this from Windows / MSVC command line?
# cmake


# Python
cd "$PROJECT_ROOT"/examples/complex/bindings/python
python app.py || abort

cd "$PROJECT_ROOT"/interoptopus_backend_cpython_cffi/tests/output/
python app.py || abort


echo
echo -e "${_GREEN}All ok!${_NC}"
echo
