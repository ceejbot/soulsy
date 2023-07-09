set windows-shell := ["powershell"]
set shell := ["bash", "-uc"]

# List available recipes.
help:
    just -l

# Install required tools.
@install:
    rustup install nightly
    cargo install nextest
    cargo install tomato

# Set up the repo for the first time.
setup:
    git submodule update --init --recursive
    cmake --preset vs2022-windows

# Build for debugging.
@debug: sources
    cmake --build --preset vs2022-windows --config Debug

# Build for release.
@release: sources
    cmake --build --preset vs2022-windows --config Release
    # cargo build --release

# Run the same tests we run in CI.
@ci:
    cargo nextest run
    cargo clippy

# Fix clippy lints and format.
@lint:
    cargo clippy --fix
    cargo +nightly fmt

# Generate source files list for CMake.
sources:
    #!/bin/bash
    set -e
    echo "set(headers \${headers}" > test.txt
    headers=$(find . -name \*\.h | sort)
    echo "${headers}" >> test.txt
    echo ")" >> test.txt
    echo "set(sources \${sources}" >> test.txt
    echo "    \${headers}" >> test.txt
    cpps=$(find . -name \*\.cpp | sort)
    echo "${cpps}" >> test.txt
    echo ")" >> test.txt
    sed -e 's/^\.\//    /' test.txt > cmake/sourcelist.cmake
    rm test.txt

# Set the crate version and tag the repo to match.
tag VERSION:
    #!/usr/bin/env bash
    set -e
    status=$(git status --porcelain)
    if [ "$status" != ""  ]; then
        echo "There are uncommitted changes! Cowardly refusing to act."
        exit 1
    fi
    tomato set package.version {{VERSION}} Cargo.toml
    # update the lock file
    cargo check
    git commit Cargo.toml Cargo.lock -m "v{{VERSION}}"
    git tag "v{{VERSION}}"
    echo "Release tagged for version v{{VERSION}}"
