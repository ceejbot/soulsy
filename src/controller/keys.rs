//! Structs and trait impls for considering keyboard/controller state.

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

    pub fn is_cycle_key(&self) -> bool {
        CYCLE_KEYS.contains(*self)
    }

    pub fn is_modifier_key(&self) -> bool {
        MODIFIER_KEYS.contains(*self)
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

impl From<u32> for Hotkey {
    fn from(v: u32) -> Self {
        let settings = settings();
        if v == settings.power() {
            Hotkey::Power
        } else if v == settings.utility() {
            Hotkey::Utility
        } else if v == settings.left() {
            Hotkey::Left
        } else if v == settings.right() {
            Hotkey::Right
        } else if v == settings.equipset() as u32 {
            Hotkey::Equipment
        } else if v == settings.refresh_layout() {
            Hotkey::Refresh
        } else if v == settings.showhide() {
            Hotkey::ShowHide
        } else if v == settings.activate() {
            Hotkey::Activate
        } else if v == settings.unequip_hotkey() as u32 {
            Hotkey::UnequipHands
        } else if settings.activate_modifier().is_positive()
            && v == settings.activate_modifier().unsigned_abs()
        {
            Hotkey::ActivateModifier
        } else if settings.cycle_modifier().is_positive()
            && v == settings.cycle_modifier().unsigned_abs()
        {
            Hotkey::CycleModifier
        } else if settings.unequip_modifier().is_positive()
            && v == settings.unequip_modifier().unsigned_abs()
        {
            Hotkey::UnequipModifier
        } else if settings.menu_modifier().is_positive()
            && v == settings.menu_modifier().unsigned_abs()
        {
            Hotkey::MenuModifier
        } else {
            Hotkey::None
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
    pub key: Hotkey,
    pub state: KeyState,
    pub press_start: Option<Instant>,
}

impl TrackedKey {
    pub fn ignore(&self) -> bool {
        matches!(self.key, Hotkey::None)
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
            key: Hotkey::None,
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
