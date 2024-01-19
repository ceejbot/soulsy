//! Structs and trait impls for considering keyboard/controller state.
//! There are too many enums here and a substantial rework is called for.

use std::fmt::Display;
use std::time::{Duration, Instant};

use enumset::{enum_set, EnumSet, EnumSetType};
use eyre::eyre;
use strum::Display;

use super::control::RequestedAction;
use super::settings::{settings, ActivationMethod, UnarmedMethod};
use crate::plugin::{hasRangedEquipped, Action, ButtonEvent, HudElement};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Display)]
pub enum CycleSlot {
    Left,
    Power,
    Right,
    Utility,
}

#[derive(Debug, Hash, Default, Display, EnumSetType)]
pub enum Hotkey {
    Power,
    Utility,
    Left,
    Right,
    Equipment,
    Activate,
    UnequipHands,
    Refresh,
    ShowHide,
    UnequipModifier,
    CycleModifier,
    ActivateModifier,
    MenuModifier,
    #[default]
    None,
}

const CYCLE_KEYS: EnumSet<Hotkey> =
    enum_set!(Hotkey::Left | Hotkey::Power | Hotkey::Right | Hotkey::Utility | Hotkey::Equipment);
const MODIFIER_KEYS: EnumSet<Hotkey> = enum_set!(
    Hotkey::ActivateModifier
        | Hotkey::CycleModifier
        | Hotkey::MenuModifier
        | Hotkey::UnequipModifier
);

impl Hotkey {
    pub fn key_for(&self) -> i32 {
        let options = settings();

        match self {
            Hotkey::Power => options.power() as i32,
            Hotkey::Utility => options.utility() as i32,
            Hotkey::Left => options.left() as i32,
            Hotkey::Right => options.right() as i32,
            Hotkey::Equipment => options.equipset() as i32,
            Hotkey::Activate => options.activate() as i32,
            Hotkey::UnequipHands => options.unequip_hotkey() as i32,
            Hotkey::Refresh => options.refresh_layout() as i32,
            Hotkey::ShowHide => options.showhide() as i32,
            Hotkey::UnequipModifier => options.unequip_modifier(),
            Hotkey::CycleModifier => options.cycle_modifier(),
            Hotkey::ActivateModifier => options.activate_modifier(),
            Hotkey::MenuModifier => options.menu_modifier(),
            Hotkey::None => -1,
        }
    }

    pub fn relevant_shortcuts(v: u32) -> EnumSet<Hotkey> {
        let settings = settings();
        let mut set: EnumSet<Hotkey> = EnumSet::new();

        if v == settings.power() {
            set.insert(Hotkey::Power);
        }
        if v == settings.utility() {
            set.insert(Hotkey::Utility);
        }
        if v == settings.left() {
            set.insert(Hotkey::Left);
        }
        if v == settings.right() {
            set.insert(Hotkey::Right);
        }
        if v == settings.equipset() as u32 {
            set.insert(Hotkey::Equipment);
        }
        if v == settings.refresh_layout() {
            set.insert(Hotkey::Refresh);
        }
        if v == settings.showhide() {
            set.insert(Hotkey::ShowHide);
        }
        if v == settings.activate() {
            set.insert(Hotkey::Activate);
        }

        if v == settings.unequip_hotkey() as u32 {
            set.insert(Hotkey::UnequipHands);
        }

        if settings.activate_modifier().is_positive()
            && v == settings.activate_modifier().unsigned_abs()
        {
            set.insert(Hotkey::ActivateModifier);
        }

        if settings.cycle_modifier().is_positive() && v == settings.cycle_modifier().unsigned_abs()
        {
            set.insert(Hotkey::CycleModifier);
        }

        if settings.unequip_modifier().is_positive()
            && v == settings.unequip_modifier().unsigned_abs()
        {
            set.insert(Hotkey::UnequipModifier);
        }

        if settings.menu_modifier().is_positive() && v == settings.menu_modifier().unsigned_abs() {
            set.insert(Hotkey::MenuModifier);
        }

        set
    }

