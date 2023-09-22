# MCM settings pages

This is a textual design document for the MCM settings design. This design is realized in the file [`config.json`](../../data/mcm/config/SoulsyHUD/config.json). Settings defaults are defined in `settings.ini` in the same directory. We design the menu pages here because the test/loop cycle is agonizing, and editing the json file is also painful.

## Page 1

### Cycling items

Shouts/powers shortcut: 2
Consumables/utilities: 5
Left hand: 4
Right hand: 6

### Adding and removing items

How do you want to add/remove cycle entries?
Dropdown, single choice:
- (x) Tapping the cycle key
- ( ) Long-pressing the cycle key
- ( ) The cycle key + a modifier
- ( ) Using a universal hotkey

Favoriting an item adds it to a cycle [ ]

### Consumables/Utilities

Activating utility / consumable:
- (x) Dedicated hotkey
- ( ) Long-press of utility cycle key
- ( ) Utility cycle key + modifier

If dedicated:
Activate utility key: 3

If modifier: show mod key selection

Smart grouped potions [ ]


### Ammo

How do you want to cycle through ammo?
- (x) Left-hand cycle key when ranged equipped
- ( ) Dedicated hotkey
- ( ) No ammo cycling

If dedicated:
Ammo cycle key: -1

### Gameplay feel

Equip delay ms: 750
Long-press ms: 1000
Slow time when cycling: [x]
Slow time percent: 25

### Unequipping

- (x) No support for unequipping
- ( ) Long-press of the relevant cycle key
- ( ) Add an unequipped item to left/right cycles
- ( ) Modifier key + cycle key

### Appearance

Colorize icons [x]
Controller buttons:
- (x) Playstation
- ( ) XBox

Autofade when not in combat: [x]
Fadeout duration ms: 1000

if autofade disabled:
Show/hide HUD key: 1

### Maintenance

Clear all cycles: CLEAR button
Enable debug logging: [ ]
Reload layout file: 7 (hotkey)

### View cycles

Choose cycle to view:
- (x) Shouts/powers
- ( ) Utilities/consumables
- ( ) Left hand
- ( ) Right hand

Cycle items ⌄
(dropdown shows item names)
