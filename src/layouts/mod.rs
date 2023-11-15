//! module description here

use serde::{Deserialize, Serialize};

pub mod flattened;
pub mod layout_v1;
pub mod layout_v2;
pub mod shared;

pub use crate::plugin::HudLayout1;
pub use layout_v2::HudLayout2;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Layout {
    Version1(crate::plugin::HudLayout1), // TODO move once flattened is done
    Version2(layout_v2::HudLayout2),
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_deserialize_layouts() {
        // TODO
    }
}
