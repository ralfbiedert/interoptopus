build:
    cargo build

test verbose="":
    cargo test --all-features {{verbose}}

ci verbose="":
    cargo build {{verbose}}
    cargo fmt --check
    cargo clippy -- -D warnings
    just test {{verbose}}
