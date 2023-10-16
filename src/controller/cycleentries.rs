use crate::data::base::BaseType;
use crate::data::huditem::HudItem;
use crate::data::icons::Icon;
use crate::data::item_cache::ItemCache;

/// A single equipment set.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EquipSet {
    /// A unique id for this equipset.
    id: u32,
    /// A human-set name for this equipset.
    name: String,
    /// A list of formspecs for items to be equipped when this equipset is selected.
    pub items: Vec<String>,
    /// A list of empty slots.
    pub empty: Vec<u8>,
    /// Which icon to use.
    pub icon: Icon,
}

impl EquipSet {
    /// Create an equipset.
    pub fn new(
        id: u32,
        name: String,
        items: Vec<String>,
        empty: Vec<u8>,
        icon_name: String,
    ) -> Self {
        let icon = Icon::try_from(icon_name.as_str()).unwrap_or(Icon::ArmorHeavy);
        Self {
            id,
            name,
            items,
            empty,
            icon,
        }
    }

    /// Create an equipset from a list of huditems.
    pub fn new_from_items(id: u32, name: String, huditems: Vec<HudItem>, empty: Vec<u8>) -> Self {
        let items: Vec<String> = huditems.iter().map(|xs| xs.form_string()).collect();
        let icon = huditems
            .first()
            .map_or(Icon::ArmorHeavy, |xs| xs.icon().clone());
        Self {
            id,
            name,
            items,
            empty,
            icon,
        }
    }

    /// Get this equipset's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    /// String identifiers did not work out very well here.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Borrow a list of this set's item formspecs.
    pub fn items(&self) -> &[String] {
        self.items.as_slice()
    }

    /// Borrow a list of this set's empty slots. Turn these slot into biped slots
    /// in-game by shifting: `1 << slotnum`.
    pub fn empty_slots(&self) -> &[u8] {
        self.empty.as_slice()
    }

    /// Get this equip set's icon.
    pub fn icon(&self) -> &Icon {
        &self.icon
    }
}

/// Trait for anything that can be in a cycle.
pub trait CycleEntry {
    /// A unique string identifier for this item.
    fn identifier(&self) -> String;
}

impl CycleEntry for HudItem {
    fn identifier(&self) -> String {
        self.form_string()
    }
}

