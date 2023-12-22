# Layouts 2: Electric Boogaloo

1. [Layouts 2: Electric Boogaloo](#layouts-2-electric-boogaloo)
   1. [How to edit layouts](#how-to-edit-layouts)
   2. [Layout overview](#layout-overview)
   3. [Building blocks](#building-blocks)
      1. [Points](#points)
      2. [Colors](#colors)
      3. [Image elements](#image-elements)
      4. [Icon elements](#icon-elements)
      5. [Text elements](#text-elements)
      6. [Poison indicators](#poison-indicators)
      7. [Meter elements](#meter-elements)
   4. [Slot elements](#slot-elements)
   5. [Top-level fields](#top-level-fields)
      1. [`global_scale`: number](#global_scale-number)
      2. [`anchor_name`: text](#anchor_name-text)
      3. [`size`: point](#size-point)
      4. [ammo and left swap options](#ammo-and-left-swap-options)
      5. [Fonts](#fonts)
      6. [Example](#example)
   6. [The full layout](#the-full-layout)
   7. [Examples](#examples)
   8. [Appendix](#appendix)

SoulsyHUD now has a second-generation layout schema you can use instead of the earlier schema. I encourage you to use this new layout approach-- it offers you more ways to customize layouts. (The first schema is still supported and will draw correctly, but it won't get any new features.)

## How to edit layouts

SoulsyHUD's layouts are described in TOML text files. TOML is an acronym for "Tom's Obvious Markup Language". It was designed to be better at describing configuration than INI, less bug-prone than YAML, and more human-friendly than JSON. You can read more about it on [its official webpage](https://toml.io/).

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

We'll go in order from simplest to most complex.

### Points

The layout uses a *point* to define a location on the screen and the size of something. (These are strictly speaking different things, but they're close enough that I got lazy and used it for both.) Points have a `x` and `y` coordinates and look like this in TOML:

```toml
offset = { x = 150.0, y = -50.0 }
size = { x = 300.0, y = 300.0}
```

The `x` is the width or the side-to-side offset. Negative values move left. `y` is height or top-to-bottom offset. Negative values move up.

### Colors

Text and images can be drawn with colors. Colors are described with red, green, blue, and alpha numbers, abbreviated to the first letter of each to keep things compact. The lowest possible value is 0. The highest is 255. Black would have `r`, `g`, and `b` all 0. White would have `r`, `g`, and `b` all 255. The alpha value controls how transparent a color is. 0 is completely transparent aka invisible; 255 is not transparent at all. SoulsyHUD skips drawing anything with alpha 0 to avoid doing useless work when rendering the HUD.

Here's a color setting describing a half-transparent orange-yellow color:

```toml
color = { r = 223, g = 188, b = 32, a = 128 }
```

Colors are conventionally given in rgba order, but the order doesn't matter.

### Image elements

Image elements appear as parts of slot layouts, the overall hud layout, and elements like the poison indicator. Images use a point to describe how large to draw the image, and a color to specify what color to draw it. The `svg` field names the image file to draw. The svg file is assumed to be in the directory `resources/backgrounds`, but you may point to an image in a different sub-folder of `resources` like this: `../icons/icon.svg`. Here's an example:

```toml
[background]
svg = "hud_bg.svg"
size = { x = 300.0, y = 300.0}
color = { r = 0, g = 0, b = 0, a = 64 }
```

### Icon elements

Every HUD slot has a required icon element. This element describes where to draw the icon for the item shown in that slot. Each icon element has a color, an offset, and a size. The offset field is relative to the center of the slot. To not offset something at all, use zeros.

Here's an example of a half-transparent gray icon that's not offset from the center of the slot:

```toml
[slotname.icon]
color = { r = 200, g = 200, b = 200, a = 128 }
size = { x = 43.0, y = 43.0 }
offset = { x = 0.0, y = 0.0 }
```

### Text elements

Each slot layout has a list of *text* elements. These describe text that should be drawn in the slot. You can have as many text elements as you need. For instance, you might display the item count in one location and name in another, or you might combine them into a single display. Each text element you add costs a little bit of time for each HUD draw (an addition measured in nanoseconds) so you won't want to add dozens of them.

Here are the fields a text element has:

- `offset`: Where to draw this text, relative to the center of the slot
- `alignment`: How to justify the text. Possible values are `left`, `center`, and `right`.
- `font_size`: A floating-point number for the size of the type used.
- `color`: The color to use to draw the text.
- `contents`: A format string describing the text to draw.

The data that can be filled into a format string is:

- `{name}`: the item's name
- `{count}`: how many of the item the player has
- `{charge}`: the charge or fuel level of the item, expressed as a percentage of the full charge
- any regular text you'd like

Some examples of valid format strings:

- `ITEM: {name}`
- `{name}: {count}`
- `{name}: {charge}%`
- `outfit`

Here's a full text element, which draws the name of the equipped shout or power for that slot:

```toml
[[power.text]]
offset = { x = 110.0, y = 110.0 }
color = { r = 255, g = 255, b = 255, a = 255 }
alignment = "left"
contents = "{name}"
font_size = 20.0
```

Any additional text elements for the power slot would also be named `[[power.text]]`. The double square brackets tells TOML that this is an [list of items](https://toml.io/en/v1.0.0#array-of-tables). Each new element named that is added to the end of the list.

### Poison indicator

Each layout slot can optionally include an indicator to show if an item is poisoned. This is only meaningful for left and right hands. Poison indicators are built from an offset plus an image element.

```toml
[left.poison]
offset = { x = 25.0, y = 20.0 }
[left.poison.indicator]
svg = "../icons/indicator_poison.svg"
color = {r = 160, g = 240, b = 2, a = 255 }
size = { x = 10.0, y = 10.0 }
```

### Meter elements

Slot layouts can optionally include a *meter* display, for graphically showing enchantment charge or torch burn time. The meaning of the meter depends on the item being shown, and SoulsyHUD does its best to guess what should be shown for an item. For example, a meter on the shouts and powers HUD slot would show shout cooldown time if that's relevant.

Right now SoulsyHUD supports three flavors of meters:

1. A rectangular meter built from two SVGs, one for the background and one to show fill level.
2. A rectangular meter built from one background SVG and a fill color.
3. Circular meters, with a background SVG, a fill color, and angles for 0% and 100%. You can draw a full circle around a slot or a partial arc. Elliptical curves aren't supported yet. (Sending cookies or coffee to the mod author might help with this feature request.)

Rectangular meters are drawn as _horizontal_ bars filling from left to right, then rotated by the angle you specify. All angles are given in degrees. 0° means no rotation. 90° is a vertical meter, with full being at the top. You can specify any degree of rotation you want: if your layout uses equilateral triangles, you can rotate a meter 60° to make it align with an edge.

- rectangular meters
- arc meters
- examples
- probably screenshots


Here's an example of a horizontal meter built from two svgs. The image for the fill uses a different color from the background:

```toml
[left.meter]
angle = 0 # horizontal
offset = { x = 0.0, y = -60.0 }
[left.meter.background]
# the svg for the background
svg = "meter_bar_empty.svg"
# the size for the background
size = { x = 100.0, y = 20.0 }
# the color for the background
empty_color = { r = 255, g = 255, b = 255, a = 255 }
[left.meter.fill]
# the svg drawn to show the fill
svg = "meter_bar_filled.svg"
size = { x = 98.0, y = 16.0 }
color = { r = 59, g = 106, b = 249, a = 200 }
```

Meters have performance costs that the mod author hasn't measured yet. Every additional thing you draw in a layout adds a little bit to the performance load.

## Slot elements

There are six slots you can describe in a layout. All of them except the `equipset` slot are *required*. These slots are:

- `power`: The shout or minor power currently ready for use.
- `utility`: The utility or consumable item ready to be activated.
- `left`: What's currently equipped in the player's left hand.
- `right`: What's currently equipped in the player's right hand.
- `ammo`: What ammo the player has equipped.
- `equipset`: The currently-worn equipment set; optional.

Each slot has the following sub-elements:

- a required `offset` field, describing where to draw this slot relative to the center of the HUD
- a required `icon` element, named `[slotname.icon]`
- an optional `background` element, named `[slotname.background]`
- an optional `hotkey` element, named `[slotname.hotkey]`
- an optional list of text elements, in the array `[[slotname.text]]`
- an optional poison indicator element, named `[slotname.poison]`
- an optional charge or fuel meter display, named [slotname.meter]

Because each slot specifies its own background element independent of the others, you can use a different background file for each slot. You might do this if your layout is asymmetrical or spread out on the screen. It's up to you!

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
offset = { x = 0.0, y = 0.0 } # relative to the center of the slot

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

[right.poison]
offset = { x = 0.0, y = -40.0 }
[left.poison.indicator]
svg = "../icons/indicator_poison.svg"
size = { x = 10.0, y = 10.0 }
color = {r = 160, g = 240, b = 2, a = 255 }

[right.meter]
angle = 0
offset = { x = 0.0, y = -60.0 }
size = { x = 100.0, y = 20.0 }
svg = "meter_bar_empty.svg"
empty_color = { r = 255, g = 255, b = 255, a = 255 }
fill_svg = "meter_bar_filled.svg"
fill_size = { x = 98.0, y = 16.0 }
fill_color = { r = 59, g = 106, b = 249, a = 200 }
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

If you include both an anchor name and an anchor point, the named anchor is used.

### `size`: point

The `size` field is a hint about the size of the HUD. This is used to place the HUD on its named anchor point without clipping on the edge of the player's display.

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

⚠️ These options are awkward. They might be replaced in the future with a single "swap left and ammo" option. If I do this, I'll support the old options until the mod reaches 1.0.

### Fonts

A set of font options. Please see [the theming docs](./article-theming.md) for more on fonts.

⚠️ Font options might be reorganized in the future. If I do this, I'll support the old options until the mod reaches 1.0.

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

[power]
offset = { x = -37.0, y = 0.0 }
# (not included to keep this readable)

[utility]
offset = { x = -37.0, y = 75.0 }
# (not included to keep this readable)

[right]
offset = { x = -37.0, y = 225.0 }
# etc

[equipset]
offset = { x = -37.0, y = 300.0 }
# etc
```

## Examples

The optional layout files you can download have examples of some of the things you can do with layouts. You can read them online on GitHub if you don't feel like downloading them. The [minimal layout](https://github.com/ceejbot/soulsy/blob/latest/layouts/SoulsyHUD_minimal.toml) shows how much you can leave out if you want to strip the HUD down to just its icons. The [centered layout](https://github.com/ceejbot/soulsy/blob/latest/layouts/SoulsyHUD_centered.toml) is an example of a layout that widely separates the HUD slots.

I am not a graphic designer but you might be-- please do something cool with these tools!

## Appendix

Pointless trivia! Did you know that this is not what Soulsy uses internally to draw the HUD? It translates both this layout format and the earlier format into a third one designed to make it easier to draw the HUD on every tick. The HUD-drawing code is performance-critical, as in, if it's slow, you'll notice it.
