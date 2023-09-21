use crate::data::base::BaseType;
use crate::data::huditem::HudItem;
use crate::data::item_cache::ItemCache;

/// A single equipment set.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EquipSet {
    /// A unique id for this equipset. Assumed to be a string representation of a number, e.g. "2".
    id: String,
    /// A human-set name for this equipset.
    name: String,
    /// A list of formspecs for items to be equipped when this equipset is selected.
    pub items: Vec<String>,
}

impl EquipSet {
    /// Create an equipset.
    pub fn new(id: String, name: String, items: Vec<String>) -> Self {
        Self { id, name, items }
    }

    /// Create an equipset from a list of huditems.
    pub fn new_from_items(id: String, name: String, huditems: Vec<HudItem>) -> Self {
        let items = huditems.iter().map(|xs| xs.form_string()).collect();
        Self { id, name, items }
    }

    /// Get this equipset's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }
}

/// Trait for anything that can be in a cycle.
pub trait CycleEntry {
    fn identifier(&self) -> String;
}

impl CycleEntry for HudItem {
    fn identifier(&self) -> String {
        self.form_string()
    }
}

impl CycleEntry for EquipSet {
    fn identifier(&self) -> String {
        self.id.clone()
    }
}

impl CycleEntry for String {
    fn identifier(&self) -> String {
        self.clone()
    }
}

/// Trait for a cycle.
pub trait Cycle<T>
where
    T: CycleEntry + PartialEq + Clone,
{
    fn ids(&self) -> Vec<String>;
    fn top(&self) -> Option<T>;
    fn set_top(&mut self, top: &String);
    fn advance(&mut self, amount: usize) -> Option<T>;
    fn peek_next(&self) -> Option<T>;
    fn includes(&self, item: &T) -> bool;
    fn add(&mut self, item: &T) -> bool;
    fn delete(&mut self, item: &T) -> bool;
    fn filter_id(&mut self, id: &str) -> bool;
}

/// Cycle implementation for vecs of things.
impl<T> Cycle<T> for Vec<T>
where
    T: CycleEntry + PartialEq + Clone,
{
    fn ids(&self) -> Vec<String> {
        self.iter().map(|xs| xs.identifier()).collect()
    }

    fn top(&self) -> Option<T> {
        self.first().cloned()
    }

    fn set_top(&mut self, top: &String) {
        if let Some(idx) = self.iter().position(|xs| xs.identifier() == *top) {
            self.rotate_left(idx);
        }
    }

    fn advance(&mut self, amount: usize) -> Option<T> {
        self.rotate_left(amount);
        self.first().cloned()
    }

    fn peek_next(&self) -> Option<T> {
        if self.len() == 1 {
            self.first().cloned()
        } else {
            self.get(1).cloned()
        }
    }

    fn includes(&self, item: &T) -> bool {
        self.iter().any(|xs| xs == item)
    }

    fn add(&mut self, item: &T) -> bool {
        if self.iter().any(|xs| xs == item) {
            false // we've already got one
        } else {
            self.push(item.clone());
            true
        }
    }

    fn delete(&mut self, item: &T) -> bool {
        let orig_len = self.len();
        self.retain(|xs| xs != item);
        orig_len != self.len()
    }

    fn filter_id(&mut self, id: &str) -> bool {
        let orig_len = self.len();
        self.retain(|xs| xs.identifier() != id);
        orig_len != self.len()
    }
}

/// These functions are unique to item cycles. They're in a trait so we can
/// supply them to Vec<String>.
pub trait HudItemCycle {
    fn filter_kind(&mut self, unwanted: &BaseType, cache: &mut ItemCache);
    fn advance_skipping(&mut self, skip: &HudItem) -> Option<String>;
    fn advance_skipping_twohanders(&mut self, cache: &mut ItemCache) -> Option<String>;
    fn names(&self, cache: &mut ItemCache) -> Vec<String>;
}

impl HudItemCycle for Vec<String> {
    fn names(&self, cache: &mut ItemCache) -> Vec<String> {
        self.iter()
            .filter_map(|xs| cache.get_or_none(xs.as_str()).map(|xs| xs.name()))
            .collect::<Vec<_>>()
    }

    fn filter_kind(&mut self, unwanted: &BaseType, cache: &mut ItemCache) {
        self.retain(|xs| {
            let found = cache.get(xs);
            found.kind() != unwanted
        });
    }

    fn advance_skipping(&mut self, skip: &HudItem) -> Option<String> {
        if self.is_empty() {
            return None;
        }

        self.rotate_left(1);
        let candidate = self.iter().find(|xs| **xs != skip.form_string());
        if let Some(v) = candidate {
            let result = v.clone();
            self.set_top(&result);
            Some(result)
        } else {
            log::trace!("advance skip found nothing?????");
            None
        }
    }

    fn advance_skipping_twohanders(&mut self, cache: &mut ItemCache) -> Option<String> {
        if self.is_empty() {
            return None;
        }

        // This requires cache lookups.
        self.rotate_left(1);
        let candidate = self.iter().find(|xs| {
            let item = cache.get(xs);
            !item.two_handed()
        });
        if let Some(v) = candidate {
            let result = v.clone();
            self.set_top(&result);
            Some(result)
        } else {
            log::trace!("no one-handers in the right cycle");
            None
        }
    }
}

/// A trait for additional behavior needed by cycles of EquipSets.
/// Possibly not really generalizable to more than just that.
pub trait UpdateableItemCycle {
    type T;
    fn update_by_id(&mut self, id: String, item: Self::T) -> bool;
    fn rename_by_id(&mut self, id: String, name: String) -> bool;
    fn get_by_id(&self, id: String) -> Option<&Self::T>;
}

impl UpdateableItemCycle for Vec<EquipSet> {
    type T = EquipSet;

    fn update_by_id(&mut self, id: String, item: EquipSet) -> bool {
        let name = item.name();
        // let id = item.identifier();

        if let Ok(idx) = self.binary_search_by(|xs| xs.identifier().cmp(&id)) {
            let Some(to_update) = self.get_mut(idx) else {
                return false;
            };
            to_update.set_name(name.as_str());
            to_update.items = item.items;
            true
        } else {
            false
        }
    }

    fn rename_by_id(&mut self, id: String, name: String) -> bool {
        if let Ok(idx) = self.binary_search_by(|xs| xs.identifier().cmp(&id)) {
            let Some(to_update) = self.get_mut(idx) else {
                return false;
            };
            to_update.set_name(name.as_str());
            true
        } else {
            false
        }
    }

    fn get_by_id(&self, id: String) -> Option<&EquipSet> {
        self.iter().find(|xs| (**xs).identifier() == id)
    }
}
