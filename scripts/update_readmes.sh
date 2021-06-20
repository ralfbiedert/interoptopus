#
# Publish all crates.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

function update_readme() {
    cd "$PROJECT_ROOT"/"$1"
    cargo readme --no-license --no-title > README.md
}

update_readme "interoptopus"
update_readme "interoptopus_proc"
update_readme "interoptopus_backend_c"
update_readme "interoptopus_backend_csharp"
update_readme "interoptopus_backend_cpython_cffi"
update_readme "interoptopus_reference_project"

cp "$PROJECT_ROOT"/interoptopus/README.md "$PROJECT_ROOT"