pub mod v2 {
    use bincode::{Decode, Encode};
    use cxx::CxxVector;

    use crate::controller::cycles::CycleData;
    use crate::controller::equipset::EquipSet;
    use crate::data::base::BaseType;
    use crate::plugin::formSpecToHudItem;

    pub const VERSION: u32 = 2;

    pub fn deserialize(bytes: &CxxVector<u8>) -> Option<CycleData> {
        let bytes: Vec<u8> = bytes.iter().copied().collect();
        let config = bincode::config::standard();
        log::debug!(
            "reading cosave format version {VERSION}; data len={};",
            bytes.len()
        );

        match bincode::decode_from_slice::<CycleSerialized, _>(&bytes[..], config) {
            Ok((value, _len)) => {
                log::info!("Cycles successfully read from cosave data.");
                Some(value.into())
            }
            Err(e) => {
                log::error!("Bincode cannot decode the cosave data. len={}", bytes.len());
                log::error!("{e:?}");
                None
            }
        }
    }

    /// The serialization format is a list of form strings. Two drivers for
    /// this choice: 1) It's compact. 2) It can be deserialized into any
    /// Rust type we want, thus making it not care about implementation details.
    #[derive(Decode, Encode, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct CycleSerialized {
        left: Vec<String>,
        right: Vec<String>,
        power: Vec<String>,
        utility: Vec<String>,
        // Vec of tuples of (name, Vec<formspec>)
        equipsets: Vec<(String, Vec<String>)>,
        hud_visible: bool,
    }

    impl From<&CycleData> for CycleSerialized {
        fn from(value: &CycleData) -> Self {
            Self {
                left: value.left.to_vec(),
                right: value.right.to_vec(),
                power: value.power.to_vec(),
                utility: value.utility.to_vec(),
                equipsets: value
                    .equipsets
                    .iter()
                    .map(|xs| (xs.name(), xs.items.to_vec()))
                    .collect(),
                hud_visible: value.hud_visible,
            }
        }
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {
            fn filter_func(xs: &String) -> Option<String> {
                match xs.as_str() {
                    "health_proxy" => Some(xs.clone()),
                    "magicka_proxy" => Some(xs.clone()),
                    "stamina_proxy" => Some(xs.clone()),
                    "unarmed_proxy" => Some(xs.clone()),
                    "" => None,
                    _ => {
                        cxx::let_cxx_string!(form_spec = xs);
                        // Noting here that we do not go through the cache at all
                        // while loading these items. We probably should. TODO
                        let found = *formSpecToHudItem(&form_spec);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found.form_string())
                        }
                    }
                }
            }

            Self {
                left: value.left.iter().filter_map(filter_func).collect(),
                right: value.right.iter().filter_map(filter_func).collect(),
                power: value.power.iter().filter_map(filter_func).collect(),
                utility: value.utility.iter().filter_map(filter_func).collect(),
                hud_visible: value.hud_visible,
                equipsets: value
                    .equipsets
                    .iter()
                    .map(|xs| EquipSet::new(xs.0.clone(), xs.1.to_vec()))
                    .collect(),
                loaded: true,
            }
        }
    }
}

pub mod v1 {
    use bincode::{Decode, Encode};
    use cxx::CxxVector;

    use crate::controller::cycles::CycleData;
    use crate::data::base::BaseType;
    use crate::plugin::formSpecToHudItem;

    pub const VERSION: u32 = 1;

    pub fn deserialize(bytes: &CxxVector<u8>) -> Option<CycleData> {
        let bytes: Vec<u8> = bytes.iter().copied().collect();
        let config = bincode::config::standard();
        log::debug!(
            "reading cosave format version {VERSION}; data len={};",
            bytes.len()
        );

        match bincode::decode_from_slice::<CycleSerialized, _>(&bytes[..], config) {
            Ok((value, _len)) => {
                log::info!("Cycles successfully read from cosave data.");
                Some(value.into())
            }
            Err(e) => {
                log::error!("Bincode cannot decode the cosave data. len={}", bytes.len());
                log::error!("{e:?}");
                None
            }
        }
    }

