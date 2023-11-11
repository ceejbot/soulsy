//! A cache of HudItems so we don't have to make them all the time. The player probably
//! has a couple dozen items they cycle among. (My tests have me hovering at about 22
//! items, but that's anecdata.) We cache 100 before we evict. This number is not driven
//! by memory pressure. The icons use more memory than this, probably. Cache updates
//! should be handled by the inventory count hooks we've got, but IDK.

use std::num::NonZeroUsize;

use lru::LruCache;

use super::huditem::HudItem;
use crate::data::{make_health_proxy, make_magicka_proxy, make_stamina_proxy};
#[cfg(not(test))]
use crate::plugin::formSpecToHudItem;

#[derive(Debug)]
pub struct ItemCache {
    lru: LruCache<String, HudItem>,
}

impl Default for ItemCache {
    fn default() -> Self {
        ItemCache::new()
    }
}

impl ItemCache {
    /// Create a new item cache with the given capacity.
    pub fn new() -> Self {
        let capacity =
            NonZeroUsize::new(200).expect("cats and dogs living together! 200 is not non-zero!");
        let lru = LruCache::new(capacity);
        Self { lru }
    }

    /// Print interesting information about the cache contents to the log.
    pub fn introspect(&self) {
        log::debug!("cache contains {} items; ", self.lru.len());
        if let Some(entry) = self.lru.peek_lru() {
            log::debug!("    least recently-used item is: {}", entry.1);
        }
    }

    /// On load from save, we do not bother attempting to reconcile what
    /// we have cached with what the save state is. We merely enjoy the
    /// eternal sunshine of the spotless mind.
    pub fn clear(&mut self) {
        self.introspect();
        self.lru.clear();
        log::debug!("item cache cleared.");
    }

    /// Retrieve the named item from the cache. As a side effect, will create a
    /// HudItem for this form id if none was in the cache.
    pub fn get(&mut self, form_string: &String) -> HudItem {
        if let Some(hit) = self.lru.get(form_string) {
            hit.clone()
        } else {
            self.get_with_refresh(form_string)
        }
    }

    /// Cache invalidation is one of the two hardest problems in computer science.
    pub fn get_with_refresh(&mut self, form_string: &String) -> HudItem {
        let item = if form_string == "health_proxy" {
            make_health_proxy()
        } else if form_string == "stamina_proxy" {
            make_stamina_proxy()
        } else if form_string == "magicka_proxy" {
            make_magicka_proxy()
        } else if form_string == "unarmed_proxy" {
            HudItem::make_unarmed_proxy()
        } else {
            fetch_game_item(form_string)
        };

        self.record(item.clone());
        item
    }

    /// Get with no retrieve.
    pub fn get_or_none(&mut self, form_string: &str) -> Option<HudItem> {
        self.lru.get(form_string).cloned()
    }

    /// If you have a HudItem, record it in the cache.
    pub fn record(&mut self, item: HudItem) {
        self.lru.put(item.form_string(), item);
    }

    /// Check if the given form id is represented in the cache.
    pub fn contains(&self, form_spec: &str) -> bool {
        self.lru.contains(form_spec)
    }

    /// Set the count of a cached item to the passed-in value.
    pub fn set_count(&mut self, form_spec: &str, new_count: u32) -> Option<&HudItem> {
        if !self.contains(form_spec) {
            let fetched = fetch_game_item(form_spec);
            self.record(fetched);
        }

        let Some(item) = self.lru.get_mut(form_spec) else {
            return None;
        };
        item.set_count(new_count);
        Some(item)
    }

    /// Update the count for a cached item. If the item is not in the
    /// cache, no action is taken.
    pub fn update_count(&mut self, form_spec: &str, delta: i32) -> Option<&HudItem> {
        if !self.contains(form_spec) {
            let fetched = fetch_game_item(form_spec);
            self.record(fetched);
        }
        let Some(item) = self.lru.get_mut(form_spec) else {
            return None;
        };

        let current = item.count();
        let new_count = if delta.is_negative() {
            if delta.unsigned_abs() >= current {
                0
            } else {
                current - delta.unsigned_abs()
            }
        } else {
            current + delta as u32
        };
        item.set_count(new_count);
        Some(item)
    }
}

#[cfg(not(test))]
pub fn fetch_game_item(form_string: &str) -> HudItem {
    cxx::let_cxx_string!(form_spec = form_string);
    *formSpecToHudItem(&form_spec)
}

// This implementation is used by tests to generate random items without
// attempting to communicate with a running game.
#[cfg(test)]
pub fn fetch_game_item(form_string: &str) -> HudItem {
    use super::color::random_color;
    use super::weapon::{WeaponEquipType, WeaponType};
    use crate::images::random_icon;

    let name = petname::petname(2, " ");
    let item = HudItem::preclassified(
        name.as_bytes().to_vec(),
        form_string.to_owned(),
        2,
        super::BaseType::Weapon(WeaponType::new(
            random_icon(),
            random_color(),
            WeaponEquipType::EitherHand,
        )),
    );
    item
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor_works() {
        let spec = "test-spec".to_string();
        let item = fetch_game_item(&spec);
        assert!(item.name_is_utf8());
        assert_eq!(item.form_string(), spec);
    }
}
