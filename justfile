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
    cargo clippy --fix --allow-staged
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
    git commit Cargo.toml Cargo.lock -m "v{{VERSION}}"
    git tag "v{{VERSION}}"
    echo "Release tagged for version v{{VERSION}}"

# Bash version of archive creation, sans 7zip step for now.
archive:
    #!/usr/bin/env bash
    set -e
    version=$(tomato get package.version Cargo.toml)
    outdir=SoulsyHUD_v${version}
    mkdir -p "$outdir"
    cp -rp data/* "$outdir"/
    cp -p build/Release/SoulsyHUD.dll "$outdir"/SKSE/plugins/SoulsyHUD.dll
    cp -p build/Release/SoulsyHUD.pdb "$outdir"/SKSE/plugins/SoulsyHUD.pdb
    #7z a "$outdir".7z "$outdir"
    echo "Mod directory ready at ${outdir}; archive at ${outdir}.7z"
    echo "don't check this in, but copying to live mod..."
    cp -p build/Release/SoulsyHUD.dll "/mnt/g/VortexStaging/Soulsy HUD/SKSE/plugins/SoulsyHUD.dll"
    cp -p build/Release/SoulsyHUD.pdb "/mnt/g/VortexStaging/Soulsy HUD/SKSE/plugins/SoulsyHUD.pdb"


# Build a full mod archive; cross-platform.
archive-win:
    #!{{shbang}}
    //! I refuse to write long pwsh scripts, so I inflict a rust-script
    //! on the world instead.
    //!
    //! ```cargo
    //! [dependencies]
    //! fs_extra="1.3.0"
    //! sevenz-rust={version="0.4.3", features=["compress"]}
    //! ```
    fn main() {
        if std::path::Path::new("archive").exists() {
            println!("Existing archive directory found. Bailing.");
            std::process::exit(1);
        }
        println!("Copying source files to `./archive`...");
        std::fs::create_dir_all("archive").expect("couldn't create archive directory");

        // recursive copy stripping off the first path segment
        let mut sources = Vec::new();
        sources.push("data/Interface");
        sources.push("data/mcm");
        sources.push("data/scripts");
        sources.push("data/SKSE");
        let options = fs_extra::dir::CopyOptions::new();
        fs_extra::copy_items(&sources, "archive", &options).expect("make life take the lemons back");

        std::fs::copy("data/SoulsyHUD.esl", "archive/SoulsyHUD.esl").expect("couldn't copy plugin file");
        std::fs::copy("build/Release/SoulsyHUD.dll", "archive/SKSE/plugins/SoulsyHUD.dll").expect("couldn't copy DLL");
        std::fs::copy("build/Release/SoulsyHUD.pdb", "archive/SKSE/plugins/SoulsyHUD.pdb").expect("couldn't copy PDB");

        sevenz_rust::compress_to_path("archive/", "archive.7z").expect("7zip compression failed");
        println!("Archive created! `archive.7z` ready to be uploaded or tested.")
    }

archive-less-precious:
    New-Item -Path "." -Name "soulsyhud" -ItemType "directory"
    Copy-Item -Path "data/*" -Destination "soulsyhud" -Recurse
    Copy-Item "build/Release/SoulsyHUD.dll" -Destination "soulsyhud/SKSE/plugins/"
    Copy-Item "build/Release/SoulsyHUD.pdb" -Destination "soulsyhud/SKSE/plugins/"

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