    pub fn long_press_action(&self) -> RequestedAction {
        let settings = settings();
        let advance = matches!(settings.cycle_advance_method(), ActivationMethod::LongPress);
        let unequip = matches!(settings.unequip_method(), UnarmedMethod::LongPress);

        if matches!(self, Hotkey::Power) {
            if unequip {
                RequestedAction::Unequip
            } else if advance {
                RequestedAction::Advance
            } else {
                RequestedAction::None
            }
        } else if matches!(self, Hotkey::Utility) {
            let consume = matches!(
                settings.utility_activation_method(),
                ActivationMethod::LongPress
            );
            if consume {
                RequestedAction::Consume
            } else if advance {
                RequestedAction::Advance
            } else {
                RequestedAction::None
            }
        } else if matches!(self, Hotkey::Left | Hotkey::Right) {
            let dual_wield = settings.long_press_to_dual_wield();
            if unequip {
                RequestedAction::Unequip
            } else if dual_wield {
                RequestedAction::Match
            } else if advance {
                if matches!(self, Hotkey::Left) && settings.cycle_ammo() && hasRangedEquipped() {
                    RequestedAction::AdvanceAmmo
                } else {
                    RequestedAction::Advance
                }
            } else {
                RequestedAction::None
            }
        } else {
            RequestedAction::None
        }
    }
}

impl From<&CycleSlot> for Hotkey {
    fn from(value: &CycleSlot) -> Self {
        match *value {
            CycleSlot::Left => Hotkey::Left,
            CycleSlot::Power => Hotkey::Power,
            CycleSlot::Right => Hotkey::Right,
            CycleSlot::Utility => Hotkey::Utility,
        }
    }
}

impl From<&Action> for Hotkey {
    fn from(value: &Action) -> Self {
        match *value {
            Action::Activate => Hotkey::Activate,
            Action::Left => Hotkey::Left,
            Action::Power => Hotkey::Power,
            Action::Right => Hotkey::Right,
            Action::Equipment => Hotkey::Equipment,
            Action::ShowHide => Hotkey::ShowHide,
            Action::Utility => Hotkey::Utility,
            Action::RefreshLayout => Hotkey::Refresh,
            Action::UnequipHands => Hotkey::UnequipHands,
            _ => Hotkey::None,
        }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Display)]
pub enum KeyState {
    #[default]
    Up,
    Pressed,
    Down,
}