impl CycleEntry for EquipSet {
    fn identifier(&self) -> String {
        self.id.to_string()
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
    fn set_top(&mut self, top: &str);
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

    fn set_top(&mut self, top: &str) {
        if let Some(idx) = self.iter().position(|xs| xs.identifier() == *top) {
            self.rotate_left(idx);
        }
    }

    fn advance(&mut self, amount: usize) -> Option<T> {
        if self.is_empty() {
            return None;
        }
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

    // This requires cache lookups to get full item info.
    fn advance_skipping_twohanders(&mut self, cache: &mut ItemCache) -> Option<String> {
        if self.is_empty() {
            return None;
        }

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
/// It's a trait so we can add the behavior to vec of equipset.
pub trait UpdateableItemCycle {
    type T;
    fn find_next_id(&self) -> u32;
    fn update_set(&mut self, id: u32, items: Vec<String>, empty: Vec<u8>) -> bool;
    fn set_icon_by_id(&mut self, id: u32, icon: Icon) -> bool;
    fn rename_by_id(&mut self, id: u32, name: String) -> bool;
    fn get_by_id(&self, id: u32) -> Option<&Self::T>;
}

impl UpdateableItemCycle for Vec<EquipSet> {
    type T = EquipSet;

    fn find_next_id(&self) -> u32 {
        // This searches for a hole in the list and fills it in,
        // otherwise it finds the last item and increments.
        let mut sorted = self.clone();
        sorted.sort_by_key(|xs| xs.id());
        let mut next: u32 = 0;
        let found = sorted.iter().find_map(|xs| {
            if xs.id == next {
                next += 1;
                None
            } else {
                Some(next)
            }
        });
        if let Some(hole) = found {
            hole
        } else {
            next
        }
    }

    fn update_set(&mut self, id: u32, items: Vec<String>, empty: Vec<u8>) -> bool {
        let Some(idx) = self.iter().position(|xs| xs.id == id) else {
            log::info!("search for id {id} failed");
            return false;
        };
        let Some(to_update) = self.get_mut(idx) else {
            return false;
        };
        log::info!(
            "Updating the items in equipment set '{}'.",
            to_update.name()
        );
        to_update.items = items;
        to_update.empty = empty;
        true
    }

    fn rename_by_id(&mut self, id: u32, name: String) -> bool {
        let Some(idx) = self.iter().position(|xs| xs.id == id) else {
            return false;
        };
        let Some(to_update) = self.get_mut(idx) else {
            return false;
        };
        log::info!(
            "Renaming equipment set '{}' to '{}'.",
            to_update.name(),
            name
        );
        to_update.set_name(name.as_str());
        true
    }

    fn set_icon_by_id(&mut self, id: u32, icon: Icon) -> bool {
        let Some(idx) = self.iter().position(|xs| xs.id == id) else {
            return false;
        };
        let Some(to_update) = self.get_mut(idx) else {
            return false;
        };
        log::info!(
            "Setting the icon for equipment set {} to {icon}.",
            to_update.name()
        );
        to_update.icon = icon;
        true
    }

    fn get_by_id(&self, id: u32) -> Option<&EquipSet> {
        self.iter().find(|xs| xs.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_cycle_behavior() {
        impl CycleEntry for &str {
            fn identifier(&self) -> String {
                self.to_string()
            }
        }

        let mut testcycle = Vec::<&str>::new();
        assert!(testcycle.add(&"one"));
        assert!(testcycle.add(&"two"));
        assert!(testcycle.add(&"three"));
        assert_eq!(testcycle.top().expect("should have a top entry"), "one");
        let advanced = testcycle.advance(1).expect("advancing should work");
        assert_eq!(advanced, "two");
        let next = testcycle
            .peek_next()
            .expect("peeking should return an item");
        assert_eq!(next, "three");
        testcycle.set_top("one");
        assert_eq!(testcycle.top().expect("top should now be one"), "one");
        assert!(testcycle.includes(&"two"));
        assert!(testcycle.delete(&"two"));
        assert!(!testcycle.includes(&"two"));

        assert!(testcycle.add(&"four"));
        assert!(!testcycle.add(&"four"));
        assert!(testcycle.add(&"five"));
        assert_eq!(testcycle.len(), 4);
        assert!(testcycle.filter_id("four"));
        assert!(!testcycle.filter_id("four"));
        assert_eq!(testcycle.len(), 3);
    }

    #[test]
    fn hud_item_cycles() {
        use crate::data::huditem::HudItem;
        use crate::data::item_cache::ItemCache;
        let mut cache = ItemCache::new();
        let mut cycle = Vec::<HudItem>::new();
        let item = cache.get(&"form-one".to_string());
        assert!(cycle.add(&item));

        // functions to test:
        // filter_kind(&mut self, unwanted: &BaseType, cache: &mut ItemCache);
        // advance_skipping(&mut self, skip: &HudItem) -> Option<String>;
        // advance_skipping_twohanders(&mut self, cache: &mut ItemCache) -> Option<String>;
        // names(&self, cache: &mut ItemCache) -> Vec<String>;
    }

    #[test]
    fn finding_the_next_id() {
        let mut cycle = Vec::<EquipSet>::new();
        let id = cycle.find_next_id();
        assert_eq!(id, 0);
        let zero = EquipSet::new(
            id,
            id.to_string(),
            Vec::new(),
            Vec::new(),
            Icon::SpellSun.to_string(),
        );
        assert_eq!(zero.icon, Icon::SpellSun);
        assert!(cycle.add(&zero));
        let id = cycle.find_next_id();
        assert_eq!(id, 1);
        let one = EquipSet::new(
            id,
            id.to_string(),
            Vec::new(),
            Vec::new(),
            Icon::ArmorBackpack.to_string(),
        );
        assert_eq!(one.icon, Icon::ArmorBackpack);
        assert!(cycle.add(&one));
        let id = cycle.find_next_id();
        assert_eq!(id, 2);
        let two = EquipSet::new(
            id,
            id.to_string(),
            Vec::new(),
            Vec::new(),
            "NotAnIcon".to_string(),
        );
        assert_eq!(two.icon, Icon::ArmorHeavy);
        assert!(cycle.add(&two));
        assert!(cycle.delete(&one));
        let id = cycle.find_next_id();
        assert_eq!(id, 1);
    }
}
