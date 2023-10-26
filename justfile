_list:
    @just --list --unsorted

# Check project
check: check-fmt
    @ just hack test
    @ just hack clippy --tests --examples -- -D warnings
    @ RUSTDOCFLAGS='-Dwarnings' just hack doc --no-deps
    taplo lint `fd --extension=toml`
    nix flake show
    cargo udeps
    cargo outdated --depth=1

# Check formatting
check-fmt:
    just --unstable --fmt --check
    nix fmt -- --check .
    taplo fmt --check `fd --extension=toml`
    cargo fmt -- --check
    prettier --check `fd --extension=md`

# Format
fmt:
    just --unstable --fmt
    nix fmt
    taplo fmt `fd --extension=toml`
    cargo fmt
    prettier --write `fd --extension=md`

[private]
hack *ARGS:
    cargo hack --feature-powerset {{ ARGS }}

# Run cargo check on changes
watch-check:
    watchexec --clear --restart --exts='rs,toml' -- cargo check --tests --examples
