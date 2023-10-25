//! Rasterize svgs and provide them to the C++ side.
//! Possibly read on the fly?
pub mod icons;
pub use icons::*;

use anyhow::{anyhow, Result};
use resvg::usvg::TreeParsing;
use resvg::*;

/// Called by C++, so it needs to handle all errors and signal its
/// success or failure through some means other than a Result.
/// In this case, a zero-length vector is a failure.
pub fn load_icon(icon: Icon, maxdim: u32) -> Vec<u8> {
    match load_and_rasterize(icon.icon_file().as_str(), maxdim) {
        Ok(v) => {
            log::trace!("successfully rasterized icon image; icon='{icon:?}';");
            v
        }
        Err(e) => {
            log::error!("failed to load SVG; loading fallback; icon='{icon:?}'; error={e:?}");
            load_and_rasterize(icon.fallback().icon_file().as_str(), maxdim)
                .unwrap_or_else(|_| Vec::new())
        }
    }
}

/// Lower-level function.
pub fn rasterize_svg(file_path: String, maxdim: u32) -> Vec<u8> {
    match load_and_rasterize(file_path.as_str(), maxdim) {
        Ok(v) => {
            log::trace!("successfully rasterized svg; path='{file_path}';");
            v
        }
        Err(e) => {
            log::error!("failed to load SVG; path='{file_path}'; error={e:?}");
            Vec::new()
        }
    }
}

fn load_and_rasterize(file_path: &str, maxdim: u32) -> Result<Vec<u8>> {
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
