use std::ffi::CString;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::plugin::ItemKind;

/// ItemData, exposed to C++ as an opaque type.
#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq)]
pub struct ItemData {
    #[serde(skip)]
    /// Player-visible name as its underlying bytes.
    name_bytes: Vec<u8>,
    #[serde(skip)]
    /// a bit saying if we had to lose data to encode the name
    name_is_utf8: bool,
    /// Name as utf8
    name: String,
    /// A string that can be turned back into form data; for serializing.
    form_string: String,
    /// An enum classifying this item for fast question-answering as well as icon selection.
    kind: ItemKind,
    /// True if this item requires both hands to use.
    two_handed: bool,
    /// True if this item should be displayed with count data.
    has_count: bool,
    /// Cached count from inventory data. Relies on hooks to be updated.
    count: u32,
    #[serde(skip)]
    /// is currently highlighted for some reason
    highlighted: bool,
}

// Testing the formstring is sufficient for our needs, which is figuring out if
// this form item is in a cycle already.
impl PartialEq for ItemData {
    fn eq(&self, other: &Self) -> bool {
        self.form_string == other.form_string
    }
}

impl Display for ItemData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name='{}'; form_string='{}'; kind='{:?}'",
            self.name(),
            self.form_string(),
            self.kind()
        )
    }
}

/// This is called from C++ when handing us a new item.
pub fn itemdata_from_formdata(
    icon_kind: ItemKind,
    two_handed: bool,
    has_count: bool,
    count: u32,
    bytes_ffi: &cxx::CxxVector<u8>,
    form_string: &str,
) -> Box<ItemData> {
    let name_bytes: Vec<u8> = bytes_ffi.iter().copied().collect();
    Box::new(ItemData::new(
        icon_kind,
        two_handed,
        has_count,
        count,
        name_bytes,
        form_string,
    ))
}

// ---------- Special items that do not correspond with game form items.

pub fn hand2hand_itemdata() -> Box<ItemData> {
    Box::new(ItemData::new_with_name(
        ItemKind::HandToHand,
        false,
        false,
        1,
        "Unarmed".to_string(),
        "unarmed_proxy",
    ))
}

pub fn make_magicka_proxy(count: u32) -> ItemData {
    ItemData::new_with_name(
        ItemKind::PotionMagicka,
        false,
        true,
        count,
        "Best Magicka".to_string(),
        "magicka_proxy",
    )
}

pub fn make_health_proxy(count: u32) -> ItemData {
    ItemData::new_with_name(
        ItemKind::PotionHealth,
        false,
        true,
        count,
        "Best Health".to_string(),
        "health_proxy",
    )
}

pub fn make_stamina_proxy(count: u32) -> ItemData {
    ItemData::new_with_name(
        ItemKind::PotionStamina,
        false,
        true,
        count,
        "Best Stamina".to_string(),
        "stamina_proxy",
    )
}

/// Construct a default ItemData struct, which is displayed as
/// an empty spot on the HUD.
pub fn empty_itemdata() -> Box<ItemData> {
    Box::<ItemData>::default()
}

// ---------- end of special items

impl ItemData {
    pub fn new(
        icon_kind: ItemKind,
        two_handed: bool,
        has_count: bool,
        count: u32,
        name_bytes: Vec<u8>,
        form_string: &str,
    ) -> Self {
        // let's try to get a name string out of the bytes
        let mut name_is_utf8 = false;
        let cstring = match CString::from_vec_with_nul(name_bytes.clone()) {
            Ok(cstring) => cstring,
            Err(e) => {
                if let Ok(cstring) = CString::new(name_bytes.clone()) {
                    cstring
                } else {
                    log::info!(
                        "Surprising: item name bytes were an invalid c string; error: {e:#}"
                    );
                    CString::default()
                }
            }
        };

        let name = if let Ok(v) = cstring.clone().into_string() {
            name_is_utf8 = true;
            v
        } else {
            log::trace!("item name is invalid utf-8; falling back to lossy string");
            cstring.to_string_lossy().to_string()
        };

        ItemData {
            name,
            name_bytes,
            name_is_utf8,
            form_string: form_string.to_string(),
            kind: icon_kind,
            two_handed,
            has_count,
            count,
            highlighted: false,
        }
    }

    pub fn new_with_name(
        icon_kind: ItemKind,
        two_handed: bool,
        has_count: bool,
        count: u32,
        name: String,
        form_string: &str,
    ) -> Self {
        let mut name_bytes: Vec<u8> = name.as_bytes().to_vec();
        name_bytes.push(0x00);
        Self {
            name,
            name_bytes,
            name_is_utf8: true,
            form_string: form_string.to_string(),
            kind: icon_kind,
            two_handed,
            has_count,
            count,
            highlighted: false,
        }
    }

    /// Get the utf8 name of this item, if possible.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn name_is_utf8(&self) -> bool {
        self.name_is_utf8
    }

    pub fn name_bytes(&self) -> Vec<u8> {
        self.name_bytes.clone()
    }

    /// Check if this item must be equipped with both hands.
    pub fn two_handed(&self) -> bool {
        self.two_handed
    }

    pub fn has_count(&self) -> bool {
        self.has_count
    }

    /// If this item has a count, e.g., is arrows, return how many the player has.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Update the count following an inventory-count-changed event.
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }

    /// Get this item's form string, which encodes mod esp and formid.
    /// Should be stable across game loads.
    pub fn form_string(&self) -> String {
        self.form_string.clone()
    }

    /// Get the enum representing this entry's kind (1-handed sword, dagger, health potion, etc.)
    pub fn kind(&self) -> ItemKind {
        self.kind
    }

    /// True if this entry should be drawn with a highlight.
    pub fn highlighted(&self) -> bool {
        self.highlighted
    }

    /// Set whether this item should be drawn highlighted or not.
    pub fn set_highlighted(&mut self, is_shiny: bool) {
        self.highlighted = is_shiny;
    }
}
