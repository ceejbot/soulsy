//! Rasterize svgs and provide them to the C++ side.
//! Possibly read on the fly?
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

use eyre::{eyre, Result};
use once_cell::sync::Lazy;
use resvg::usvg::TreeParsing;
use resvg::*;

use super::icons::Icon;
use crate::plugin::LoadedImage;

static ICON_MAP: Lazy<Mutex<HashMap<Icon, Icon>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn icon_map() -> std::sync::MutexGuard<'static, HashMap<Icon, Icon>> {
    ICON_MAP
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire icon hashmap lock. Exiting.")
}

const ICON_SVG_PATH: &str = "data/SKSE/plugins/resources/icons/";

/// C++ should call this before trying to load any icon data.
pub fn get_icon_key(name: String) -> String {
    let icon: Icon = Icon::from_str(name.as_str()).unwrap_or_default();
    key_for_icon(&icon).to_string()
}

/// Called by C++, so it needs to handle all errors and signal its
/// success or failure through some means other than a Result.
/// In this case, a zero-length vector is a failure.
pub fn rasterize_icon(name: String, maxdim: u32) -> LoadedImage {
    let icon: Icon = Icon::from_str(name.as_str()).unwrap_or_default();
    match load_icon(&icon, maxdim) {
        Ok(v) => v,
        Err(e) => {
            log::error!("failed to load icon SVG; icon={icon}; error={e}");
            LoadedImage::default()
        }
    }
}

pub fn rasterize_by_path(fpath: String) -> LoadedImage {
    match load_and_rasterize(&fpath.clone().into(), None) {
        Ok(v) => v,
        Err(e) => {
            log::error!("failed to load svg by path; icon={fpath}; error={e}");
            LoadedImage::default()
        }
    }
}

/// Rust can call this to load rasterized icon image data.
pub fn load_icon(icon: &Icon, maxdim: u32) -> Result<LoadedImage> {
    let mapped = key_for_icon(icon);
    let file_path = icon_to_path(&mapped);
    load_and_rasterize(&file_path, Some(maxdim))
}

/// Look up the fallback-aware key for this icon.
/// This allows us to load fallbacks once and hold at most one copy
/// of that texture data in memory.
pub fn key_for_icon(icon: &Icon) -> Icon {
    let mut mapping = icon_map();
    if let Some(result) = mapping.get(icon) {
        return result.clone();
    }

    let first_path = icon_to_path(icon);
    if first_path.exists() {
        mapping.insert(icon.clone(), icon.clone());
        icon.clone()
    } else {
        log::info!("TODO: add svg data for {icon} to this icon pack.");
        let fb = icon.fallback();
        if icon_to_path(&fb).exists() {
            mapping.insert(icon.clone(), fb.clone());
            fb
        } else {
            log::debug!(
                "Fallback icon {fb} failed! path='{}';",
                icon_to_path(&fb).display()
            );
            mapping.insert(icon.clone(), Icon::IconDefault);
            Icon::IconDefault
        }
    }
}

/// Turn an icon into a full path to its svg.
fn icon_to_path(icon: &Icon) -> PathBuf {
    [ICON_SVG_PATH, icon.icon_file().as_str()].iter().collect()
}

/// Internal shared implementation: do the real work.
fn load_and_rasterize(file_path: &PathBuf, maxsize: Option<u32>) -> Result<LoadedImage> {
    let buffer = std::fs::read(file_path)?;
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(&buffer, &opt)?;
    let rtree = resvg::Tree::from_usvg(&tree);

    let (size, transform) = if let Some(maxdim) = maxsize {
        let size = if rtree.size.width() > rtree.size.height() {
            rtree.size.to_int_size().scale_to_width(maxdim)
        } else {
            rtree.size.to_int_size().scale_to_height(maxdim)
        };
        if let Some(size) = size {
            let transform = tiny_skia::Transform::from_scale(
                size.width() as f32 / rtree.size.width() as f32,
                size.height() as f32 / rtree.size.height() as f32,
            );
            (size, transform)
        } else {
            (rtree.size.to_int_size(), tiny_skia::Transform::default())
        }
    } else {
        (rtree.size.to_int_size(), tiny_skia::Transform::default())
    };

    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or(eyre!("unable to allocate pixmap to render into"))?;
    rtree.render(transform, &mut pixmap.as_mut());

    Ok(LoadedImage {
        width: pixmap.width(),
        height: pixmap.height(),
        buffer: pixmap.data().to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_rasterize() {
        let icon = Icon::WeaponSwordOneHanded;
        let full = icon_to_path(&icon);
        assert_eq!(
            full.clone().to_string_lossy(),
            "data/SKSE/plugins/resources/icons/weapon_sword_one_handed.svg".to_string()
        );
        let loaded = load_and_rasterize(&full, Some(128))
            .expect("should return okay for a known-present file");
        assert!(!loaded.buffer.is_empty());
        assert_eq!(loaded.buffer.len(), 128 * 128 * 4); // expected size given dimensions & square image
    }

    #[test]
    fn rasterize_icon_by_variant() {
        let loaded = load_icon(&Icon::Food, 256).expect("this icon should exist");
        assert!(!loaded.buffer.is_empty());
        assert_eq!(loaded.buffer.len(), 256 * 256 * 4); // expected size given dimensions & square image
    }

    #[test]
    fn rasterize_unscaled() {
        let previous = "data/SKSE/plugins/resources/icons/weapon_sword_one_handed.svg".to_string();
        let loaded = rasterize_by_path(previous);
        assert!(!loaded.buffer.is_empty());
        assert_eq!(
            loaded.buffer.len(),
            loaded.width as usize * loaded.height as usize * 4
        );

        let full = "layouts/icon-pack-soulsy/shout_call_dragon.svg".to_string();
        let loaded = rasterize_by_path(full);
        assert!(!loaded.buffer.is_empty());
        assert_eq!(
            loaded.buffer.len(),
            loaded.width as usize * loaded.height as usize * 4
        );
    }
}
