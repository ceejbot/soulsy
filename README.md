# Soulsy

Soulsy is a minimal-features Souls-style hotkey HUD for Skyrim SE and AE. It is inspired by hotkey mods like Elden Equip, iEquip, and LamasTinyHud. It is in fact a fork of [LamasTinyHud](https://github.com/mlthelama/LamasTinyHUD)! It is simpler than LamasTinyHud is, however.

## Development goals

My goals are two-fold: make a Souls-style equip HUD that is exactly what I want to use, and learn how to do Rust FFI. A bonus is demonstrating how to write Skyrim native-code mods in Rust.

This project is still in development, though it does in fact run and act as
expected in-game! (Almost.) See the TODO list at the end of this README for details about its current status. My eventual goal is to move everything except the UI rendering and the plugin hooks/listeners to Rust, and have the C++ vanish down to glue code with CommonLibSE and the UI framework.

## User documentation

Soulsy lets you set hotkeys for managing what you have equipped or readied in four equipment slots:

- right hand: weapons, spells, two-handed weapons
- left hand: one-handed weapons, spells, shields, torches, lanterns
- shouts and minor powers
- a utility slot: potions, scrolls, poisons, food

Soulsy sets up _cycles_ for each of these equipment slots. For example, if you want to switch between Flames and Healing spells in your left hand, you'd add each of them to your left hand cycle. For your right hand, you might set up a long sword with an anti-undead enchantment, a dagger with Soul Trap, and a bow. Pressing the key assigned to a slot moves to the next item in your cycle and equips it (or readies it, in the case of the utility slot). If you tap the key several times quickly, you'll advance through the cycle and then equip the item you were on when you stopped tapping the button.

The maximum cycle length is configurable, but caps out at 20. 20 items is a lot of items to cycle through this way, and if you have that many you'd probably be better off using the inventory or favorites menu.

To add or remove an item from a cycle, bring up the inventory, magic or favorites menu, hover over the item, and press the hotkey for the cycle you want to change. If the item is not in the cycle for that slot and it's appropriate for the slot, it'll be added. If it's already in that cycle, it'll be removed. Soulsy prints text feedback on the screen about what it did.

Soulsy also has a hotkey for activating your selected utility item. This is the only category of item that Soulsy will try to activate for you; everything else needs to used the same way the base game has you use them. The last hotkey-able shortcut is for hiding and showing the HUD. There is an MCM setting if you want the HUD to fade out when you're not in combat or don't have your weapons readied.

That's it for the feature set. Soulsy does not (yet?) attempt to select the best ammo, potion or poison the way iEquip does. It equips what you tell it to equip, as quickly and reliably as it can. Soulsy also does not offer in-game layout editing, though you can modify the layout by editing a toml file outside the game. There's a refresh key that you can set and use to reload on the fly to
look at your changes.

### Settings options

TKTK: screenshot of MCM, explanation of defaults, etc

## How to theme the HUD

TKTKTK

The HUD look can be changed by modifying files in `SKSE/plugins`.

```text
SoulsyHUD/SKSE/plugins
├── resources
│  ├── animations/highlight/
│  ├── backgrounds/*
│  ├── buttons/*
│  ├── fonts/*
│  └── icons/*
└── SoulsyHUD_Layout.toml
```

- `SoulsyHud_Layout.toml` - The HUD layout, in TOML format. Set text colors and locations.
- `backgrounds/hud_bg.svg` - The background for the entire HUD.
- `backgrounds/slot_bg.svg` - The background for a single cycle element (left hand, power, etc).
- `backgrounds/key_bg.svg` - The background for hotkey hints.
- `animations/highlight` - An animation to play on a highlighted slot. NOT YET FUNCTIONAL.
- `SKSE/plugins/resources/buttons` - Xbox and Playstation button art.
- `SKSE/plugins/resources/fonts` - A TrueType font to use for display. The HUD comes with `futura-book-bt` to match Untarnished UI.
- `SKSE/plugins/resources/icons` - Icon files in SVG format, each named for the item. The HUD comes with the usual SkyUI icons.

## Building

Soulsy is a Rust and C++ project, using CMake to drive Cargo to build the Rust parts. The application logic is implemented in Rust, with a bridge to the C++ libraries required to implement an SKSE plugin. I have not attempted to build it anywhere other than on Windows. (The Rust side builds anywhere, but the C++ side does not.)

- [Rust](https://rustup.rs) set up for Windows (not for WSL)
- [Visual Studio 2022](https://visualstudio.microsoft.com) with C++ compilers installed
- [CMake](https://cmake.org)
- [vcpkg](https://github.com/microsoft/vcpkg) with `VCPKG_ROOT` set in a user environment variable

The plugin requires the following vcpkg libraries, which will be installed for you:

- [CommonLibSSE](https://github.com/powerof3/CommonLibSSE)
- [spdlog](https://github.com/gabime/spdlog)
- [simpleini](https://github.com/brofield/simpleini)
- [nanosvg](https://github.com/memononen/nanosvg) (for rastering the svgfiles)
- [imgui](https://github.com/ocornut/imgui) (for displaying ui elements)

There are a number of development conveniences in the [justfile](https://just.systems), including build and archive recipes for Powershell. `cargo install just` if you do not have it.

`cargo --doc open` displays programmer documentation for the Rust side of the plugin. The C++ side is commented, but not to the same degree.

## Credits

I could not have approached the rendering code without the work in [LamasTinyHud](https://www.nexusmods.com/skyrimspecialedition/mods/82545), so [mlthelama](https://github.com/mlthelama) gets all the props. I also learned a lot about how to make an SKSE plugin by reading their source. Give that HUD a try if you don't like the souls-game style, or want a UI you can edit in-game. The original has more features than this one does! It's also the only hotkeys hud mod I tried that worked well in my game, so that's a testimonial.

The icons for the built-in theme are the usual SkyUI icons, plus the `futura-book-bt` true-type font. The other layout data is adapted from the [Untarnished UI skin](https://www.nexusmods.com/skyrimspecialedition/mods/82545) for LamasTinyHUD by [MinhazMurks](https://www.nexusmods.com/skyrimspecialedition/users/26341279).

[cxx](https://cxx.rs/) made developing the C++/Rust bridge a snap. This crate unlocks Rust as a viable language for all of your modding needs. The only drawback is that async Rust is not yet supported, but there are workarounds described in the docs.

## TODO

Ceej's development to-do list:

- [x] Figure out how to compile papyrus scripts. Answer: PCA.
- [x] Edit the `.esp`` if necessary. Check it in.
- [x] Rewrite or merely just tweak the script that builds the mod archive itself, with correctly-placed files.
- [x] Test to see if the mod loads at all into the game. Fix whatever's broken.
- [x] Finish up the icon data loading function.
- [x] Hack out the per-page position settings stuff to ask Rust for info for exactly four slots, the ones visible right now.
- [x] Handle the case of equipped items not being in the cycle, while the cycle is being advanced.
- [x] Wire up the equip-item functions as well as the equip delay. Implement a timer using the tick in the imgui rendering code.
- [x] Implement a get-current-slot-info function that handles the case where the current item is not in a cycle.
- [x] Debounce keys. Especially the show/hide button.
- [x] Wire up the mod to MCM to show its config & write user settings.
- [x] Figure out what I'm doing wrong with MCM config settings. No really.
- [ ] Figure out what I'm doing wrong with translation files. UTF-16 LE, one tab. What else?
- [ ] Is there an official way to show a textual feedback message in SkyUI?
- [x] Make re-equipping the left-hand item work.
- [x] Wire up the inventory-changed hooks.
- [x] Inform Rust about equip changes.
- [x] Get ammo showing correctly.
- [x] Validate cycle data on save load. Baking the data into the save might be more robust long-term, but I don't know how to do that yet.
- [x] Wire up activating the utility button.
- [x] Figure out why activating potions makes the game lock up.
- [x] Get all layout info into one file; load it into the shared struct. (Is shared the right choice? who knows.)
- [x] Come up with an adequate default layout for the HUD.
- [ ] Make a *good-looking* layout. Find a designer if necessary.
- [ ] I18n: fonts.
- [x] I18n: translation files.
- [x] Code cleanup. DRY up the C++. Reorganize the Rust. Tighten up names.
- [ ] Review the 20-or-so TODO items noted in code comments.
- [x] Sort out `gear.h` vs `utility_items.h`. Merge?
- [x] Improve the CMake files so rebuilding is reliable.
- [ ] Add support for debug builds to CMake, or at least remove the half-done option.
- [ ] Hammer the hell out of it while playing. Fix whatever doesn't stand up to abuse.
- [ ] Consider getting more testers.
- [ ] Track highlight status in the controller to support animating a highlighted slot.
- [x] Make Rust log to a second file in the same directory as SKSE.
- [x] Add more Rust debug-level logging for happy-path cases.

Stretch goals:

- [ ] Move image loading code to Rust. This will bring in the [windows](https://lib.rs/crates/windows) crate ecosystem.
- [ ] Move imgui rendering to Rust. Bindings exist already, plus a DX11 rendering back end.
- [ ] Make image loading on-demand, to save memory. (Maybe an unimportant optimization? Measure.)
- [ ] Update to CommonLibSSE-NG?

## License

GPL-3.0.
