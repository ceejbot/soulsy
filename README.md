# Soulsy

Soulsy is a minimal-features Souls-style hotkey HUD for Skyrim SE and AE. It is inspired by hotkey mods like Elden Equip, iEquip, and LamasTinyHud. It is in fact a fork of [LamasTinyHud](https://github.com/mlthelama/LamasTinyHUD)! It is simpler than LamasTinyHud is, however.

![Screenshot of the hud](./docs/SoulsyHUD_preview.jpeg)

Check out the remarkably terse [user docs](./docs/). Or take a peek at a [this tour of the HUD](https://youtu.be/4Y2lpa-GcCA). If you like it, you can download it for your favorite mod manager [from NexusMods](https://www.nexusmods.com/skyrimspecialedition/mods/96210/).

## Development goals

My goals are two-fold: make a Souls-style equip HUD that is exactly what I want to use, and learn how to do Rust FFI. A bonus is demonstrating how to write Skyrim native-code mods in Rust.

This project has been released and is in active use. My eventual goal is to move everything except the SKSE plugin glue code to Rust, and have the C++ mostly vanish. See the TODO list at the end of this readme for details about my next steps.

## Building

Soulsy is a Rust and C++ project, using CMake to drive Cargo to build the Rust parts. The application logic is implemented in Rust, with a bridge to the C++ libraries required to implement an SKSE plugin. It requires the following to build:

- [Rust](https://rustup.rs) set up for Windows (not for WSL)
- [Visual Studio 2022](https://visualstudio.microsoft.com) with C++ compilers installed
- [CMake](https://cmake.org)
- [vcpkg](https://github.com/microsoft/vcpkg) with `VCPKG_ROOT` set in a user environment variable

The plugin requires the following vcpkg libraries, which will be installed for you:

- [CommonLibSSE-NG](https://github.com/CharmedBaryon/CommonLibSSE-NG)
- [spdlog](https://github.com/gabime/spdlog)
- [simpleini](https://github.com/brofield/simpleini)
- [nanosvg](https://github.com/memononen/nanosvg)
- [imgui](https://github.com/ocornut/imgui)

There are a number of development conveniences in the [justfile](https://just.systems), including build and archive recipes for Powershell. `cargo install just` if you do not have it. Because I am more comfortable on Unixes than on Windows, some recipes are written in Bash.
The just recipes can build, copy to a test mod directory, update version
numbers and tag a new release, and build archives for upload to the Nexus.

`cargo --doc open` displays programmer documentation for the Rust side of the plugin. The C++ side is commented, but not to the same degree.

You are absolutely invited to contribute. This project follows the standard [Contributor's Covenant](./CODE_OF_CONDUCT.md).

## Credits

I could not have approached the rendering code without the work in [LamasTinyHud](https://www.nexusmods.com/skyrimspecialedition/mods/82545), so [mlthelama](https://github.com/mlthelama) gets all the props. I also learned a lot about how to make an SKSE plugin by reading their source. Give that HUD a try if you don't like the souls-game style, or want a UI you can edit in-game. The original has more features than this one does! It's also the only hotkeys hud mod I tried that worked well in my game, so that's a testimonial.

The icons for the built-in theme are the usual SkyUI icons, plus the `futura-book-bt` true-type font. The background assets were built from scratch but were inspired by the [Untarnished UI skin](https://www.nexusmods.com/skyrimspecialedition/mods/82545) for LamasTinyHUD by [MinhazMurks](https://www.nexusmods.com/skyrimspecialedition/users/26341279). The icons are the SkyUI icons by psychosteve, which are used in so many places I am not sure how to credit them.

The icons for the Ceej remix layout are licensed to me from the Noun Project for use without attribution, but I am going to give attribution anyway because they're great icons. I am using the [Role Playing Game collection](https://thenounproject.com/browse/collection-icon/role-playing-game-70773/?p=1) by [Maxicons](https://thenounproject.com/maxicons/).

The font in use for some layouts is [Inter](https://rsms.me/inter/).

[cxx](https://cxx.rs/) made developing the C++/Rust bridge a snap. This crate unlocks Rust as a viable language for all of your modding needs. The only drawback is that async Rust is not yet supported, but there are workarounds described in the docs.

## TODO

Current tasks:

- [ ] Make a *good-looking* layout. Find a designer if necessary.
- [ ] Fix filed issues.
- [ ] Move image loading code to Rust. This will bring in the [windows](https://lib.rs/crates/windows) crate ecosystem.
- [ ] Move `imgui` rendering to Rust. Bindings exist already, plus a DX11 rendering back end.
- [ ] Make image loading on-demand, to save memory. (Maybe an unimportant optimization? Measure.)
- [ ] Add support for debug builds to CMake, or at least remove the half-done option.
- [ ] Decide what to do about highlight animations.
- [ ] If I decide to highlight, track highlight status in the controller to support it.
- [x] I18n: fonts. ??
- [x] Hammer the hell out of it while playing. Fix whatever doesn't stand up to abuse.

## License

GPL-3.0.
