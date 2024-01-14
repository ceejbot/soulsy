SoulsyHUD is a both a HUD mod and a hotkeys mod. SoulsyHud shows you what you have equipped right now at all times, updating in response to changes from any source. (CGO grip changes, for example!) You can set up lists of items you use often and cycle through them with a tap of a hotkey. You can also cycle through entire armor sets with a single key tap. Where possible, SoulsyHUD uses the game's existing menus to let you set all this up.

Soulsy's design goals are to stay *fast* and snappy with its interactions; to support as many mods as possible using keywords from [Object Categorization Framework](https://www.nexusmods.com/skyrimspecialedition/mods/81469), and to keep its user interface out of the way.

SoulsyHUD lets you set hotkeys for managing what you have equipped or readied in four equipment slots:

- right hand: weapons, spells, two-handed weapons, scrolls
- left hand: one-handed weapons, spells, shields, torches, lanterns
- shouts and minor powers, scrolls
- utility slot: potions, poisons, food, armor
- equipment sets: swap from one armor set to another

Soulsy sets up __cycles__ for each of these equipment slots. For example, if you want to switch between Flames and Healing spells in your left hand, you'd add each of them to your left hand cycle. For your right hand, you might set up a long sword with an anti-undead  enchantment, a dagger with Soul Trap, and a bow. Pressing the key assigned to a slot moves to the next item in your cycle and equips it (or readies it, in the case of the utility slot). If you tap the key several times quickly, you'll advance through the cycle and then equip the item you were on when you stopped tapping the button.

Here's what it looks like in action:

[youtube]nNhgXxlYdXA[/youtube]

To add or remove an item from a cycle, bring up the inventory, magic or favorites menu, hover over the item, and press the hotkey for the cycle you want to change. If the item is not in the cycle for that slot and it's appropriate for the slot, it'll be added. If it's already in that cycle, it'll be removed. Soulsy prints text feedback on the screen about what it did.

Soulsy also has a hotkey for activating your selected utility item. This is the only category of item that Soulsy will try to activate for you; everything else needs to used the same way the base game has you use them. The last hotkey-able shortcut is for hiding and showing the HUD. There is an MCM setting if you want the HUD to fade out when you're not in combat or don't have your weapons readied.

Soulsy's goal is to equip what you tell it to equip, as quickly and reliably as it can.

## Set up hotkeys first

SoulsyHUD has an MCM menu that allows you to edit its options and, most importantly, set up your hotkeys for it. Before you dive into gameplay, you'll want to set up these five hotkeys:

- cycling through shouts and powers
- cycling through items for your left hand
- cycling through items for your right hand
- cycling through consumables
- using a consumable

You can use a hotkey, a modifier + a hotkey, or a long-press to do most actions.

