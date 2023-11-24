One of SoulsyHUD's design goals was to be easily themeable. If I got it right, you should be able to design a HUD layout that complements any UI theme, using its fonts, icons, and colors. This article discusses how you can change images and fonts to theme the HUD beyond how it lays out items on the screen. There's a separate full article on [what you can do with layout files](./article-layouts-v2.md).

Here's what Soulsy pulls together to draw a HUD on the screen:

- a layout file describing where to draw the HUD elements, how large they are, and so on
- a a `.ttf` TrueType font file containing the typeface to use for text
- background images for the whole hud, for slots, and hotkeys, in `svg` format
- images for hotkeys (PC keyboard shortcuts, PS5 controller buttons, Xbox controller buttons)
- images for some or all of its icons also in `svg` format

SoulsyHUD provides decent defaults for all of these. However, all of these elements can be replaced by a theme mod. If you're integrating the HUD with an overall UI makeover, like Untarnished or Nordic, you'll want to replace some or all of these things.

## Fonts

You can use any Truetype font to render text in the HUD. If the font supports it, you can generate glyphs for character sets beyond the usual Western alphabet glyphs. The HUD should be able to render any valid [UTF-8 character](https://www.utf8.com) if the font includes glyphs for that character.

⚠️ There are some characters the game menus can display that are invalid UTF-8 characters. Skyrim's Flash menus support an older text encoding called [UCS-2](https://en.wikipedia.org/wiki/Universal_Coded_Character_Set), and some characters in that encoding are not converted properly to UTF-8. I have encountered two mods that have item names that can't be represented properly. Fixing this bug is on my list for post-1.0. See [the ucs2-rs library](https://lib.rs/crates/ucs2). Even simple text is nothing but simple, it turns out.

Put the `.ttf` file in `SKSE/plugins/resources/fonts` and name it in the layout. The `font_size` option specifies what size to generate Imgui font billboard data. Text will look best when rendered at this size, so make this match whatever size most of your HUD text is.

Here's the full set of font options as they'd appear in a layout file:

```toml
font = "ComicSans.ttf"
font_size = 20.0
chinese_full_glyphs = false
simplified_chinese_glyphs = true
cyrillic_glyphs = true
japanese_glyphs = false
korean_glyphs = false
thai_glyphs = false
vietnamese_glyphs = false
```

## Images

All of the images used by Soulsy are in [Scalable Vector Graphics](https://www.w3.org/Graphics/SVG/) or `SVG` format, to keep their size in mod archives small. SoulsyHUD uses the [resvg](https://lib.rs/crates/resvg) Rust library to parse and render svgs. This library supports nearly all SVG features aside from animation. You can use images with color and alpha gradients, for example.

The SoulsyHUD icon pack sources its svgs from [The Noun Project](https://thenounproject.com/icons/), paying for the right to redistribute them without attribution. Other free icon sources include [SVGRepo](https://www.svgrepo.com) and [Flaticon](https://www.flaticon.com). You can also export svg from vector drawing apps. If you are using a UI mod for Skyrim, you can export its images in svg format using an app like [the JPEX Flash decompiler](https://github.com/jindrapetrik/jpexs-decompiler). (If you do this, make sure you have permission to use the assets! You can always do this for your own private use, however.)

### Background images

[SoulsyHUD v2 layouts](./article-layouts-v2.md) can specify what file to use for any background image, independently for each slot. These background images are loaded from the `SKSE/plugins/resources/backgrounds` directory of the theme mod. If the file is not found, no image is drawn.

SoulsyHUD comes with some black and white background images named `hud_bg.svg`, `slot_bg.svg`, and `key_bg.svg`. You can rely on those files being present if your theme has no special needs here. They were, however, designed to look good with an understated black and white theme like Untarnished, so most themed layouts will want to provide their own images.

### Controller shortcuts & keys

All of these images can be replaced if you wish. Look at the files in the directory  `SKSE/plugins/resources/buttons` in the core mod to see what you can change.

### Icons

All of the icons SoulsyHUD uses can be replaced. Your theme mod should put its replacement icons into the directory `SKSE/plugins/resources/icons`. You *must* name your icons the way the HUD expects. The names follow a convention that I hope is predictable and understandable-- the names mostly mention obvious game concepts.

✨ I am open to adding more icons. I do need to support each icon in code, because the mod does some work to assign icons to in-game items on the fly, and it has to know what the available icons are. If you have an icon you'd like me to add, please do message me on the Nexus about it, or make a GitHub PR to the mod if you prefer. I'll need to know what game concept it represents, so I can figure out which OCF keywords I can use to assign the icon appropriately. Soulsy already distributes some keywords itself to help classify items, so I'm open to adding those if needed as well.

This is the set of core icons any theme should replace:

```text
alteration.svg
ammo_arrow.svg
armor_clothing.svg
armor_heavy.svg
armor_light.svg
armor_mask.svg
armor_shield_heavy.svg
conjuration.svg
destruction.svg
food.svg
hand_to_hand.svg
icon_default.svg
illusion.svg
misc_lantern.svg
misc_torch.svg
potion_default.svg
potion_health.svg
potion_magicka.svg
potion_poison.svg
potion_resist_fire.svg
potion_resist_frost.svg
potion_resist_magic.svg
potion_resist_shock.svg
potion_stamina.svg
power.svg
restoration.svg
scroll.svg
shout.svg
spell_default.svg
spell_fire.svg
spell_frost.svg
spell_shock.svg
weapon_axe_one_handed.svg
weapon_axe_two_handed.svg
weapon_bow.svg
weapon_claw.svg
weapon_crossbow.svg
weapon_dagger.svg
weapon_halberd.svg
weapon_katana.svg
weapon_mace.svg
weapon_pike.svg
weapon_quarterstaff.svg
weapon_rapier.svg
weapon_staff.svg
weapon_sword_one_handed.svg
weapon_sword_two_handed.svg
weapon_whip.svg
```

The list of *additional* icon images in the SoulsyHUD icon pack follows. These are optional to provide, but will make the HUD displays more useful by displaying icons for mod-added weapon types.

```text
ammo_bullet.svg
armor_amulet.svg
armor_backpack.svg
armor_belt.svg
armor_bracelet.svg
armor_circlet.svg
armor_cloak.svg
armor_clothing_feet.svg
armor_clothing_hands.svg
armor_clothing_head.svg
armor_earring.svg
armor_heavy_feet.svg
armor_heavy_hands.svg
armor_heavy_head.svg
armor_light_feet.svg
armor_light_hands.svg
armor_light_head.svg
armor_quiver.svg
armor_ring.svg
armor_robes.svg
armor_shield_light.svg
drink_mead.svg
drink_tea.svg
drink_water.svg
drink_wine.svg
food_bread.svg
food_carrot.svg
food_cheese.svg
food_fish.svg
food_meat.svg
food_pie.svg
food_stew.svg
misc_campfire.svg
misc_lute.svg
misc_tent.svg
potion_resist.svg
potion_skooma.svg
power_craft.svg
power_fill_bottles.svg
power_horse.svg
power_peek.svg
power_pray.svg
power_wash.svg
power_werebear.svg
power_werewolf.svg
shout_animal_allegiance.svg
shout_breath_attack.svg
shout_call_dragon.svg
shout_clear_skies.svg
shout_cyclone.svg
shout_ice_form.svg
shout_marked_for_death.svg
shout_stormcall.svg
spell_arclight.svg
spell_bleed.svg
spell_circle.svg
spell_constellation.svg
spell_control.svg
spell_cure.svg
spell_desecration.svg
spell_detect.svg
spell_eagle_eye.svg
spell_earth.svg
spell_elemental_fury.svg
spell_evade.svg
spell_fear.svg
spell_feather.svg
spell_heal.svg
spell_holy.svg
spell_leaf.svg
spell_leaves.svg
spell_light.svg
spell_lightning_blast.svg
spell_moon.svg
spell_paralyze.svg
spell_poison.svg
spell_reanimate.svg
spell_reflect.svg
spell_root.svg
spell_rune.svg
spell_shadow.svg
spell_sharpen.svg
spell_silence.svg
spell_slow.svg
spell_soultrap.svg
spell_sprint.svg
spell_stamina.svg
spell_stars.svg
spell_summon.svg
spell_sun.svg
spell_teleport.svg
spell_time.svg
spell_vampire.svg
spell_ward.svg
spell_water.svg
spell_wind.svg
spell_wisp.svg
tool_fishing_rod.svg
tool_pickaxe.svg
tool_shovel.svg
tool_sickle.svg
weapon_flail.svg
weapon_grenade.svg
weapon_gun.svg
weapon_hammer.svg
weapon_lance.svg
weapon_scythe.svg
weapon_wood_axe.svg
```
