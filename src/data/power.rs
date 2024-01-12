use super::color::InvColor;
use super::keywords::*;
use super::{strings_to_enumset, HasIcon};
use crate::images::Icon;
use crate::plugin::Color;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PowerType {
    icon: Icon,
    color: InvColor,
}

impl PowerType {
    pub fn new(name: &str, tags: Vec<String>) -> Self {
        log::info!("{tags:?}");
        let kywds = strings_to_enumset::<SpellKeywords>(&tags);

        let icon = if let Some(found) = icon_for_tagset(&kywds) {
            found
        } else {
            log::debug!("Falling back to default icon for power; name='{name}'; keywords={tags:?}");
            Icon::Power
        };

        PowerType {
            icon,
            color: color_for_tagset(&kywds).unwrap_or_default(),
        }
    }
}

impl HasIcon for PowerType {
    fn color(&self) -> Color {
        self.color.color()
    }

    fn icon(&self) -> &Icon {
        &self.icon
    }
}
