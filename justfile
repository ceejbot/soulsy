set windows-shell := ["pwsh.exe", "-Command"]
set shell := ["bash", "-uc"]
set dotenv-load := true

shbang := if os_family() == "windows" { "rust-script.exe" } else { "/usr/bin/env rust-script" }

# List available recipes.
help:
    just -l

# Install required tools.
@install:
    rustup install nightly
    cargo install nextest
    cargo install tomato-toml
    cargo install rust-script

# Run initial cmake step.
setup:
    cmake --preset vs2022-windows

# Rebuild the archive for testing. Requires windows.
@rebuild:
    if (test-path build/Release/SoulsyHUD.dll) { rm build/Release/SoulsyHUD.dll }
    cargo build --release
    cmake --build --preset vs2022-windows --config Release

# Lint Rust.
@lint:
    cargo clippy

# Fix clippy lints and format both Rust & C++.
@format:
    cargo clippy --fix --allow-dirty
    cargo +nightly fmt
    find src -iname '*.h' -o -iname '*.cpp' | xargs clang-format -i

# Generate source files list for CMake. Bash.
sources:
    #!/bin/bash
    set -e
    echo "set(headers \${headers}" > test.txt
    headers=$(find ./src -name \*\.h | sort)
    echo "${headers}" >> test.txt
    echo ")" >> test.txt
    echo "set(sources \${sources}" >> test.txt
    echo "    \${headers}" >> test.txt
    cpps=$(find ./src -name \*\.cpp | sort)
    echo "${cpps}" >> test.txt
    echo ")" >> test.txt
    sed -e 's/^\.\//    /' test.txt > cmake/sourcelist.cmake
    rm test.txt

# Set the crate version and tag the repo to match. Bash.
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
    # update the version header for the plugin
    sed -i -e 's/set(VERSION [0-9][0-9]*\.[0-9]*\.[0-9]*\(\.[0-9]*\)/set(VERSION {{VERSION}}\1/' CMakeLists.txt
    git commit Cargo.toml Cargo.lock -m "v{{VERSION}}"
    git tag "v{{VERSION}}"
    echo "Release tagged for version v{{VERSION}}"

# Create a mod archive and 7zip it. Requires bash.
archive:
    #!/usr/bin/env bash
    set -e
    version=$(tomato get package.version Cargo.toml)
    outdir=SoulsyHUD_v${version}
    mkdir -p "$outdir"
    cp -rp data/* "$outdir"/
    cp -p build/Release/SoulsyHUD.dll "$outdir"/SKSE/plugins/SoulsyHUD.dll
    cp -p build/Release/SoulsyHUD.pdb "$outdir"/SKSE/plugins/SoulsyHUD.pdb
    rm "${outdir}"/scripts/source/TESV_Papyrus_Flags.flg
    #7z a "$outdir".7z "$outdir"
    echo "Mod directory ready at ${outdir}; archive at ${outdir}.7z"
    echo "don't check this in, but copying to live mod..."
    cp -p build/Release/SoulsyHUD.dll "/mnt/g/VortexStaging/Soulsy HUD/SKSE/plugins/SoulsyHUD.dll"
    cp -p build/Release/SoulsyHUD.pdb "/mnt/g/VortexStaging/Soulsy HUD/SKSE/plugins/SoulsyHUD.pdb"

# Copy English translation to other translation files.
translations:
    #!/bin/bash
    declare -a langs=(czech french german italian japanese polish russian spanish)
    for lang in "${langs[@]}"; do
        cp data/Interface/Translations/SoulsyHUD_english.txt data/Interface/Translations/SoulsyHUD_$lang.txt
    done

# The traditional
@clean:
    rm -f archive.7z
    rm -rf archive/

# A little niche, but still handy
spotless: clean
    cargo clean
    rm -rf build

# Pwsh version of the timeless classic.
@clean-win:
    if (test-path archive.7z) { remove-item archive.7z }
    if (test-path archive) { rm archive -r -force }

# Pwsh version of the recipe for the ultra-tidy
@spotless-win: clean-win
    cargo clean
    if (test-path build) { rm build -r -force }
