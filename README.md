# Soulsy

[![Test Rust features](https://github.com/ceejbot/soulsy/actions/workflows/test.yaml/badge.svg)](https://github.com/ceejbot/soulsy/actions/workflows/test.yaml) [![Build mod archive](https://github.com/ceejbot/soulsy/actions/workflows/build.yaml/badge.svg)](https://github.com/ceejbot/soulsy/actions/workflows/build.yaml)

Soulsy is a lightweight, fast Souls-style hotkey HUD mod for Skyrim SE and AE. It is inspired by hotkey mods like Elden Equip, iEquip, and LamasTinyHud. It started life as a fork of [LamasTinyHud](https://github.com/mlthelama/LamasTinyHUD), though it has since diverged significantly.

![Screenshot of the hud](./docs/SoulsyHUD_preview.jpeg)

The [NexusMods page](https://www.nexusmods.com/skyrimspecialedition/mods/96210/) has screenshots and videos of the HUD in use as well as player documentation. The documentation is more readable here in markdown. See [Configuring the HUD](./docs/article-options.md) and [Customizing Layouts](docs/article-layouts.md).

## Development goals

My goals are two-fold: make a Souls-style equip HUD that is exactly what I want to use, and learn how to do Rust FFI. A bonus is demonstrating how to write Skyrim native-code mods in Rust.

This project has been released and is in active use. My eventual goal is to move everything except the SKSE plugin glue code to Rust, and have the C++ mostly vanish. There will always be some C++ in the project to interact with the Skyrim reverse-engineered library, which is all in C++ as is the game itself.

## Building

Soulsy is a Rust and C++ project, using CMake to drive Cargo to build the Rust parts. The application logic is implemented in Rust, with a bridge to the C++ libraries required to implement an SKSE plugin. It requires the following to build:

- [Rust](https://rustup.rs) set up for Windows (not for WSL)
- [Visual Studio 2022](https://visualstudio.microsoft.com) with C++ compilers installed
- [CMake](https://cmake.org)
- [vcpkg](https://github.com/microsoft/vcpkg) with `VCPKG_ROOT` set in a user environment variable

The plugin requires the following vcpkg libraries, which will be installed for you:

- [spdlog](https://github.com/gabime/spdlog) (consumed by CommonLib, not by Soulsy)
- [imgui](https://github.com/ocornut/imgui)

Finally, [CommonLibSSE-NG](https://github.com/CharmedBaryon/CommonLibSSE-NG) is pulled in as a git submodule and built. The repo used is [my fork](https://github.com/ceejbot/CommonLibSSE-NG) of CommonLib, which has some minor fixes in addition to the upstream.

There are a number of development conveniences in the [justfile](https://just.systems), including build recipes for Powershell. `cargo install just` if you do not have it. Because I am more comfortable on Unixes than on Windows, some recipes are written in Bash. The just recipes can build, copy to a test mod directory, update version numbers and tag a new release, and build archives for upload to the Nexus.

`cargo --doc open` displays programmer documentation for the Rust side of the plugin. The C++ side is commented, but not to the same degree.

You are absolutely invited to contribute. This project follows the standard [Contributor's Covenant](./CODE_OF_CONDUCT.md).

## Credits

I could not have approached the rendering code without the work in [LamasTinyHud](https://www.nexusmods.com/skyrimspecialedition/mods/82545), so [mlthelama](https://github.com/mlthelama) gets all the props. I also learned a lot about how to make an SKSE plugin by reading their source. Give that HUD a try if you don't like the souls-game style, or want a UI you can edit in-game. The original is the only hotkeys hud mod I tried that worked well in my game, so that's a testimonial.

The icons for the built-in theme are the usual SkyUI icons, plus the `futura-book-bt` true-type font. The background assets were built from scratch but were inspired by the [Untarnished UI skin](https://www.nexusmods.com/skyrimspecialedition/mods/82545) for LamasTinyHUD by [MinhazMurks](https://www.nexusmods.com/skyrimspecialedition/users/26341279).

The built-in icons are the SkyUI icons by psychosteve, which are used in so many places I am not sure how to credit them. The icons for the Ceej remix layout are licensed to me from the Noun Project for use without attribution, but I am going to give attribution anyway because they're great icons. I am using the [Role Playing Game collection](https://thenounproject.com/browse/collection-icon/role-playing-game-70773/?p=1) by [Maxicons](https://thenounproject.com/maxicons/). The THICC icon pack uses icons with permission from the [THICC icon mod](https://www.nexusmods.com/skyrimspecialedition/mods/90508).

The font in use for some layouts is [Inter](https://rsms.me/inter/).

[cxx](https://cxx.rs/) made developing the C++/Rust bridge a snap. This crate unlocks Rust as a viable language for all of your modding needs. [bindgen](https://rust-lang.github.io/rust-bindgen/introduction.html) is also available for doing this, but `cxx` generates _safer_ C++ bindings by restricting the kinds of code generated. Its major drawback is that async Rust is not yet supported, but there are workarounds described in the docs.

## License

GPL-3.0.
