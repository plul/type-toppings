_list:
    @just --list

# Check project
check:
    just check-fmt
    just test
    just lint
    just check-docs
    just check-nix
    just check-deps

# Test project
test:
    just hack test

# Lint project
lint:
    just lint-rust
    just lint-toml

# Lint Rust
lint-rust:
    just hack clippy --tests --examples -- -D warnings

# Lint TOML
lint-toml:
    fd --extension=toml -X taplo lint

# Check documentation
check-docs:
    RUSTDOCFLAGS='-Dwarnings' just hack doc --no-deps

# Check for unused/outdated dependencies
check-deps:
    cargo udeps
    cargo outdated --depth=1

# Check Nix
check-nix:
    nix flake check

# Check formatting
check-fmt:
    just check-fmt-just
    just check-fmt-nix
    just check-fmt-toml
    just check-fmt-rust
    just check-fmt-markdown

# Check formatting of justfile
check-fmt-just:
    just --unstable --fmt --check

# Check formatting of Nix
check-fmt-nix:
    fd --extension=nix -X nixfmt --check

# Check formatting of TOML
check-fmt-toml:
    fd --extension=toml -X taplo fmt --check

# Check formatting of Rust
check-fmt-rust:
    cargo fmt -- --check

# Check formatting of Markdown
check-fmt-markdown:
    fd --extension=md -X prettier --check

# Format project
fmt:
    just fmt-just
    just fmt-nix
    just fmt-toml
    just fmt-rust
    just fmt-markdown

# Format Justfile
fmt-just:
    just --unstable --fmt

# Format Nix
fmt-nix:
    fd --extension=nix -X nixfmt

# Format TOML
fmt-toml:
    fd --extension=toml -X taplo fmt

# Format Rust
fmt-rust:
    cargo fmt

# Format Markdown
fmt-markdown:
    fd --extension=md -X prettier --write 

[private]
hack *ARGS:
    cargo hack --feature-powerset {{ ARGS }}

# Update flake inputs
update-flake-inputs:
    nix flake update