__Important notes for controller users:__ A natural setup for HUD mods like this is to use the dpad keys to cycle. You can't do that out of the box with the game's control maps. I suggest you use a [souls-like control map](https://www.nexusmods.com/skyrimspecialedition/mods/44160). You can find many premade examples of control maps on the Nexus if you don't want to edit one yourself. There is also a web app to [edit control maps](https://hawk.bar/SkyrimControlMapper/). Upload your current control map, edit, then copy-and-paste the result.

⚠️ If you have upgraded to game version 1.6.1130, you *must* use a control map updated for that version. Bethesda added new things to control maps and older files missing the new items will crash the game.

## Other options

Thanks to user requests, SoulsyHUD now has a lot of optional features to support your gameplay beyond just changing what you have readied. Explore the MCM to see all of them. Here's a fast overview:

- favoriting & unfavoriting an item to add and remove it from appropriate cycles
- a window of time to keep tapping a cycle button before an item gets equipped
- two different ways to support unarmed combat (adding unarmed as a cycle entry; long-press to unequip a slot)
- slow-motion if you're cycling items in combat; you can change how slow
- grouping health, magicka, and stamina potions into a single smart potion
- colorizing icons using Object Compatibility Framework's color suggestions plus other item information
- auto-hiding the HUD if you're not in combat or don't have your weapons drawn
- cycling through ammo with the left hand when you have a bow or crossbow equipped

For a detailed walk-through of the config settings, see the [configuration options](https://www.nexusmods.com/skyrimspecialedition/articles/5634) article.

## Theming

Soulsy does not have any in-game way to edit the UI. However, almost everything about how it looks can be modified by tabbing out and editing a layout file. Use any text editor to change values in `SoulsyHUD_Layout.toml`, save the file, then press the refresh hotkey in-game. The HUD reads your new layout and redraws itself.

You can change almost every aspect of the HUD, and edit the locations and appearance of each HUD slot independently. You can also replace the entire icon set used by Soulsy if you want. See the optional layouts and icon packs for examples.

If you read the [theming article](https://www.nexusmods.com/skyrimspecialedition/articles/5633) for more details on how to change layouts to match your game UI's theme.

More icons! Look at the requirements section on the mod description page to find more layouts and even some really nice icon packs. I want to call out [Komegaki's weapon icons](https://www.nexusmods.com/skyrimspecialedition/mods/106432), which you can use to make your menus, Wheeler, and Soulsy use the same hand-crafted icon set.

## Bug reports and feature requests

The comments have sticky posts with future plans for a few more features and some polish work before Soulsy reaches 1.0. If there's something you'd really like it to do, please leave a comment! Most Soulsy features have been added because somebody asked for them.

If you are experiencing trouble with Soulsy, such as crashes or weird behavior, please do post a comment here. When you are reporting a crash, *please* include a link to a crashlog uploaded to pastebin. Please don't post crashlogs in comments-- they're too hard to read there. Use [Crash Logger SSE AE VR - PDB support](https://www.nexusmods.com/skyrimspecialedition/mods/59818) to capture logs. It works on all versions of Skyrim and links the crash to exact lines in the plugin source. Crashlogs are incredibly helpful to me in debugging crashes, though, so I'll take them any way you can give them to me. If you have other problems with the mod, you can look at its logs in the SKSE logs directory to see if anything helpful shows up. The logs are designed to be read by humans.

You can also file an issue on the GitHub repo if that's easier for you. I usually end up filing them there myself if I don't fix them immediately.

## Credits

I owe infinite thanks to [mlthelama](https://www.nexusmods.com/skyrimspecialedition/users/5190780) and [LamasTinyHud](https://www.nexusmods.com/skyrimspecialedition/mods/82545) for giving me a turbo-boost with this project. I learned everything I know about writing SKSE plugins and using CommonLibSSE-NG by reading the source for LamasTinyHud. ❤️

The optional icon pack is licensed to me from The Noun Project. I've licensed it for use without attribution, but I am giving attribution anyway because it's an excellent set. It is the [Role-Playing Game icon set](https://thenounproject.com/browse/collection-icon/role-playing-game-70773/?p=1) by MaxIcons.

The *THICC icon pack* is used with permission from [The Handy Icon Collection Collective](https://www.nexusmods.com/skyrimspecialedition/mods/90508).

One fun fact about SoulsyHUD is that it is about 60% written in the [Rust programming language](https://www.rust-lang.org), and every day I move a little bit more over to Rust. This was a snap thanks to the [CXX crate](https://cxx.rs/), which makes implementing interfaces between C++ and Rust pleasant for lovers of each language. If you're curious how this works, [SoulsyHUD source is available on GitHub](https://github.com/ceejbot/soulsy) under the GPL-3.0 license. Just as __mlthelama__ generously allowed me to fork LamasTinyHud, you have permission in advance to fork SoulsyHUD and do what you'd like with it, so long as you also share your source via the GPL. Permissions are open. Credit would be lovely, and remember to tip your hat to __mlthelama__ as well.

## Support the author

If you enjoy this mod, please do buy me a coffee. I am a machine for turning coffee into code.

[![ceejbot on ko-fi](https://storage.ko-fi.com/cdn/kofi2.png)](https://ko-fi.com/ceejbot)
