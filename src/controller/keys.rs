//! Structs and trait impls for considering keyboard/controller state.

use std::fmt::Display;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use strum::Display;

use crate::controller::user_settings;
use crate::plugin::{Action, ButtonEvent, HudElement};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Display)]
pub enum CycleSlot {
    Left,
    Power,
    Right,
    Utility,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Display)]
pub enum HotkeyKind {
    Power,
    Utility,
    Left,
    Right,
    Activate,
    Refresh,
    ShowHide,
    UnequipModifier,
    CycleModifier,
    ActivateModifier,
    MenuModifier,
    #[default]
    None,
}

impl HotkeyKind {
    pub fn is_cycle_key(&self) -> bool {
        matches!(
            *self,
            HotkeyKind::Left | HotkeyKind::Power | HotkeyKind::Right | HotkeyKind::Utility
        )
    }

    pub fn is_modifier_key(&self) -> bool {
        matches!(
            *self,
            HotkeyKind::ActivateModifier
                | HotkeyKind::CycleModifier
                | HotkeyKind::MenuModifier
                | HotkeyKind::UnequipModifier
        )
    }
}

impl From<&CycleSlot> for HotkeyKind {
    fn from(value: &CycleSlot) -> Self {
        match *value {
            CycleSlot::Left => HotkeyKind::Left,
            CycleSlot::Power => HotkeyKind::Power,
            CycleSlot::Right => HotkeyKind::Right,
            CycleSlot::Utility => HotkeyKind::Utility,
        }
    }
}

impl From<&Action> for HotkeyKind {
    fn from(value: &Action) -> Self {
        match *value {
            Action::Activate => HotkeyKind::Activate,
            Action::Left => HotkeyKind::Left,
            Action::Power => HotkeyKind::Power,
            Action::Right => HotkeyKind::Right,
            Action::ShowHide => HotkeyKind::ShowHide,
            Action::Utility => HotkeyKind::Utility,
            Action::RefreshLayout => HotkeyKind::Refresh,
            _ => HotkeyKind::None,
        }
    }
}

impl From<u32> for HotkeyKind {
    fn from(v: u32) -> Self {
        let settings = user_settings();
        if v == settings.power() {
            HotkeyKind::Power
        } else if v == settings.utility() {
            HotkeyKind::Utility
        } else if v == settings.left() {
            HotkeyKind::Left
        } else if v == settings.right() {
            HotkeyKind::Right
        } else if v == settings.refresh_layout() {
            HotkeyKind::Refresh
        } else if v == settings.showhide() {
            HotkeyKind::ShowHide
        } else if v == settings.activate() {
            HotkeyKind::Activate
        } else if settings.activate_modifier().is_positive()
            && v == settings.activate_modifier().unsigned_abs()
        {
            HotkeyKind::ActivateModifier
        } else if settings.cycle_modifier().is_positive()
            && v == settings.cycle_modifier().unsigned_abs()
        {
            HotkeyKind::CycleModifier
        } else if settings.unequip_modifier().is_positive()
            && v == settings.unequip_modifier().unsigned_abs()
        {
            HotkeyKind::UnequipModifier
        } else if settings.menu_modifier().is_positive()
            && v == settings.menu_modifier().unsigned_abs()
        {
            HotkeyKind::MenuModifier
        } else {
            HotkeyKind::None
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
    pub key: HotkeyKind,
    pub state: KeyState,
    pub press_start: Option<Instant>,
}

impl TrackedKey {
    pub fn ignore(&self) -> bool {
        matches!(self.key, HotkeyKind::None)
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
            let settings = user_settings();
            elapsed_time > Duration::from_millis(settings.long_press_ms().into())
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
            key: HotkeyKind::None,
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

impl From<&HotkeyKind> for Action {
    fn from(value: &HotkeyKind) -> Self {
        match value {
            HotkeyKind::Power => Action::Power,
            HotkeyKind::Utility => Action::Utility,
            HotkeyKind::Left => Action::Left,
            HotkeyKind::Right => Action::Right,
            HotkeyKind::Activate => Action::Activate,
            HotkeyKind::Refresh => Action::RefreshLayout,
            HotkeyKind::ShowHide => Action::ShowHide,
            _ => Action::None,
        }
    }
}

/// Soon I'll have moved this around to the point where I can remove enums.
impl From<&HotkeyKind> for HudElement {
    fn from(value: &HotkeyKind) -> Self {
        match value {
            HotkeyKind::Power => HudElement::Power,
            HotkeyKind::Utility => HudElement::Utility,
            HotkeyKind::Left => HudElement::Left,
            HotkeyKind::Right => HudElement::Right,
            _ => HudElement::None,
        }
    }
}

impl TryFrom<Action> for CycleSlot {
    type Error = anyhow::Error;

    fn try_from(value: Action) -> Result<Self, Self::Error> {
        match value {
            Action::Power => Ok(CycleSlot::Power),
            Action::Utility => Ok(CycleSlot::Utility),
            Action::Left => Ok(CycleSlot::Left),
            Action::Right => Ok(CycleSlot::Right),
            _ => Err(anyhow!(
                "this action does not map to a cycle; key={value:?}"
            )),
        }
    }
}

impl TryFrom<HotkeyKind> for CycleSlot {
    type Error = anyhow::Error;

    fn try_from(value: HotkeyKind) -> Result<Self, Self::Error> {
        match value {
            HotkeyKind::Power => Ok(CycleSlot::Power),
            HotkeyKind::Utility => Ok(CycleSlot::Utility),
            HotkeyKind::Left => Ok(CycleSlot::Left),
            HotkeyKind::Right => Ok(CycleSlot::Right),
            _ => Err(anyhow!("this hotkey is not a cycle key; key={value}")),
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
