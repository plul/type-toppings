_list:
    @just --list --unsorted

# Check project
check:
    bc-check

    # Features
    cargo hack --feature-powerset clippy --tests --examples -- -D warnings
    cargo hack --feature-powerset test

    # Check documentation
    RUSTDOCFLAGS='-Dwarnings' cargo hack --feature-powerset doc --no-deps

    # Check for unused/outdated dependencies
    # NOTE: Waiting for edition 2024 support
    # cargo udeps

    # NOTE: Waiting for edition 2024 support
    # cargo outdated --depth=1

    # Check Nix
    nix flake check

# Format project
fmt:
    bc-fmt

# Update flake inputs
update-flake-inputs:
    nix flake update
