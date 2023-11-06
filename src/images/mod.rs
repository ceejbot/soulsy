//! Rasterize svgs and provide them to the C++ side.
//! Possibly read on the fly?
pub mod icons;
use crate::plugin::LoadedImage;
pub use icons::*;

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use resvg::usvg::TreeParsing;
use resvg::*;

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
pub fn load_icon_data(name: String, maxdim: u32) -> LoadedImage {
    let icon: Icon = Icon::from_str(name.as_str()).unwrap_or_default();
    match load_icon(&icon, maxdim) {
        Ok(v) => {
            log::debug!("successfully rasterized svg; icon={icon}; width={}; data len={};", v.width, v.buffer.len());
            v
        }
        Err(e) => {
            log::error!("failed to load SVG; icon={icon}; error={e:?}");
            LoadedImage::default()
        }
    }
}

/// Rust can call this to load rasterized icon image data.
pub fn load_icon(icon: &Icon, maxdim: u32) -> Result<LoadedImage> {
    let mapped = key_for_icon(icon);
    let file_path = icon_to_path(&mapped);
    load_and_rasterize(&file_path, maxdim)
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
        log::debug!("first path did not exist: {}", first_path.display());
        let fb = icon.fallback();
        if icon_to_path(&fb).exists() {
            mapping.insert(icon.clone(), fb.clone());
            fb
        } else {
            log::debug!("second path did not exist: {}", icon_to_path(&fb).display());
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
fn load_and_rasterize(file_path: &PathBuf, maxdim: u32) -> Result<LoadedImage> {
    let buffer = std::fs::read(file_path)?;
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(&buffer, &opt)?;
    let rtree = resvg::Tree::from_usvg(&tree);

    let size = if rtree.size.width() > rtree.size.height() {
        rtree.size.to_int_size().scale_to_width(maxdim)
    } else {
        rtree.size.to_int_size().scale_to_height(maxdim)
    };

    let Some(size) = size else {
        return Err(anyhow!("surprising failure to build a new size object"));
    };

    let transform = tiny_skia::Transform::from_scale(
        size.width() as f32 / rtree.size.width() as f32,
        size.height() as f32 / rtree.size.height() as f32,
    );

    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or(anyhow!("unable to allocate first pixmap"))?;
    rtree.render(transform, &mut pixmap.as_mut());

    Ok(LoadedImage {
        width: pixmap.width(),
        height: pixmap.height(),
        buffer: pixmap.data().to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::icons::Icon;
    use super::*;

    #[test]
    fn load_one() {
        let icon = Icon::WeaponSwordOneHanded;
        let full = icon_to_path(&icon);
        assert_eq!(
            full.clone().to_string_lossy(),
            "data/SKSE/plugins/resources/icons/weapon_sword_one_handed.svg".to_string()
        );
        let loaded =
            load_and_rasterize(&full, 128).expect("should return okay for a known-present file");
        assert!(!loaded.buffer.is_empty());
        assert_eq!(loaded.buffer.len(), 128 * 128 * 4); // expected size given dimensions & square image
    }
}