impl From<&ButtonEvent> for KeyState {
    fn from(event: &ButtonEvent) -> Self {
        if event.IsDown() {
            KeyState::Down
        } else if event.IsPressed() {
            KeyState::Pressed
        } else {
            KeyState::Up
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TrackedKey {
    /// The code for the pressed key being tracked.
    pub key: u32,
    /// The UX-meaningful hotkey actions represented by this key event.
    hotkeys: EnumSet<Hotkey>,
    /// The current statue of the key.
    pub state: KeyState,
    /// When we started tracking this key.
    pub press_start: Option<Instant>,
}

impl TrackedKey {
    pub fn new(key: u32, event: &ButtonEvent) -> Self {
        let press_start = Some(Instant::now());
        let state = KeyState::from(event);
        let hotkeys = Hotkey::relevant_shortcuts(key);

        Self {
            key,
            hotkeys,
            state,
            press_start,
        }
    }

    pub fn shortcut_for(&self) -> Hotkey {
        self.hotkeys.iter().find_map(Some).unwrap_or_default()
    }

    pub fn ignore(&self) -> bool {
        self.hotkeys.contains(Hotkey::None) || self.hotkeys.is_empty()
    }

    pub fn is_modifier(&self) -> bool {
        self.hotkeys.is_subset(MODIFIER_KEYS)
    }

    pub fn is_cycle_key(&self) -> bool {
        self.hotkeys.is_subset(CYCLE_KEYS)
    }

    pub fn update(&mut self, event: &ButtonEvent) {
        self.state = KeyState::from(event);
        match self.state {
            KeyState::Up => {
                // nothing?
            }
            KeyState::Pressed => {
                if self.press_start.is_none() {
                    self.press_start = Some(Instant::now());
                }
            }
            KeyState::Down => {
                self.press_start = Some(Instant::now());
            }
        }
    }

    pub fn is_long_press(&self) -> bool {
        if let Some(start) = self.press_start {
            let elapsed_time = start.elapsed();
            elapsed_time > Duration::from_millis(settings().long_press_ms().into())
        } else {
            false
        }
    }

    pub fn is_up(&self) -> bool {
        matches!(self.state, KeyState::Up)
    }

    pub fn is_pressed(&self) -> bool {
        matches!(self.state, KeyState::Pressed | KeyState::Down)
    }
}

impl Default for TrackedKey {
    fn default() -> Self {
        Self {
            key: 0,
            hotkeys: enum_set!(Hotkey::None),
            state: KeyState::Up,
            press_start: None,
        }
    }
}

impl Display for TrackedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key: kind={}; state={};", self.key, self.state)
    }
}

impl From<&Hotkey> for Action {
    fn from(value: &Hotkey) -> Self {
        match value {
            Hotkey::Power => Action::Power,
            Hotkey::Utility => Action::Utility,
            Hotkey::Left => Action::Left,
            Hotkey::Right => Action::Right,
            Hotkey::Activate => Action::Activate,
            Hotkey::Refresh => Action::RefreshLayout,
            Hotkey::ShowHide => Action::ShowHide,
            _ => Action::None,
        }
    }
}

/// Soon I'll have moved this around to the point where I can remove enums.
impl From<&Hotkey> for HudElement {
    fn from(value: &Hotkey) -> Self {
        match value {
            Hotkey::Power => HudElement::Power,
            Hotkey::Utility => HudElement::Utility,
            Hotkey::Left => HudElement::Left,
            Hotkey::Right => HudElement::Right,
            _ => HudElement::None,
        }
    }
}

impl TryFrom<Action> for CycleSlot {
    type Error = eyre::Error;

    fn try_from(value: Action) -> Result<Self, Self::Error> {
        match value {
            Action::Power => Ok(CycleSlot::Power),
            Action::Utility => Ok(CycleSlot::Utility),
            Action::Left => Ok(CycleSlot::Left),
            Action::Right => Ok(CycleSlot::Right),
            _ => Err(eyre!("this action does not map to a cycle; key={value:?}")),
        }
    }
}

impl TryFrom<Hotkey> for CycleSlot {
    type Error = eyre::Error;

    fn try_from(value: Hotkey) -> Result<Self, Self::Error> {
        match value {
            Hotkey::Power => Ok(CycleSlot::Power),
            Hotkey::Utility => Ok(CycleSlot::Utility),
            Hotkey::Left => Ok(CycleSlot::Left),
            Hotkey::Right => Ok(CycleSlot::Right),
            _ => Err(eyre!("this hotkey is not a cycle key; key={value}")),
        }
    }
}

impl From<&CycleSlot> for HudElement {
    fn from(value: &CycleSlot) -> Self {
        match value {
            CycleSlot::Power => HudElement::Power,
            CycleSlot::Utility => HudElement::Utility,
            CycleSlot::Left => HudElement::Left,
            CycleSlot::Right => HudElement::Right,
        }
    }
}

impl From<CycleSlot> for Action {
    fn from(value: CycleSlot) -> Self {
        match value {
            CycleSlot::Power => Action::Power,
            CycleSlot::Utility => Action::Utility,
            CycleSlot::Left => Action::Left,
            CycleSlot::Right => Action::Right,
        }
    }
}