    /// The serialization format is a list of form strings. Two drivers for
    /// this choice: 1) It's compact. 2) It can be deserialized into any
    /// representation of a TES form item we want, thus making it not care about
    /// implementation details of the hud item cache.
    #[derive(Decode, Encode, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct CycleSerialized {
        left: Vec<String>,
        right: Vec<String>,
        power: Vec<String>,
        utility: Vec<String>,
        hud_visible: bool,
    }

    impl From<&CycleData> for CycleSerialized {
        fn from(value: &CycleData) -> Self {
            Self {
                left: value.left.to_vec(),
                right: value.right.to_vec(),
                power: value.power.to_vec(),
                utility: value.utility.to_vec(),
                hud_visible: value.hud_visible,
            }
        }
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {
            fn filter_func(xs: &String) -> Option<String> {
                match xs.as_str() {
                    "health_proxy" => Some(xs.clone()),
                    "magicka_proxy" => Some(xs.clone()),
                    "stamina_proxy" => Some(xs.clone()),
                    "unarmed_proxy" => Some(xs.clone()),
                    "" => None,
                    _ => {
                        cxx::let_cxx_string!(form_spec = xs);
                        // Noting here that we do not go through the cache at all
                        // while loading these items. We probably should. TODO
                        let found = *formSpecToHudItem(&form_spec);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found.form_string())
                        }
                    }
                }
            }

            Self {
                left: value.left.iter().filter_map(filter_func).collect(),
                right: value.right.iter().filter_map(filter_func).collect(),
                power: value.power.iter().filter_map(filter_func).collect(),
                utility: value.utility.iter().filter_map(filter_func).collect(),
                hud_visible: value.hud_visible,
                equipsets: Vec::new(),
                loaded: true,
            }
        }
    }
}

pub mod v0 {
    use bincode::{Decode, Encode};
    use cxx::CxxVector;

    use crate::controller::cycles::CycleData;
    use crate::data::base::BaseType;
    use crate::plugin::formSpecToHudItem;

    const VERSION: u8 = 0;

    pub fn deserialize(bytes: &CxxVector<u8>) -> Option<CycleData> {
        let bytes: Vec<u8> = bytes.iter().copied().collect();
        let config = bincode::config::standard();
        log::debug!(
            "reading cosave format version {VERSION}; data len={};",
            bytes.len()
        );
        match bincode::decode_from_slice::<CycleSerialized, _>(&bytes[..], config) {
            Ok((value, _len)) => {
                log::info!("Cycles successfully read from cosave data.");
                Some(value.into())
            }
            Err(e) => {
                log::error!("Bincode cannot decode the cosave data. len={}", bytes.len());
                log::error!("{e:?}");
                None
            }
        }
    }

    #[derive(Decode, Encode, Hash, Debug, Clone, PartialEq, Eq)]
    pub struct CycleSerialized {
        left: Vec<ItemSerialized>,
        right: Vec<ItemSerialized>,
        power: Vec<ItemSerialized>,
        utility: Vec<ItemSerialized>,
        hud_visible: bool,
    }

    #[derive(Decode, Encode, Hash, Debug, Clone, Default, PartialEq, Eq)]
    pub struct ItemSerialized {
        name_bytes: Vec<u8>,
        form_string: String,
        kind: u8,
        two_handed: bool,
        has_count: bool,
        count: u32,
    }

    impl From<CycleSerialized> for CycleData {
        fn from(value: CycleSerialized) -> Self {
            fn filter_func(item: &ItemSerialized) -> Option<String> {
                let formstr = item.form_string.clone();
                match formstr.as_str() {
                    "health_proxy" => Some(formstr.clone()),
                    "magicka_proxy" => Some(formstr.clone()),
                    "stamina_proxy" => Some(formstr.clone()),
                    "unarmed_proxy" => Some(formstr.clone()),
                    "" => None,
                    _ => {
                        cxx::let_cxx_string!(form_spec = formstr);
                        let found = *formSpecToHudItem(&form_spec);
                        if matches!(found.kind(), BaseType::Empty) {
                            None
                        } else {
                            Some(found.form_string())
                        }
                    }
                }
            }

            Self {
                left: value.left.iter().filter_map(filter_func).collect(),
                right: value.right.iter().filter_map(filter_func).collect(),
                power: value.power.iter().filter_map(filter_func).collect(),
                utility: value.utility.iter().filter_map(filter_func).collect(),
                equipsets: Vec::new(),
                hud_visible: value.hud_visible,
                loaded: true,
            }
        }
    }
}
