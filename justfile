set windows-shell := ["pwsh.exe", "-Command"]
set shell := ["bash", "-uc"]
set dotenv-load := true

spriggit := "~/bin/spriggit"

# List available recipes.
help:
    just -l

# Install required tools.
@tools:
    rustup install nightly
    cargo install cargo-nextest
    cargo install tomato-toml

# Run initial cmake step. Cannot be run in WSL.
cmake:
    cmake --preset vs2022-windows

# Rebuild the archive for testing. Requires windows.
@build:
    # if (test-path build/Release/SoulsyHUD.dll) { rm build/Release/SoulsyHUD.dll }
    cargo build --release
    cmake --build --preset vs2022-windows --config Release

# Check clippy lints and format both Rust & C++.
@lint:
    cargo +nightly fmt
    find src -iname '*.h' -o -iname '*.cpp' | xargs clang-format -i
    cargo clippy

# Run rust tests
@test:
    cargo nextest run

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
    tomato set package.version {{VERSION}} Cargo.toml
    # update the version header for the plugin
    sed -i -e 's/set(VERSION [0-9][0-9]*\.[0-9]*\.[0-9]*\(\.[0-9]*\)/set(VERSION {{VERSION}}\1/' CMakeLists.txt
    # update the lock file
    #cargo check
    git commit CMakeLists.txt Cargo.toml Cargo.lock -m "v{{VERSION}}"
    git tag "v{{VERSION}}"
    echo "Release tagged for version v{{VERSION}}"

# Create a mod archive and 7zip it. Requires bash.
archive:
    #!/usr/bin/env bash
    set -e
    version=$(tomato get package.version Cargo.toml)
    release_name=SoulsyHUD_v${version}
    mkdir -p "releases/$release_name"
    cp -rp data/* "releases/${release_name}/"
    cp -p build/Release/SoulsyHUD.dll "releases/${release_name}/SKSE/plugins/SoulsyHUD.dll"
    cp -p build/Release/SoulsyHUD.pdb "releases/${release_name}/SKSE/plugins/SoulsyHUD.pdb"
    rm "releases/${release_name}/scripts/source/TESV_Papyrus_Flags.flg"
    cd releases
    7z a "$release_name".7z "$release_name"
    rm -rf "$release_name"
    cd ..
    echo "Mod archive for v${version} ready at releases/${release_name}.7z"

# copy files to my test mod
install:
    #!/usr/bin/env bash
    echo "copying to live mod for testing..."
    outdir="/mnt/g/Vortex Mods/skyrimse/Soulsy HUD dev version/"
    cp -rp data/* "$outdir"
    cp -p build/Release/SoulsyHUD.dll "${outdir}/SKSE/plugins/SoulsyHUD.dll"
    cp -p build/Release/SoulsyHUD.pdb "${outdir}/SKSE/plugins/SoulsyHUD.pdb"

# Copy English translation to other translation files.
translations:
    #!/usr/bin/env bash
    declare -a langs=(czech french german italian japanese polish russian spanish)
    for lang in "${langs[@]}"; do
        cp -p data/Interface/Translations/SoulsyHUD_english.txt data/Interface/Translations/SoulsyHUD_$lang.txt
    done

# check that all $ strings in config have matching translation strings
check-translations:
    #!/usr/bin/env bash
    converted=$(iconv -f utf-16 -t utf-8 data/Interface/Translations/SoulsyHUD_english.txt > tmp.txt)

    # I am too lazy to figure out how to get jq to do all of it.
    keys=$(cat data/mcm/config/SoulsyHUD/config.json | jq '.pages[] | .content[] | .[]' -r | grep "\\$" | tr -d '," $' | sort | uniq)
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

    ar=$(which 7zz)
    if [[ -z "$ar" ]]; then
        ar=$(which 7z)
    fi
    if [[ -z "$ar" ]]; then
        echo "7zip not found at 7z or 7zz. You need to install or alias it to archive."
        exit 1
    fi

    set -e

    mkdir -p releases
    for layout in layouts/*.toml; do
        name="${layout/layouts\/SoulsyHUD_/}"
        name="SoulsyHUD-layout-${name/.toml/}"
        dest="releases/${name}/SKSE/plugins"
        mkdir -p "releases/${name}/SKSE/plugins"
        cp -p "$layout" "$dest/SoulsyHUD_Layout.toml"
        font=$(tomato get font "$dest/SoulsyHUD_Layout.toml")
        if [[ "$font" =~ "Inter" ]]; then
            mkdir -p "$dest/resources/fonts"
            cp -p "layouts/${font}" "$dest/resources/fonts"
            cp -p "layouts/${font}" "$dest/resources/fonts"
        fi

        cd releases
        ${ar} -y -bsp0 -bso0 a "${name}.7z" "${name}"
        rm -rf "${name}"
        cd ..
        echo "Built ${name}.7z"
    done

    # Build the equip-sets-aware layout
    dest="releases/SoulsyHUD_layout_square/SKSE/plugins"
    mkdir -p "$dest/resources/backgrounds"
    cp -p layouts/square/SoulsyHUD_layout.toml $dest
    cp -p layouts/square/*.svg "$dest/resources/backgrounds/"

    # build the Soulsy icon pack
    dest="releases/SoulsyHUD_icon_pack/SKSE/plugins/resources/icons"
    mkdir -p "$dest"
    cp -rp layouts/icon-pack/*.svg "$dest/"

    # build the THICC icon pack
    dest="releases/THICC_icon_pack/SKSE/plugins/resources/icons"
    mkdir -p "$dest"
    cp -rp layouts/thicc-icon-pack/*.svg "$dest/"

    archive_dirs="SoulsyHUD_icon_pack THICC_icon_pack SoulsyHUD_layout_square"
    cd releases
    for i in $archive_dirs; do
         ${ar} -y -bsp0 -bso0 a "$i.7z" "$i"
        rm -rf "$i"
        echo "Built $i.7z"
    done
    cd ..

# Use spriggit to dump the plugin to text.
plugin-ser:
    {{spriggit}} serialize --InputPath ./data/SoulsyHUD.esl --OutputPath ./plugin/ --GameRelease SkyrimSE --PackageName Spriggit.Json

# Use spriggit to rehydrate the plugin.
@plugin-de:
    {{spriggit}} deserialize --InputPath ./plugin --OutputPath ./SoulsyHUD_test.esl

# The traditional
@clean:
    rm -f archive.7z
    rm -rf archive/

# A little niche, but still handy
spotless: clean
    cargo clean
    rm -rf build
