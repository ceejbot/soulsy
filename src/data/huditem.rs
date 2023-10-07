use std::ffi::CString;
use std::fmt::Display;

use super::base::BaseType;
use super::icons::Icon;
use super::{HasIcon, IsHudItem};
use crate::plugin::{Color, ItemCategory};

/// A TESForm item that the player can use or equip, with the data
/// that drives the HUD cached for fast access.
#[derive(Hash, Debug, Clone, Default, PartialEq, Eq)]
pub struct HudItem {
    /// Player-visible name as its underlying bytes.
    name_bytes: Vec<u8>,
    /// a bit saying if we had to lose data to encode the name
    name_is_utf8: bool,
    /// Name as utf8
    name: String,
    /// A string that can be turned back into form data; for serializing.
    form_string: String,
    /// An enum classifying this item for fast question-answering as well as icon selection.
    kind: BaseType,
    /// Cached count from inventory data. Relies on hooks to be updated.
    count: u32,
}

impl HudItem {
    pub fn from_keywords(
        category: ItemCategory,
        keywords: Vec<String>,
        name_bytes: Vec<u8>,
        form_string: String,
        count: u32,
        twohanded: bool,
    ) -> Self {
        let (name_is_utf8, name) = name_from_bytes(&name_bytes);

        // log::debug!("calling BaseType::classify() with keywords={keywords:?};");
        let kind: BaseType = BaseType::classify(name.as_str(), category, keywords, twohanded);
        Self {
            name_bytes,
            name,
            name_is_utf8,
            form_string,
            count,
            kind,
        }
    }

    pub fn preclassified(
        name_bytes: Vec<u8>,
        form_string: String,
        count: u32,
        kind: BaseType,
    ) -> Self {
        let (name_is_utf8, name) = name_from_bytes(&name_bytes);
        Self {
            name_bytes,
            name,
            name_is_utf8,
            form_string,
            count,
            kind,
        }
    }

    pub fn for_equip_set(name: String, id: u32, icon: Icon) -> Self {
        Self {
            name_bytes: name.as_bytes().to_vec(),
            name,
            name_is_utf8: true,
            form_string: format!("equipset_{id}"),
            count: 1,
            kind: BaseType::Equipset(icon),
        }
    }

    pub fn make_unarmed_proxy() -> Self {
        HudItem::preclassified(
            "Unarmed".as_bytes().to_vec(),
            "unarmed_proxy".to_string(),
            1,
            BaseType::HandToHand,
        )
    }

    pub fn icon(&self) -> &Icon {
        self.kind().icon()
    }

    pub fn icon_file(&self) -> String {
        self.kind().icon().icon_file()
    }

    pub fn icon_fallback(&self) -> String {
        self.kind().icon_fallback().icon_file()
    }

    pub fn color(&self) -> Color {
        self.kind.color()
    }

    pub fn kind(&self) -> &BaseType {
        &self.kind
    }

    pub fn form_string(&self) -> String {
        self.form_string.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn name_is_utf8(&self) -> bool {
        self.name_is_utf8
    }

    pub fn name_bytes(&self) -> Vec<u8> {
        self.name_bytes.clone()
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn set_count(&mut self, v: u32) {
        self.count = v
    }

    pub fn count_matters(&self) -> bool {
        self.kind.count_matters()
    }

    pub fn two_handed(&self) -> bool {
        self.kind.two_handed()
    }

    pub fn is_magic(&self) -> bool {
        self.kind.is_magic()
    }
}

impl Display for HudItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name='{}'; form_string='{}'; kind='{:?}'; count={};",
            self.name(),
            self.form_string(),
            self.kind(),
            self.count()
        )
    }
}

fn name_from_bytes(name_bytes: &[u8]) -> (bool, String) {
    // let's try to get a name string out of the bytes
    let mut name_is_utf8 = false;
    let cstring = match CString::from_vec_with_nul(name_bytes.to_owned()) {
        Ok(cstring) => cstring,
        Err(e) => {
            if let Ok(cstring) = CString::new(name_bytes.to_owned()) {
                cstring
            } else {
                log::info!("Surprising: item name bytes were an invalid C string; error: {e:#}");
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

    (name_is_utf8, name)
}
