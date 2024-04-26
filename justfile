_list:
    @just --list --unsorted

# Check project
check:
    @ just check-fmt
    @ just hack test
    @ just hack clippy --tests --examples -- -D warnings
    @ RUSTDOCFLAGS='-Dwarnings' just hack doc --no-deps
    fd --extension=toml -X taplo lint
    nix flake show
    cargo udeps
    cargo outdated --depth=1

# Check formatting
check-fmt:
    just --unstable --fmt --check
    nix fmt -- --check .
    fd --extension=toml -X taplo fmt --check
    cargo fmt -- --check
    fd --extension=md -X prettier --check

# Format
fmt:
    just --unstable --fmt
    nix fmt
    fd --extension=toml -X taplo fmt
    cargo fmt
    fd --extension=md -X prettier --write 

[private]
hack *ARGS:
    cargo hack --feature-powerset {{ ARGS }}

# Run cargo check on changes
watch-check:
    watchexec --clear --restart --exts='rs,toml' -- cargo check --tests --examples

update-flake-inputs:
    nix flake update
