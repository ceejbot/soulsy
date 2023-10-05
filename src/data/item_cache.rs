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

impl ItemCache {
    /// Create a new item cache with the given capacity.
    pub fn new(capacity: NonZeroUsize) -> Self {
        let lru = LruCache::new(capacity);
        Self { lru }
    }

    /// Print interesting information about the cache contents to the log.
    pub fn introspect(&self) {
        log::debug!("cache contains {} items; ", self.lru.len());
        log::debug!("    least recently-used item is: {:?}", self.lru.peek_lru());
    }

    /// On load from save, we do not bother attempting to reconcile what
    /// we have cached with what the save state is. We merely enjoy the
    /// eternal sunshine of the spotless mind.
    pub fn clear(&mut self) {
        self.lru.clear();
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

    /// Update the count for a cached item. If the item is not in the
    /// cache, no action is taken.
    pub fn update_count(&mut self, form_spec: &str, delta: i32) -> Option<&HudItem> {
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
fn fetch_game_item(form_string: &String) -> HudItem {
    cxx::let_cxx_string!(form_spec = form_string);
    *formSpecToHudItem(&form_spec)
}

#[cfg(test)]
fn fetch_game_item(form_string: &String) -> HudItem {
    // This is a test implementation that makes a random item.

    use super::color::random_color;
    use super::icons::random_icon;
    use super::weapon::{WeaponEquipType, WeaponType};

    let name = petname::petname(2, " ");
    let item = HudItem::preclassified(
        name.as_bytes().to_vec(),
        form_string.clone(),
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
