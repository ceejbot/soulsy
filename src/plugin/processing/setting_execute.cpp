#include "setting_execute.h"
#include "include/gear.h"
#include "include/utility_items.h"
#include "include/magic.h"
#include "handle/data/page/position_setting.h"
#include "handle/key_position_handle.h"
#include "handle/page_handle.h"
#include "include/user_settings.h"
#include "include/constant.h"
#include "include/string_util.h"

namespace processing {
    using mcm = config::MCMGlue;

    void setting_execute::activate(const std::vector<handle::slot_setting*>& a_slots,
        bool a_only_equip,
        bool a_only_instant) {
        if (a_slots.empty()) {
            logger::info("settings are empty. return.");
            return;
        }
        logger::trace("got {} settings execute, only_equip {}, only_instant {}"sv,
            a_slots.size(),
            a_only_equip,
            a_only_instant);
        std::vector<RE::BGSEquipSlot*> un_equip;
        auto* player = RE::PlayerCharacter::GetSingleton();
        for (auto* slot : a_slots) {
            if (!slot->form && slot->type == slot_type::consumable && slot->actor_value != RE::ActorValue::kNone) {
                logger::debug("form is null, but actor value is set to {}"sv, static_cast<int>(slot->actor_value));
            } else if (mcm::get_elden_demon_souls() && !slot->form) {
                logger::debug("form is null and I am in elden mode, skipping."sv);
                continue;
            } else if (!slot->form && slot->type != slot_type::empty) {
                logger::warn("form is null and not type empty, skipping."sv);
                continue;
            }

            if ((!slot->form && slot->type == slot_type::empty && slot->action == action_type::un_equip) ||
                (slot->form && slot->form->formID == util::unarmed)) {
                un_equip.push_back(slot->equip_slot);
            }

            if (mcm::get_elden_demon_souls() && a_only_equip && slot->action != action_type::default_action) {
                logger::trace("form {} does not need equip, skipping"sv,
                    slot->form ? util::string_util::int_to_hex(slot->form->GetFormID()) : "null");
                equip::equip_slot::un_equip_shout_slot(player);
                continue;
            }

            if (mcm::get_elden_demon_souls() && a_only_instant && slot->action != action_type::instant) {
                logger::trace("form {} does not need any work, skipping"sv,
                    slot->form ? util::string_util::int_to_hex(slot->form->GetFormID()) : "null");
                continue;
            }

            logger::trace("executing setting for type {}, action {}, form {}, left {} ..."sv,
                static_cast<uint32_t>(slot->type),
                static_cast<uint32_t>(slot->action),
                slot->form ? util::string_util::int_to_hex(slot->form->GetFormID()) : "null",
                slot->equip_slot == equip::equip_slot::get_left_hand_slot());
            execute_setting(slot, player);
        }

        if (!un_equip.empty()) {
            for (RE::BGSEquipSlot*& slot : un_equip) {
                equip::equip_slot::un_equip_hand(slot, player, action_type::un_equip);
            }
        }
    }

    handle::position_setting* setting_execute::get_position_setting_for_key(const uint32_t a_key) {
        const auto position = handle::key_position_handle::get_singleton()->get_position_for_key(a_key);
        if (position == position_type::total) {
            logger::warn("nothing to do, nothing set. return."sv);
            return nullptr;
        }

        const auto* page_handle = handle::page_handle::get_singleton();
        handle::position_setting* position_setting;
        uint32_t page;
        if (mcm::get_elden_demon_souls()) {
            page = page_handle->get_active_page_id_position(position);
            position_setting = page_handle->get_page_setting(page, position);
        } else {
            page = page_handle->get_active_page_id();
            position_setting = page_handle->get_page_setting(page, position);
        }
        if (!position_setting) {
            logger::warn("nothing to do, nothing set. return."sv);
        }
        logger::debug("page {}, position is {}, setting count {}"sv,
            page,
            static_cast<uint32_t>(position),
            position_setting->slot_settings.size());

        return position_setting;
    }

    void setting_execute::execute_ammo(const RE::TESForm* a_form) {
        if (a_form) {
            auto* player = RE::PlayerCharacter::GetSingleton();
            equip::item::equip_ammo(a_form, player);
        }
    }

    void setting_execute::reequip_left_hand_if_needed(handle::position_setting* a_setting) {
        if (!a_setting) {
            return;
        }
        logger::trace("checking and calling re equip for setting {}, is setting empty {}"sv,
            static_cast<uint32_t>(a_setting->position),
            a_setting->slot_settings.empty());
        auto* left_slot = equip::equip_slot::get_left_hand_slot();
        auto* equip_manager = RE::ActorEquipManager::GetSingleton();
        auto* player = RE::PlayerCharacter::GetSingleton();
        equip::equip_slot::un_equip_object_ft_dummy_dagger(left_slot, player, equip_manager);
        if (!a_setting->slot_settings.empty()) {
            processing::setting_execute::activate(a_setting->slot_settings);
        }
    }

    void setting_execute::execute_setting(handle::slot_setting*& a_slot, RE::PlayerCharacter*& a_player) {
        switch (a_slot->type) {
            case slot_type::consumable:
                if (a_slot->form) {
                    equip::item::consume_potion(a_slot->form, a_player);
                } else if (a_slot->actor_value != RE::ActorValue::kNone) {
                    equip::item::find_and_consume_fitting_option(a_slot->actor_value, a_player);
                }
                break;
            case slot_type::magic:
                equip::magic::magic::cast_magic(a_slot->form, a_slot->action, a_slot->equip_slot, a_player);
                break;
            case slot_type::shout:
                equip::magic::equip_shout(a_slot->form, a_player);
                break;
            case slot_type::power:
                equip::magic::equip_or_cast_power(a_slot->form, a_slot->action, a_player);
                break;
            case slot_type::weapon:
            case slot_type::shield:
            case slot_type::light:
                equip::item::equip_item(a_slot->form, a_slot->equip_slot, a_player, a_slot->type);
                break;
            case slot_type::armor:
            case slot_type::lantern:
            case slot_type::mask:
                equip::item::equip_armor(a_slot->form, a_player);
                break;
            case slot_type::scroll:
                equip::magic::cast_scroll(a_slot->form, a_slot->action, a_player);
                break;
            case slot_type::misc:
                //TODO
                logger::warn("ignoring misc-item."sv);
                break;
            case slot_type::empty:
                equip::equip_slot::un_equip_hand(a_slot->equip_slot, a_player, a_slot->action);
                break;
        }
    }
}
