use super::controller::Action;

// TODO: This should move to settings.
static MAX_CYCLE_LEN: usize = 10;

/*

class slot_setting {
public:
    //un equip just makes sense with form == nullptr
    enum class action_type : std::uint32_t { default_action = 0, instant = 1, un_equip = 2 };

    enum class hand_equip : std::uint32_t { single = 0, both = 1, total = 2 };


    RE::TESForm* form = nullptr;
    slot_type type = slot_type::empty;
    action_type action = action_type::default_action;
    hand_equip equip = hand_equip::total;
    RE::BGSEquipSlot* equip_slot = nullptr;
    int32_t item_count = 0;
    RE::ActorValue actor_value = RE::ActorValue::kNone;
    bool display_item_count = false;
};
 */

#[derive(Debug, Clone, PartialEq, Eq)]
enum ItemKind {
    Empty,
    Weapon,
    Magic,
    Shield,
    Shout,
    Power,
    Consumable,
    Armor,
    Scroll,
    Misc,
    Light,
    Lantern,
    Torch,
    Mask,
}

impl Default for ItemKind {
    fn default() -> Self {
        ItemKind::Empty
    }
}

// Haven't yet figured out how to serialize this to toml or anything yet.
// Still working on what data I want to track.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CycleEntry {
    spec: String,
    kind: ItemKind,
    // form: &TESForm,
    // slot: &BGSEquipSlot
}

#[derive(Debug, Clone, Default)]
pub struct CycleData {
    left: Vec<CycleEntry>,
    right: Vec<CycleEntry>,
    power: Vec<CycleEntry>,
    utility: Vec<CycleEntry>,
}

pub enum ToggleResults {
    Added,
    Removed,
    Inappropriate,
    TooManyItems,
}

impl CycleData {
    pub fn advance(&mut self, which: Action, amount: usize) -> Option<CycleEntry> {
        let cycle = match which {
            Action::Power => &mut self.power,
            Action::Left => &mut self.left,
            Action::Right => &mut self.right,
            Action::Utility => &mut self.utility,
            _ => {
                log::warn!("It is a programmer error to call advance() with {which:?}");
                return None;
            }
        };
        if cycle.is_empty() {
            return None;
        }
        cycle.rotate_left(amount);
        if let Some(next) = cycle.first() {
            Some(next.clone())
        } else {
            None
        }
    }

    pub fn toggle(&mut self, which: Action, item: CycleEntry) -> ToggleResults {
        let cycle = match which {
            Action::Power => {
                if item.kind != ItemKind::Power && item.kind != ItemKind::Shout {
                    return ToggleResults::Inappropriate;
                }
                &mut self.power
            }
            Action::Left => {
                match item.kind {
                    ItemKind::Weapon => {}
                    ItemKind::Magic => {}
                    ItemKind::Shield => {}
                    ItemKind::Scroll => {}
                    ItemKind::Light => {}
                    ItemKind::Lantern => {}
                    ItemKind::Torch => {}
                    _ => {
                        return ToggleResults::Inappropriate;
                    }
                }
                &mut self.left
            }
            Action::Right => {
                match item.kind {
                    ItemKind::Weapon => {}
                    ItemKind::Magic => {}
                    ItemKind::Scroll => {}
                    _ => {
                        return ToggleResults::Inappropriate;
                    }
                }
                &mut self.right
            }
            Action::Utility => {
                match item.kind {
                    ItemKind::Consumable => {}
                    ItemKind::Armor => {}
                    ItemKind::Misc => {}
                    ItemKind::Mask => {}
                    _ => {
                        return ToggleResults::Inappropriate;
                    }
                }
                &mut self.utility
            }
            _ => {
                log::warn!("It is a programmer error to call toggle() with {which:?}");
                return ToggleResults::Inappropriate;
            }
        };

        // We have at most 10 items, so we can do this with a linear search.
        if let Some(idx) = cycle.iter().position(|xs| *xs == item) {
            cycle.remove(idx);
            ToggleResults::Removed
        } else if cycle.len() >= MAX_CYCLE_LEN {
            return ToggleResults::TooManyItems;
        } else {
            cycle.push(item);
            ToggleResults::Added
        }
    }
}
