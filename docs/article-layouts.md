# Theming SoulsyHUD

Nearly every aspect of the HUD layout can be changed by editing the layout file. This article explains how to do that, starting with fast and easy edits to change the HUD location, then going into ways to make your own very different-looking layouts.

The optional files have a variety of layouts intended to show how varied they can be.

__Note:__ This article is [available on GitHub](https://github.com/ceejbot/soulsy/blob/latest/docs/article-layouts.md) with better formatting. You might find it easier to read there.

## Basics: how to edit a layout

Here's the fast version for basic edits, like changing the position or size of a layout:

1. Start with a layout that is close to what you want already.
1. Launch the game with this layout enabled.
1. Go into the SoulsyHUD MCM and make sure you have a hotkey bound for "refresh layout".
1. Tab out of the game.
1. Open the file `SKSE/plugins/SoulsyHUD_Layout.toml` with a text editor like Notepad or [VSCode](https://code.visualstudio.com). The layout is in TOML format, which is like INI but more structured. It should have comments explaining what each item means.
1. Make some changes. Save the file. A good starting change is to move the anchor point of the HUD to change where it's drawn.
1. Tab back into the game. Press the refresh key. You should see your changes!
1. If your changes don't take effect, look at the log file for the mod. The layout needs to be valid TOML, so if you forget to close a quote the mod will log the error and leave your layout unchanged.
1. Edit until you're happy with where things are, then enjoy.

## Global layout settings

The opening section of the layout file has settings that affect the layout as a whole: where it is, how big it is, and so on. This section walks through each of the values you can set and when you might want to change them. You can read the source for [the default layout online on GitHub](https://github.com/ceejbot/soulsy/blob/latest/data/SKSE/plugins/SoulsyHUD_Layout.toml).

### HUD position

Layouts offer two ways to say where the HUD should be drawn: a named anchor and an anchor location in pixels. The *center* of the HUD is drawn at the anchor location.

The easiest way to say where the HUD should be is to use the anchor name. Look for this line in the layout file:

```toml
anchor_name = "bottom_left"
```

These locations are calculated at runtime from your screen size. Valid anchor names are:

- `bottom_left`: bottom left screen corner
- `bottom_right`: bottom right screen corner
- `top_left`: top left corner
- `top_right`: top right corner
- `center`: the middle of the screen
- `center_top`: the middle of the top edge
- `center_bottom`: the middle of the bottom edge
- `left_center`: the middle of the left edge
- `right_center`: the middle of the right edge

If you want to put your HUD somewhere not along the screen edge or smack dab in the center, with more fine control, use the `anchor` attribute. It looks like this:

```toml
anchor = { x = 2000.0, y = 825.0 }
```

The `x` and `y` values are offsets from the top left corner. These values are not adapted to your screen resolution. Soulsy trusts you to know where you want your HUD.

### Global scale

You can control the size of the entire HUD by setting the global scale factor:

```toml
# Global scaling factor: the whole hud size changes when you change this.
global_scale = 1.0
```

One thing I sometimes do is get fussy about layout positioning using sizes much larger than are practical on-screen with a large scale, like 2.0 or 3.0, then bring it down to 1.0 or smaller for gameplay.

_All size and offset values from here down are affected by the global scale_. This includes font sizes as well as icon sizes.

### HUD background

The HUD can optionally have a background image that you can resize and set an alpha value for. The `size` and `bg_color` top-level options control this background.

```toml
# The size of the hud, before the global scaling factor is applied.
size = { x = 300.0, y = 300.0 }
# The color of the HUD background file. Set alpha to 0 to skip drawing a background.
bg_color = { r = 0, g = 0, b = 0, a = 64 }
```

Colors are expressed as red, green, blue, and alpha values between 0 and 255 inclusive.

### Ammo and the left hand

This feature is tricky to explain but easy to understand if you see it on-screen. Layouts can, if they choose, swap ammo in for the left-hand item when the player has a ranged weapon equipped. The default layout does this.

The feature has two parts that you can control separately. First, you can hide the ammo slot if it doesn't matter, such as when the player has spells equipped instead of a bow. Second, you can hide the left hand if a two-handed weapon is equipped. The game's point of view is that the player is holding one item with both hands, but Soulsy draws the left hand empty. You can avoid wasting screen space by making the left hand slot and the ammo slot be identical, and enabling both of these booleans. *When you enable this swapping*, the left hand cycle button rotates through ammo instead of left-hand items.

Here's how the default layout sets these values:

```toml
# Only draw the ammo slot when a weapon that uses ammo (ranged) is equipped.
hide_ammo_when_irrelevant = true
# Hide the left hand slot when a ranged weapon is equipped. This lets you
# use the same location as the left hand to show ammo.
hide_left_when_irrelevant = true
```

### Text and typefaces

Soulsy allows you to set which typeface to use for all HUD text. It ships with Futura by default, so it can match Untarnished UI, but you might want to change this to match the theme of your UI. To change the typeface, put a TrueType `.ttf` file into `SKSE/plugins/resources/fonts`. (Look for where the file `futura-book-bt.ttf` is!) Then change the line `font = "typeface_name.ttf` to point to your new file.

Another reason to change the typeface would be to enable character glyphs beyond the limited set of Western characters supported by Futura. Soulsy has an optional layout called "SoulsyHUD-i18n" that comes with the typeface [Inter](https://rsms.me/inter/), which supports many character sets. The I18N layout enables Simplified Chinese and Cyrillic glyphs by default, but Soulsy supports a number more. To enable a character set for your language, set the glyphs value for that language to `true`. If your language is not in the list, please comment and ask for it. If ImGui supports it, so can Soulsy.

Here's the full config for fonts:

```toml
font = "Inter-Medium.ttf"
chinese_full_glyphs = false
simplified_chinese_glyphs = true
cyrillic_glyphs = true
japanese_glyphs = false
korean_glyphs = false
thai_glyphs = false
vietnamese_glyphs = false
```

That's the end of the options you can set for the layout as a whole. Next the layout file contains five `[[layouts]]` sections, each of which describes a different HUD element.

Fun aside: `[[layouts]]` is the way TOML says that the layouts field is an array. Each section starting with that heading is an entry in the array. This is the worst thing about using TOML for the layouts. JSON is too human-unfriendly, however.

## Hud slots

The layouts use the term "slot" to refer to a single HUD element. There are five "slots", one for each cycle and a fifth to show ammo for ranged weapons:

- slot 0: shouts & powers
- slot 1: utility items and consumables
- slot 2: left hand items
- slot 3: right hand items
- slot 4: ammunition

Each slot can be sized and positioned independently from the others. Here's a full slot layout, with some comments to group related items:

```toml
[[layouts]]
name = "Consumables"
element = { repr = 1 }

# position
offset = { x = 0.0, y = 83.0 }

# bg image size and color
size = { x = 100.0 , y = 100.0 }
bg_color = { r = 255, g = 255, b = 255, a = 128 }

# text alignment for name & count
align_text = "left"

# icon
icon_color = { r = 200, g = 200, b = 200, a = 255 }
icon_size = { x = 43.0, y = 43.0 }
icon_offset = { x = 0.0, y=0.0 }

# hotkey
hotkey_color = { r = 255, g = 255, b = 255, a = 255 }
hotkey_offset = { x = 0.0, y = -60.0 }
hotkey_size = { x = 30.0, y = 30.0 }
hotkey_bg_color = { r = 0, g = 0, b = 0, a = 0 }

# item count text
count_color = { r = 200, g = 200, b = 200, a = 255 }
count_font_size = 20.0
count_offset = { x = 40.0, y = 20.0 }

# item name text
name_offset = { x = 65.0, y = 20.0 }
name_font_size = 20.0
name_color = { r = 255, g = 255, b = 255, a = 255 }
```

Let's walk through the fields in this slot together.

The `name` field is there to remind you which slot is which. It does not change anything about the layout. The `element` field is important to the software-- it tells the layout engine which slot is which. *Do not change the element field.* Everything else can be edited!

The `offset` is the location of the center of this hud element relative to the anchor point of the HUD itself. `size` is the size of any background image for the slot. `bg_color` is the color used to draw the background image. You are most likely to vary the _alpha_ or `a` value for this color, to control transparency. If the alpha is 0, the hud image is not drawn at all.

`align_text` sets how the name and count text for this slot is aligned. Valid choices here are `left`, `right`, and `center`.

If the item shown in this slot has a meaningful count-- say, a stack of potions or scrolls-- then a slot can show that count. `count_offset` says where to draw the count text. `count_font_size` says how large to draw it. `count_color` says what color to draw the text with. If alpha is zero, a count won't be drawn.

`name_offset`: Where to draw name text for the item shown in this slot. `name_font_size`: How large the font is, in points. `name_color`: What color to use. As with all other items, an alpha value of zero for this color means the name is not drawn at all.

The next four fields starting with `hotkey_` are, you guessed it, for drawing a hotkey reminder. If this slot has a hotkey associated with it, where and how should the hotkey reminder be drawn? The fields here are similar to all the others. You have rgba colors for the background image and the shortcut image, a size for the background image, and an offset relative to the center of the slot saying where to draw it.

The order the slot fields are in does not matter, so long as they're present. If you are missing a required field, you will not be able to load the layout file. You'll find details about what went wrong in the log file `SoulsyHUD_rust.log`.

Repeat this five times, once for each slot, and you have a full layout!

## Theming

```
SoulsyHUD/SKSE/plugins
├── resources
│  ├── backgrounds/*
│  ├── buttons/*
│  ├── fonts/*
│  └── icons/*
└── SoulsyHUD_Layout.toml
```

All images and icons must be SVG files.

### Icons

We've already discussed how you can change the font to match your UI theme, but you can also change *background images* and any or all *icons* to make major changes to the way the HUD looks.

Soulsy offers two optional icon packs to expand the original set of SkyUI icons. The THICC icon pack is used with permission from the [Object Categorization Framework](https://www.nexusmods.com/skyrimspecialedition/mods/81469) project and the [THICC](https://www.nexusmods.com/skyrimspecialedition/mods/90508) icons for [Inventory Interface Information Injector, aka I4](https://www.nexusmods.com/skyrimspecialedition/mods/85702). If you're using those mods to style your menus, you might want the HUD to match.

Or you could use the optional SoulsyHUD icon pack, which contains icons sourced by Soulsy's author from The Noun Project. Pick whichever you like! Or mix and match.

### Background images

The look of a HUD layout is mostly controlled by its background image and by the background images for each slot. Spread-out layouts like the centered layout have no background image at all-- this layout draws its slots spaced around the center of the screen. Others, like the default layout, establish their look with a clean white-on-black diamond shape. Other silly approaches are possible.

All background images must be in SVG format and in the `SKSE/plugins/resources/backgrounds/` directory. The background for the whole HUD is named `hud_bg.svg`. The background for a single slot is named `slot_bg.svg`. The background for a hotkey is `key_bg.svg`.

These images are rasterized and scaled at DLL load time, and can't be changed while the game is running.

When designing a new font layout, I use a vector graphics program like Affinity Designer to draw the shapes and figure out good positions, then I export svg from the program to use in the game.
