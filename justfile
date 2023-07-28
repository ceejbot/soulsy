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

# Fix clippy lints and format both Rust & C++.
@lint-fix:
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
    git commit CMakeLists.txt Cargo.toml Cargo.lock -m "v{{VERSION}}"
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
    7z a "$outdir".7z "$outdir"
    echo "Mod directory ready at ${outdir}; archive at ${outdir}.7z"

# copy files to my test mod
test:
    #!/bin/bash
    echo "copying to live mod for testing..."
    outdir="/mnt/g/VortexStaging/Soulsy HUD dev version/"
    cp -rp data/* "$outdir"
    cp -p build/Release/SoulsyHUD.dll "${outdir}/SKSE/plugins/SoulsyHUD.dll"
    cp -p build/Release/SoulsyHUD.pdb "${outdir}/SKSE/plugins/SoulsyHUD.pdb"


# Copy English translation to other translation files.
translations:
    #!/bin/bash
    declare -a langs=(czech french german italian japanese polish russian spanish)
    for lang in "${langs[@]}"; do
        cp -p data/Interface/Translations/SoulsyHUD_english.txt data/Interface/Translations/SoulsyHUD_$lang.txt
    done

# check that all $ strings in config have matching translation strings
check-translations:
    #!/bin/bash
    converted=$(iconv -f utf-16 -t utf-8 data/Interface/Translations/SoulsyHUD_english.txt > tmp.txt)

    # I am too lazy to figure out how to get jq to do all of it.
    keys=$(cat data/mcm/config/SoulsyHUD/config.json | jq '.content[] | .[]' -r | grep "\\$" | tr -d '," $' | sort | uniq)
    for k in $keys; do
        cmd="grep $k tmp.txt"
        suppressed=$(sh -c "$cmd")
        exit=$?
        if [ $exit != '0' ]; then
            echo "missing translation: $k"
        fi
    done
    rm tmp.txt

# Build mod structures for additional layouts. Bash.
build-layouts:
    #!/usr/bin/env bash
    set -e

    mkdir -p releases
    for layout in layouts/*.toml; do
        name="${layout/layouts\/SoulsyHUD_/}"
        name="SoulsyHUD-layout-${name/.toml/}"
        dest="releases/${name}/SKSE/plugins"
        mkdir -p "releases/${name}/SKSE/plugins"
        cp -p "$layout" "$dest/SoulsyHUD_Layout.toml"
        font=$(tomato get font "$dest/SoulsyHUD_Layout.toml")
        if [ "$font" = "Inter-Medium.ttf" ]; then
            mkdir -p "$dest/resources/fonts"
            cp -p "layouts/Inter-Medium.ttf" "$dest/resources/fonts"
        fi
        cd releases
        7zz -y -bsp0 -bso0 a "${name}.7z" "${name}"
        rm -rf "${name}"
        cd ..
        echo "Built ${name}.7z"
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
