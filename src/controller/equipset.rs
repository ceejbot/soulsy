//! Equip set data.

use std::fmt::Display;

use crate::data::{huditem::HudItem, item_cache::ItemCache};

#[derive(Debug, Clone, Hash)]
pub struct FormSpec {
    inner: String,
}

impl FormSpec {
    pub fn to_hud_item(&self, cache: &mut ItemCache) -> HudItem {
        cache.get(&self.inner)
    }

    pub fn equipslot(&self) {
        // not sure what the right return type is yet
        todo!()
    }
}

impl Display for FormSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct EquipSet {
    name: String,
    pub items: Vec<String>,
}

impl EquipSet {
    pub fn new(name: String, items: Vec<String>) -> Self {
        Self { name, items }
    }

    pub fn new_from_items(name: String, huditems: Vec<HudItem>) -> Self {
        let items = huditems.iter().map(|xs| xs.form_string()).collect();
        Self { name, items }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
