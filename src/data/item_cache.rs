//! A cache of HudItems so we don't have to make them all the time. The player probably
//! has a couple dozen items they cycle through. This allows us to use form spec strings
//! as the canonical way to identify an item inside the mod.

use std::num::NonZeroUsize;

use lru::LruCache;

use super::huditem::HudItem;
use crate::data::{make_health_proxy, make_magicka_proxy, make_stamina_proxy};
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
            cxx::let_cxx_string!(form_spec = form_string);
            *formSpecToHudItem(&form_spec)
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
