//! Rasterize svgs and provide them to the C++ side.
//! Possibly read on the fly?
pub mod icons;
use std::ffi::OsString;

use anyhow::{anyhow, Result};
pub use icons::*;
use resvg::usvg::TreeParsing;
use resvg::*;

#[cfg(test)]
const ICON_SVG_PATH: &'static str = "data/SKSE/plugins/resources/icons/";
#[cfg(not(test))]
const ICON_SVG_PATH: &'static str = "SKSE/plugins/resources/icons/";

/// Called by C++, so it needs to handle all errors and signal its
/// success or failure through some means other than a Result.
/// In this case, a zero-length vector is a failure.
pub fn load_icon_with_fallback(icon: Icon, maxdim: u32) -> Vec<u8> {
    let first_path = icon_to_path(&icon);
    match load_and_rasterize(&first_path, maxdim) {
        Ok(v) => {
            log::trace!("successfully rasterized icon image; icon='{icon:?}';");
            v
        }
        Err(e) => {
            let fallback_path = icon_fallback_path(&icon);
            log::error!("failed to load SVG; loading fallback; icon='{icon:?}'; error={e:?}");
            load_and_rasterize(&fallback_path, maxdim).unwrap_or_else(|_| Vec::new())
        }
    }
}

pub fn rasterize_svg(icon: Icon, maxdim: u32) -> Vec<u8> {
    let file_path = icon_to_path(&icon);
    match load_and_rasterize(&file_path, maxdim) {
        Ok(v) => {
            log::trace!(
                "successfully rasterized svg; path='{:?}';",
                file_path.into_string()
            );
            v
        }
        Err(e) => {
            log::error!(
                "failed to load SVG; path='{:?}'; error={e:?}",
                file_path.into_string()
            );
            Vec::new()
        }
    }
}

/// Rust wants to use this.
pub fn load_icon(icon: Icon, maxdim: u32) -> Result<Vec<u8>> {
    let file_path = icon_to_path(&icon);
    load_and_rasterize(&file_path, maxdim)
}

fn icon_to_path(icon: &Icon) -> OsString {
    let icon_chonk = OsString::from(icon.icon_file());
    let mut full_path = OsString::from(ICON_SVG_PATH);
    full_path.push(icon_chonk.as_os_str());
    full_path
}

fn icon_fallback_path(icon: &Icon) -> OsString {
    let icon_chonk = OsString::from(icon.fallback().icon_file());
    let mut full_path = OsString::from(ICON_SVG_PATH);
    full_path.push(icon_chonk.as_os_str());
    full_path
}

/// Internal shared implementation.
fn load_and_rasterize(file_path: &OsString, maxdim: u32) -> Result<Vec<u8>> {
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

    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
    rtree.render(transform, &mut pixmap.as_mut());

    let pixmap_size = rtree.size.to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
    Ok(pixmap.data().to_vec())
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
            full.clone()
                .into_string()
                .expect("this should be valid utf8"),
            "data/SKSE/plugins/resources/icons/weapon_sword_one_handed.svg".to_string()
        );
        let buffer =
            load_and_rasterize(&full, 128).expect("should return okay for a known-present file");
        eprintln!("{}", buffer.len());
        assert!(buffer.len() > 0);
    }
}
