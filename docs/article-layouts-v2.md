# Layouts 2: Electric Boogaloo

SoulsyHUD now has a second-generation layout schema you can use instead of the earlier schema. (The first schema is still supported and will draw correctly, but it won't get any new features.)

## How to edit layouts

SoulsyHUD's layouts are described in TOML text files. TOML is an acronym for "Tom's Obvious Markup Language". It was designed to be better than INI files at describing configuration, less bug-prone than YAML, and more human-friendly than JSON. You can read more about it on [its official webpage](https://toml.io/).

Here's the fast version for basic edits, like changing the position or size of a layout:

1. Start with a layout that is close to what you want already.
1. Launch the game with this layout enabled.
1. Go into the SoulsyHUD MCM and make sure you have a hotkey bound for "refresh layout".
1. Tab out of the game.
1. Open the file `Skyrim Special Edition/data/SKSE/plugins/SoulsyHUD_Layout.toml` with a text editor like Notepad or [VSCode](https://code.visualstudio.com). The layout has comments explaining what each item means.
1. Make some changes. Save the file. A good starting change is to move the anchor point of the HUD to change where it's drawn.
1. Tab back into the game. Press the refresh key. You should see your changes!
1. If your changes don't take effect and you see a warning message on the screen, look at the log file for the mod. The layout needs to be valid TOML, so if you forget to close a quote the mod will log the error and leave your layout unchanged.
1. Edit until you're happy with where things are, then enjoy.

The rest of this document goes into deep detail about what all the pieces of a layout are, what's optional and what's required, and what the numbers mean.

## Layout overview

All layouts have seven (optionally eight) sections.

- required top-level fields; these must come *first*
- an optional background image for the entire HUD
- the five required slot layouts plus the optional equipset layout, in any order

The top-level fields describe things that apply to all parts of the layout, like the font used to draw text, and the HUD's location. The slot layouts all follow the slot layout template. Each of these uses some common building blocks. For instance, they all specify their background images the same way.

We're going to cover the building blocks first, then start putting them together.

## Building blocks

### Points

The layout uses a *point* to define a location on the screen and the size of something. (These are strictly speaking different things, but they're close enough that I got lazy and used it for both.) Points have a `x` and `y` coordinates and look like this in TOML:

```toml
offset = { x = 150.0, y = -50.0 }
size = { x = 300.0, y = 300.0}
```

The `x` is the width or the side-to-side offset. Negative values move left. `y` is height or top-to-bottom offset. Negative values move up.

### Colors

Text and images can be drawn with colors. Colors are described with red, green, blue, and alpha numbers, abbreviated to the first letter of each to keep things compact. The lowest possible value is 0. The highest is 255. Black would have `r`, `g`, and `b` all 0. White would have `r`, `g`, and `b` all 255. The alpha value controls how transparent a color is. 0 is completely transparent aka invisible; 255 is not transparent at all. SoulsyHUD skips drawing anything with alpha 0 to avoid doing useless work when rendering the HUD.

Here's a field describing a half-transparent orange-yellow color:

```toml
color = { r = 223, g = 188, b = 32, a = 128 }
```

Order does not matter, but color fields are usually mentioned in this order.

### Background image elements

Background images use a point and a color

```toml
[background]
# This is the file name of an SVG that must be in resources/backgrounds.
svg = "hud_bg.svg"
# The size to draw the image.
size = { x = 300.0, y = 300.0}
# The color to use to draw this image.
color = { r = 0, g = 0, b = 0, a = 64 }
```


### Icon elements

### Text elements

Each layout has an array (or list) of *text* elements. These describe text that should be drawn in the slot.

- `offset`: Where to draw this text, relative to the center of the slot
- `alignment`: How to justify the text. Possible values are `left`, `center`, and `right`.
- `font_size`: A floating-point number for the size of the type used.
- `color`: The color to use to draw the text.
- `contents`: A format string describing the text to draw.

The data that can be filled into a format string is:

- `{name}`: the item's name
- `{count}`: how many of the item the player has
- `{kind}`: the item's category
- any regular text you'd like

Some examples of format strings:

- `ITEM: {name}`
- `{name}: {count}`
- `outfit`

You can have as many text elements as you need. For instance, you might display the item count in one location and name in another, or you might combine them into a single display.

Here's a full text element:

```toml
[[power.text]]
offset = { x = 110.0, y = 110.0 }
color = { r = 255, g = 255, b = 255, a = 255 }
alignment = "left"
contents = "{name}"
font_size = 20.0
```

Any additional text elements for the power slot will also be named `[[power.text]]`. The double square brackets tells TOML that this is an array of items.

## Slot elements

There are six slots you can describe in a layout. All of them except the `equipset` slot are *required*.

- `power`: The shout or minor power currently ready for use.
- `utility`: The utility or consumable item ready to be activated.
- `left`: What's currently equipped in the player's left hand.
- `right`: What's currently equipped in the player's right hand.
- `ammo`: What ammo the player has equipped.
- `equipset`: The currently-worn equipment set.

Each slot has the following sub-elements:

- an `offset` field, describing where to draw this slot
- a required `icon` element, named `[slotname.icon]`
- an optional `background` element, named `[slotname.background]`
- an optional `hotkey` element, named `[slotname.hotkey]`
- a list of text elements, in the array `[[slotname.text]]`

An example slot layout with all of the elements:

```toml
[right]
offset = { x = 83.0, y = 0.0 }

[right.background]
svg = "slot_bg.svg"
size = { x = 100.0 , y = 100.0 }
color = { r = 255, g = 255, b = 255, a = 128 }

[right.icon]
color = { r = 200, g = 200, b = 200, a = 255 }
size = { x = 43.0, y = 43.0 }
offset = { x = 0.0, y = 0.0 }

[right.hotkey]
color = { r = 255, g = 255, b = 255, a = 255 }
offset = { x = -60.0, y = 0.0 }
size = { x = 30.0, y = 30.0 }

[right.hotkey.background]
svg = "key_bg.svg"
size = { x = 30.0 , y = 30.0 }
color = { r = 255, g = 255, b = 255, a = 128 }

[[right.text]]
color = { r = 255, g = 255, b = 255, a = 255 }
offset = { x = 10.0, y = 52.0 }
font_size = 20.0
alignment = "left"
contents = "{name}"
```

## Top-level fields

At last we come to the fields that must come first in the layout: the fields that affect the entire HUD. The order these fields are in does not matter, so long as they're present and come before any other element.

### `global_scale`: number

The `global_scale` field is a floating-point number that's used to scale the entire HUD larger or smaller. Change this value if the layout you're using looks too large or too small for you. Layout authors should probably ship with this set to 1.0.

### `anchor_name`: text

The `anchor_name` field specifies one of several known anchor points for the mod. You can use this anchor to place the HUD in a common location regardless of what screen resolution the player has. `top_right` will always mean top right. The possible values are:

- bottom_left
- bottom_right
- top_left
- top_right
- center
- center_top
- center_bottom
- left_center
- right_center

You can also specify the location of the HUD using x and y coordinates directly:

```toml
anchor = { x = 2100.0, y = 825.0 }
```

If you include both fields, the named anchor is used.

### `size`: point

The `size` field is a hint about the size of the HUD. This is used to center the HUD on its named anchor point.

### ammo and left swap options

These are two boolean fields that control when the ammo and left hand slots are visible. They can be set independently because I was unable to make up my mind, but together they make a full feature for swapping an ammo view into the spot where the left hand is normally drawn. It's up to you to position the two slots in the same location if you want to do this!

If you want to swap the two when the player equips a ranged weapon:

```toml
hide_ammo_when_irrelevant = true
hide_left_when_irrelevant = true
```

If you want to hide the ammo slot when it doesn't matter but leave the empty left hand visible:

```toml
hide_ammo_when_irrelevant = true
hide_left_when_irrelevant = false
```

If you want to show both slots all the time, set both to false.

### Fonts

A set of font options. Please see [the theming docs](./article-theming.md) for more on fonts.

### Example

Here is a complete example of the top-level fields:

```toml
global_scale = 1.0
anchor_name = "bottom_left"
# anchor = { x = 2100.0, y = 825.0 }
size = { x = 300.0, y = 300.0 }
hide_ammo_when_irrelevant = true
hide_left_when_irrelevant = true
font = "Inter-Medium.ttf"
font_size = 20.0
chinese_full_glyphs = false
simplified_chinese_glyphs = true
cyrillic_glyphs = true
japanese_glyphs = false
korean_glyphs = false
thai_glyphs = false
vietnamese_glyphs = false
```

## The full layout

Let's put all of these elements together into a full layout! As a reminder, these are the pieces we need:

- required top-level fields; these must come *first*
- an optional background image for the entire HUD
- the five required slot layouts plus the optional equipset layout, in any order

Here's what we might end up with for a layout that starts in the top right corner of the screen and runs down the right edge:

```toml
global_scale = 1.0
anchor_name = "top_right"
# anchor = { x = 2100.0, y = 150.0 }
size = { x = 100.0, y = 350.0 }
hide_ammo_when_irrelevant = true
hide_left_when_irrelevant = true
font = "Iosevka-Medium.ttf"
font_size = 20.0
chinese_full_glyphs = false
simplified_chinese_glyphs = false
cyrillic_glyphs = true
japanese_glyphs = false
korean_glyphs = false
thai_glyphs = false
vietnamese_glyphs = false

[background]
svg = "sleek_bg.svg"
size = { x = 100.0, y = 400.0 }
color = { r = 255, g = 255, b = 255, a = 128 }

[power]
offset = { x = -37.0, y = 0.0 }
# etc

[utility]
offset = { x = -37.0, y = 75.0 }
# etc

[left]
offset = { x = -37.0, y = 150.0 }
[left.icon]
color = { r = 200, g = 200, b = 200, a = 200 }
size = { x = 50.0, y = 50.0 }
offset = { x = 0.0, y = 0.0 }
[left.hotkey]
color = { r = 255, g = 255, b = 255, a = 255 }
offset = { x = 25.0, y = 25.0 }
size = { x = 30.0, y = 30.0 }
[[left.text]]
color = { r = 255, g = 255, b = 255, a = 255 }
offset = { x = -37.0, y = 37.0 }
font_size = 20.0
alignment = "right"
contents = "{name}"

[ammo]
offset = { x = -37.0, y = 150.0 }
[ammo.icon]
color = { r = 200, g = 200, b = 200, a = 200 }
size = { x = 50.0, y = 50.0 }
offset = { x = 0.0, y = 0.0 }
[ammo.hotkey]
color = { r = 255, g = 255, b = 255, a = 255 }
offset = { x = 25.0, y = 25.0 }
size = { x = 30.0, y = 30.0 }
[[ammo.text]]
color = { r = 255, g = 255, b = 255, a = 255 }
offset = { x = -37.0, y = 37.0 }
font_size = 20.0
alignment = "right"
contents = "{name}: {count}"

[right]
offset = { x = -37.0, y = 225.0 }
# etc

[equipset]
offset = { x = -37.0, y = 300.0 }
# etc
```

## Appendix

Pointless trivia! Did you know that this is not what Soulsy uses internally to draw the HUD? It translates both this format and the earlier format into a third format that makes it easier and faster to draw the HUD on every tick. The HUD-drawing code must perform well or you'll notice it sapping your framerate, because of how frequently it draws.
