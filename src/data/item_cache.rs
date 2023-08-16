//! A cache of HudItems so we don't have to make them all the time. The player probably
//! has a couple dozen items they cycle among.

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
    pub fn new(capacity: NonZeroUsize) -> Self {
        let lru = LruCache::new(capacity);
        Self { lru }
    }

    pub fn get_or_create(&mut self, form_string: &String) -> HudItem {
        if let Some(hit) = self.lru.get(form_string) {
            hit.clone()
        } else {
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
    }

    pub fn get(&mut self, form_spec: &String) -> Option<HudItem> {
        self.lru.get(form_spec).cloned()
    }

    pub fn record(&mut self, item: HudItem) {
        self.lru.put(item.form_string(), item);
    }

    pub fn contains(&self, form_spec: &String) -> bool {
        self.lru.contains(form_spec)
    }

    pub fn update_count(&mut self, form_spec: &String, delta: i32) -> Option<&HudItem> {
        if let Some(item) = self.lru.get_mut(form_spec) {
            let current = item.count();
            let new_count = if delta.is_negative() {
                if delta > current as i32 {
                    0
                } else {
                    current - delta.unsigned_abs()
                }
            } else {
                current + delta as u32
            };
            item.set_count(new_count);
            Some(item)
        } else {
            None
        }
    }
}
