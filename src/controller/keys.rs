//! Structs and trait impls for considering keyboard/controller state.

use std::fmt::Display;

use strum::Display;

use crate::controller::user_settings;
use crate::plugin::{Action, ButtonEvent, HudElement};

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
    #[default]
    None,
}

impl From<u32> for HotkeyKind {
    fn from(v: u32) -> Self {
        let settings = user_settings();
        if v == settings.power {
            HotkeyKind::Power
        } else if v == settings.utility {
            HotkeyKind::Utility
        } else if v == settings.left {
            HotkeyKind::Left
        } else if v == settings.right {
            HotkeyKind::Right
        } else if v == settings.refresh_layout {
            HotkeyKind::Refresh
        } else if v == settings.showhide {
            HotkeyKind::ShowHide
        } else if v == settings.activate {
            HotkeyKind::Activate
        } else if settings.activate_modifier.is_positive()
            && v == settings.activate_modifier.unsigned_abs()
        {
            HotkeyKind::ActivateModifier
        } else if settings.cycle_modifier.is_positive()
            && v == settings.cycle_modifier.unsigned_abs()
        {
            HotkeyKind::CycleModifier
        } else if settings.unequip_modifier.is_positive()
            && v == settings.unequip_modifier.unsigned_abs()
        {
            HotkeyKind::UnequipModifier
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
}

impl TrackedKey {
    pub fn ignore(&self) -> bool {
        matches!(self.key, HotkeyKind::None)
    }

    pub fn update(&mut self, event: &ButtonEvent) {
        self.state = KeyState::from(event);
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
