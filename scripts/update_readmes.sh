#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

function update_readme() {
    cd "$PROJECT_ROOT"/"$1"
    cargo readme --no-license --no-title > README.md
}

update_readme "crates/core"
update_readme "crates/proc_macros"
update_readme "crates/backend_c"
update_readme "crates/backend_csharp"
update_readme "crates/backend_cpython"
update_readme "crates/reference_project"

cp "$PROJECT_ROOT"/crates/core/README.md "$PROJECT_ROOT"