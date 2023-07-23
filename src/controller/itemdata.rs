use serde::{Deserialize, Serialize};

use crate::plugin::ItemKind;

/// TesItemData, exposed to C++ as an opaque type.
#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq)]
pub struct ItemData {
    /// Player-visible name.
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

/// Make a TesItemData struct from the given data.
pub fn itemdata_from_formdata(
    icon_kind: ItemKind,
    two_handed: bool,
    has_count: bool,
    count: u32,
    name: &str,
    form_string: &str,
) -> Box<ItemData> {
    Box::new(ItemData::new(
        icon_kind,
        two_handed,
        has_count,
        count,
        name,
        form_string,
    ))
}

pub fn hand2hand_itemdata() -> Box<ItemData> {
    Box::new(ItemData::new(
        ItemKind::HandToHand,
        false,
        false,
        1,
        "Unarmed",
        "",
    ))
}

/// Construct a default TesItemData struct, which is displayed as
/// an empty spot on the HUD.
pub fn empty_itemdata() -> Box<ItemData> {
    Box::<ItemData>::default()
}

impl ItemData {
    /// This is called from C++ when handing us a new item.
    pub fn new(
        icon_kind: ItemKind,
        two_handed: bool,
        has_count: bool,
        count: u32,
        name: &str,
        form_string: &str,
    ) -> Self {
        ItemData {
            name: name.to_string(),
            form_string: form_string.to_string(),
            kind: icon_kind,
            two_handed,
            has_count,
            count,
            highlighted: false,
        }
    }

    /// Get the name of the item. Cloned string. Might be empty string.
    pub fn name(&self) -> String {
        self.name.clone()
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
