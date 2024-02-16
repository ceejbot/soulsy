set windows-shell := ["pwsh.exe", "-Command"]
set shell := ["bash", "-uc"]
set dotenv-load := true

SPRIGGIT := "~/bin/spriggit"
TESTMOD := "/mnt/g/MO2Skyrim/Soulsy HUD dev version"

# List available recipes.
help:
    just -l

# Build everything from a clean repo. One-stop shop.
full-build: tools cmake build archive

# Install required tools.
@tools:
    rustup install nightly
    cargo install cargo-nextest
    cargo install tomato-toml

# Run initial cmake to generate project files. Requires Windows.
cmake:
    cmake --preset vs2022-windows

# Use cargo & cmake to compile the mod in release mode. Requires Windows.
@build:
    cargo build --release
    cmake --build --preset vs2022-windows --config Release

# Format both Rust & C++. Can run anywhere.
@format:
    cargo +nightly fmt
    find src -iname '*.h' -o -iname '*.cpp' | xargs clang-format -i

# Clippy.
@lint:
	cargo clippy --all-targets --no-deps

# Run rust tests. Cannot run on Windows (yet; use Mac or WSL Ubuntu for now).
@test:
    cargo nextest run -E 'not test(/.*pack_complete/)'

# Run icon checks.
@test-icons:
	cargo nextest run -- soulsy_pack_complete thicc_pack_complete

# Generate source files list for CMake. Requires bash. Use a *nix.
[unix]
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

# Set the version in four (!!) places and tag the repo to match. Requires bash.
[unix]
tag VERSION:
	#!/usr/bin/env bash
	sed="sed"
	ignored=$(which gsed)
	if [ $? == 0 ]; then
		sed="gsed"
	fi
	set -e
	tomato set package.version {{VERSION}} Cargo.toml
	# update the version header for the plugin
	$sed -i -e 's/set(VERSION [0-9][0-9]*\.[0-9]*\.[0-9]*\(\.[0-9]*\)/set(VERSION {{VERSION}}\1/' CMakeLists.txt
	# update the lock file
	cargo check --target-dir target/wsl-check
	jq '."version-string" = "{{VERSION}}"' vcpkg.json > vcpkg.tmp
	mv vcpkg.tmp vcpkg.json
	# update the fomod version
	iconv -f UTF-16LE -t UTF-8 installer/fomod/info.xml >installer/fomod/info_utf8.xml
	$sed -i -e 's/<Version>[0-9][0-9]*\.[0-9]*\.[0-9]*<\/Version>/<Version>{{VERSION}}<\/Version>/' installer/fomod/info_utf8.xml
	iconv -f UTF-8 -t UTF-16LE installer/fomod/info_utf8.xml >installer/fomod/info.xml
	rm installer/fomod/info_utf8.xml
	git commit CMakeLists.txt Cargo.toml Cargo.lock installer/fomod/info.xml vcpkg.json -m "v{{VERSION}}"
	git tag "v{{VERSION}}"
	echo "Release tagged for version v{{VERSION}}"

# Copy the built mod files to my test mod.
[unix]
install:
    #!/usr/bin/env bash
    echo "copying to live mod for testing..."
    outdir="{{TESTMOD}}"
    rsync -a installer/core/ "$outdir"
    cp -p build/Release/SoulsyHUD.{dll,pdb} "$outdir"/SKSE/plugins/
    date

# Copy English translation to other translation files.
[unix]
translations:
    #!/usr/bin/env bash
    declare -a langs=(czech french german italian japanese polish russian spanish)
    for lang in "${langs[@]}"; do
        cp -p installer/core/Interface/Translations/SoulsyHUD_english.txt installer/core/Interface/Translations/SoulsyHUD_$lang.txt
    done

# check that all $ strings in config have matching translation strings
[unix]
check-translations:
	mcm-meta-helper --moddir installer/core check all


# Create a mod archive and 7zip it. Requires bash.
[unix]
archive:
    #!/usr/bin/env bash
    set -e
    version=$(tomato get package.version Cargo.toml)
    release_name=SoulsyHUD_v${version}
    mkdir -p "releases/$release_name"
    cp -rp installer/* "releases/${release_name}/"
    cp -p build/Release/SoulsyHUD.dll "releases/${release_name}/core/SKSE/plugins/SoulsyHUD.dll"
    cp -p build/Release/SoulsyHUD.pdb "releases/${release_name}/core/SKSE/plugins/SoulsyHUD.pdb"
    rm "releases/${release_name}/core/scripts/source/TESV_Papyrus_Flags.flg"
    cd releases
    rm -f "$release_name"_fomod.7z
    7z a "$release_name"_fomod.7z "$release_name"
    rm -rf "$release_name"
    cd ..
    echo "Mod archive for v${version} ready at releases/${release_name}.7z"

# Make the two icon pack archives.
packs:
	#!/usr/bin/env bash
	set -e
	ar="7z"
	if [ -z $(which $ar) ]; then
		ar="7zz"
	fi
	mkdir -p releases/SoulsyHUD_{soulsy,thicc}_icon_pack/SKSE/plugins/resources/icons
	rsync -a installer/icon-pack-soulsy/ releases/SoulsyHUD_soulsy_icon_pack/SKSE/plugins/resources/icons
	rsync -a installer/icon-pack-thicc/ releases/SoulsyHUD_thicc_icon_pack/SKSE/plugins/resources/icons
	cd releases
	rm -f SoulsyHUD_thicc_icon_pack.7z
	"$ar" a SoulsyHUD_thicc_icon_pack.7z SoulsyHUD_thicc_icon_pack
	rm -f SoulsyHUD_soulsy_icon_pack.7z
	"$ar" a SoulsyHUD_soulsy_icon_pack.7z SoulsyHUD_soulsy_icon_pack
	rm -rf SoulsyHUD_soulsy_icon_pack/ SoulsyHUD_thicc_icon_pack/
	echo "Two mod packs archived in releases/"


# Use spriggit to dump the plugin to text.
plugin-ser:
    {{SPRIGGIT}} serialize --InputPath ./installer/core/SoulsyHUD.esl --OutputPath ./plugin/ --GameRelease SkyrimSE --PackageName Spriggit.Json

# Use spriggit to rehydrate the plugin.
@plugin-de:
    {{SPRIGGIT}} deserialize --InputPath ./plugin --OutputPath ./SoulsyHUD_test.esl

# Remove archive files.
@clean:
    rm -rf releases/

# Remove archive files & all build artifacts.
spotless: clean
    cargo clean
    rm -rf build

# The rest of these are stubs so windows doesn't just hork.

[windows]
@archive:
    echo "Run this recipe in a bash shell."

[windows]
@check-translations:
    echo "Run this recipe in a bash shell."

[windows]
@install:
    echo "Run this recipe in a bash shell."

[windows]
@translations:
    echo "Run this recipe in a bash shell."

[windows]
@tag VERSION:
	echo "Run this in a bash shell or somewhere with sed on the path."

[windows]
@sources:
    echo "Run this recipe in a bash shell."
