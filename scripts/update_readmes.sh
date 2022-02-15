#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

function update_readme() {
    cd "$PROJECT_ROOT"/"$1"
    cargo readme --no-license --no-title > README.md
}

update_readme "core"
update_readme "proc_macros"
update_readme "backends/c"
update_readme "backends/csharp"
update_readme "backends/cpython"
update_readme "reference_project"

cp "$PROJECT_ROOT"/core/README.md "$PROJECT_ROOT"