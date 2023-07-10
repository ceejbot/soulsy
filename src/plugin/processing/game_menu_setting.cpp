#include "game_menu_setting.h"
#include "../handle/page_handle.h"
#include "set_setting_data.h"
#include "include/user_settings.h"
#include "include/constant.h"
#include "include/helper.h"
#include "include/player.h"
#include "include/string_util.h"

namespace processing {
    void game_menu_setting::elden_souls_config(RE::TESForm* a_form, position_type a_position, bool a_overwrite) {
        if (!a_form) {
            logger::warn("form is null. return."sv);
            return;
        }

        std::vector<data_helper*> data;

        write_notification(fmt::format("Elden Souls Config, Position {}, overwrite {}"sv,
            static_cast<uint32_t>(a_position),
            static_cast<uint32_t>(a_overwrite)));
        const auto pos_max = handle::page_handle::get_singleton()->get_highest_page_id_position(a_position);
        auto max = config::MCMGlue::get_max_page_count() - 1;  //we start at 0 so count -1
        logger::trace("Max for Position {} is {}, already set before edit {}"sv,
            static_cast<uint32_t>(a_position),
            max,
            pos_max);
        if (pos_max != -1) {
            max = config::MCMGlue::get_max_page_count() - 1 - pos_max;
        }

        if (!a_overwrite && (data.size() == max || max == 0)) {
            write_notification(fmt::format("Can not add more Items to Position", max));
            logger::trace("Max is 0, can not add anymore, return.");
            return;
        }

        const auto check_duplicates = config::MCMGlue::get_check_duplicate_items();

        auto* item = is_suitable_for_position(a_form, a_position);
        if (item->form || (a_form && item->actor_value != RE::ActorValue::kNone)) {
            if (check_duplicates && already_used(a_form, a_position, data)) {
                auto log_string =
                    fmt::format("Item {} already used in that position", a_form ? a_form->GetName() : "null");
                write_notification(log_string);
                logger::trace("{}. return."sv, log_string);  //well
                return;
            } else {
                write_notification(fmt::format("Added Item {}", a_form ? a_form->GetName() : "null"));
                data.push_back(item);
            }
        } else {
            if (a_form && !a_form->Is(RE::FormType::Enchantment)) {
                write_notification(
                    fmt::format("Ignored Item {}, because it did not fit the requirement", a_form->GetName()));
            }
        }

        logger::trace("Size is {}. calling to set data now, overwrite is {}."sv,
            data.size(),
            static_cast<uint32_t>(a_overwrite));

        //use set_single_slot for replacement handling
        if (a_overwrite) {
            auto page = handle::page_handle::get_singleton()->get_active_page_id_position(a_position);
            processing::set_setting_data::set_single_slot(page, a_position, data);
        } else {
            processing::set_setting_data::set_queue_slot(a_position, data);
        }

        logger::trace("Setting done. return.");
    }
    void game_menu_setting::default_config(RE::TESForm*& a_form, position_type a_position_type, bool a_left) {
        if (!a_form) {
            logger::warn("form is null. return."sv);
            return;
        }

        const auto two_handed = util::helper::is_two_handed(a_form);
        if (two_handed && a_left) {
            auto log_string = fmt::format("Going to Ignore {}, because Two Handed {} and Left {}",
                a_form ? a_form->GetName() : "null",
                two_handed,
                a_left);
            write_notification(log_string);
            logger::trace("{}. return."sv, log_string);  //well
            return;
        }

        std::vector<data_helper*> data;
        const auto type = util::helper::get_type(a_form);
        const auto item = new data_helper();
        switch (type) {
            case slot_type::empty:
                item->form = nullptr;
                item->type = type;
                data.push_back(item);
                break;
            case slot_type::shout:
            case slot_type::power:
            case slot_type::armor:
            case slot_type::scroll:
            case slot_type::misc:
            case slot_type::lantern:
            case slot_type::mask:
                item->form = a_form;
                item->type = type;
                data.push_back(item);
                break;
            case slot_type::consumable:
                item->form = nullptr;
                item->type = type;
                item->actor_value = util::helper::get_actor_value_effect_from_potion(a_form);
                if (item->actor_value == RE::ActorValue::kNone) {
                    item->form = a_form;
                }
                data.push_back(item);
                break;
            case slot_type::weapon:
            case slot_type::magic:
            case slot_type::shield:
            case slot_type::light:
                item->form = a_form;
                item->left = a_left;
                item->type = type;
                item->action_type = handle::slot_setting::action_type::default_action;
                item->two_handed = two_handed;
                data.push_back(item);
                break;
        }

        for (const auto* data_item : data) {
            write_notification(fmt::format("Name {}, Type {}, Action {}, Left {}",
                data_item->form ? data_item->form->GetName() : "null",
                static_cast<uint32_t>(data_item->type),
                static_cast<uint32_t>(data_item->action_type),
                data_item->left));
        }

        std::vector<data_helper*> new_data;
        add_empty_data(new_data);
        auto* page_handle = handle::page_handle::get_singleton();
        auto page = page_handle->get_active_page_id();
        //for some types we need to check if there is something on the other hand
        if (type == slot_type::weapon || type == slot_type::magic || type == slot_type::shield ||
            type == slot_type::light) {
            if (!two_handed) {
                auto slot_settings = page_handle->get_page_setting(page, a_position_type)->slot_settings;

                std::vector<data_helper*> current_data;
                add_empty_data(current_data);

                auto current_two_handed = false;
                RE::TESForm* current_right = nullptr;
                RE::TESForm* current_left = nullptr;
                if (!slot_settings.empty()) {
                    current_right = slot_settings.front()->form;
                    current_two_handed = current_right && util::helper::is_two_handed(current_right);
                }
                if (slot_settings.size() == 2) {
                    current_left = slot_settings.at(1)->form;
                }

                logger::trace("got form {}, name {} on both/right hand"sv,
                    current_right ? util::string_util::int_to_hex(current_right->GetFormID()) : "null",
                    current_right ? current_right->GetName() : "null");

                logger::trace("got form {}, name {} on left hand"sv,
                    current_left ? util::string_util::int_to_hex(current_left->GetFormID()) : "null",
                    current_left ? current_left->GetName() : "null");

                if (current_two_handed && current_right) {
                    current_data[0]->form = current_right;
                    current_data[0]->left = false;
                    current_data[0]->type = slot_settings.front()->type;
                    current_data[0]->action_type = slot_settings.front()->action;
                    current_data.erase(current_data.begin() + 1);
                }

                if (!current_two_handed && current_right) {
                    if (slot_settings.front()->type == slot_type::weapon ||
                        slot_settings.front()->type == slot_type::magic ||
                        slot_settings.front()->type == slot_type::shield ||
                        slot_settings.front()->type == slot_type::light) {
                        current_data[0]->form = current_right;
                        current_data[0]->left = false;
                        current_data[0]->type = slot_settings.front()->type;
                        current_data[0]->action_type = slot_settings.front()->action;
                        current_data[0]->actor_value = slot_settings.front()->actor_value;
                    }
                    //will just keep it null
                }

                if (current_left) {
                    current_data[1]->form = current_left;
                    current_data[1]->left = true;
                    current_data[1]->type = slot_settings.at(1)->type;
                    current_data[1]->action_type = slot_settings.at(1)->action;
                }

                //should be nothing we need here, overwrite everything
                if (current_data.size() == 1 && !current_two_handed) {
                    new_data[0] = data[0];
                    const auto item2 = new data_helper();
                    item2->form = RE::TESForm::LookupByID(util::unarmed);
                    item2->left = !a_left;  //need the opposite
                    item2->type = slot_type::weapon;
                    item2->action_type = handle::slot_setting::action_type::default_action;
                    new_data[1] = item2;
                } else {
                    if (a_left) {
                        new_data[0] = current_data[0];
                        new_data[1] = data[0];
                    } else {
                        new_data[0] = data[0];
                        new_data[1] = current_data[1];
                    }
                }
            } else {
                new_data = data;
            }
        } else {
            new_data = data;
        }

        logger::trace("Size is {}. calling to set data now."sv, new_data.size());
        for (const auto* data_item : new_data) {
            logger::trace("Name {}, Type {}, Action {}, Left {}",
                data_item->form ? data_item->form->GetName() : "null",
                static_cast<uint32_t>(data_item->type),
                static_cast<uint32_t>(data_item->action_type),
                data_item->left);
        }
        //do things
        processing::set_setting_data::set_single_slot(page, a_position_type, new_data);
    }

    uint32_t game_menu_setting::get_selected_form(RE::UI*& a_ui) {
        uint32_t menu_form = 0;
        if (a_ui->IsMenuOpen(RE::InventoryMenu::MENU_NAME)) {
            auto* inventory_menu = static_cast<RE::InventoryMenu*>(a_ui->GetMenu(RE::InventoryMenu::MENU_NAME).get());
            if (inventory_menu) {
                RE::GFxValue result;
                //inventory_menu->uiMovie->SetPause(true);
                inventory_menu->uiMovie->GetVariable(&result,
                    "_root.Menu_mc.inventoryLists.itemList.selectedEntry.formId");
                if (result.GetType() == RE::GFxValue::ValueType::kNumber) {
                    menu_form = static_cast<std::uint32_t>(result.GetNumber());
                    logger::trace("formid {}"sv, util::string_util::int_to_hex(menu_form));
                }
            }
        }

        if (a_ui->IsMenuOpen(RE::MagicMenu::MENU_NAME)) {
            auto* magic_menu = static_cast<RE::MagicMenu*>(a_ui->GetMenu(RE::MagicMenu::MENU_NAME).get());
            if (magic_menu) {
                RE::GFxValue result;
                magic_menu->uiMovie->GetVariable(&result, "_root.Menu_mc.inventoryLists.itemList.selectedEntry.formId");
                if (result.GetType() == RE::GFxValue::ValueType::kNumber) {
                    menu_form = static_cast<std::uint32_t>(result.GetNumber());
                    logger::trace("formid {}"sv, util::string_util::int_to_hex(menu_form));
                }
            }
        }

        if (a_ui->IsMenuOpen(RE::FavoritesMenu::MENU_NAME)) {
            auto* favorite_menu = static_cast<RE::FavoritesMenu*>(a_ui->GetMenu(RE::FavoritesMenu::MENU_NAME).get());
            if (favorite_menu) {
                RE::GFxValue result;
                favorite_menu->uiMovie->GetVariable(&result, "_root.MenuHolder.Menu_mc.itemList.selectedEntry.formId");
                if (result.GetType() == RE::GFxValue::ValueType::kNumber) {
                    menu_form = static_cast<std::uint32_t>(result.GetNumber());
                    logger::trace("formid {}"sv, util::string_util::int_to_hex(menu_form));
                }
            }
        }

        return menu_form;
    }

    bool game_menu_setting::relevant_menu_open(RE::UI*& a_ui) {
        return a_ui->IsMenuOpen(RE::InventoryMenu::MENU_NAME) || a_ui->IsMenuOpen(RE::MagicMenu::MENU_NAME) ||
               a_ui->IsMenuOpen(RE::FavoritesMenu::MENU_NAME);
    }

    data_helper* game_menu_setting::is_suitable_for_position(RE::TESForm*& a_form,
        const handle::position_setting::position_type a_position) {
        //all kind of weapons and magic/spells
        const auto item = new data_helper();
        const auto type = util::helper::get_type(a_form);
        const auto two_handed = util::helper::is_two_handed(a_form);
        logger::trace("Item {}, is Type {}, TwoHanded {}"sv,
            a_form ? util::string_util::int_to_hex(a_form->formID) : "null",
            static_cast<uint32_t>(type),
            two_handed);

        switch (a_position) {
            case position_type::top:
                switch (type) {
                    case slot_type::power:
                    case slot_type::shout:
                        //case slot_type::misc:
                        item->form = a_form;
                        item->type = type;
                        item->two_handed = two_handed;
                        item->left = false;
                        item->action_type = util::helper::can_instant_cast(a_form, type) ?
                                                handle::slot_setting::action_type::instant :
                                                handle::slot_setting::action_type::default_action;
                        break;
                    case slot_type::magic:
                        if (util::helper::can_instant_cast(a_form, type)) {
                            item->form = a_form;
                            item->type = type;
                            item->two_handed = two_handed;
                            item->left = false;
                            item->action_type = handle::slot_setting::action_type::instant;
                        }
                        break;
                }
                break;
            case position_type::right:
                switch (type) {
                    case slot_type::weapon:
                    case slot_type::magic:
                        item->form = a_form;
                        item->type = type;
                        item->two_handed = two_handed;
                        item->left = false;
                        break;
                }
                break;
            case position_type::bottom:
                switch (type) {
                    case slot_type::consumable:
                        item->form = nullptr;
                        item->type = type;
                        item->two_handed = two_handed;
                        item->left = false;
                        item->actor_value = util::helper::get_actor_value_effect_from_potion(a_form);
                        if (item->actor_value == RE::ActorValue::kNone) {
                            item->form = a_form;
                        }
                        break;
                    case slot_type::lantern:  //not sure if best here
                    case slot_type::mask:
                        item->form = a_form;
                        item->type = type;
                        item->two_handed = two_handed;
                        item->left = false;
                        break;
                    case slot_type::scroll:
                        item->form = a_form;
                        item->type = type;
                        item->two_handed = two_handed;
                        item->left = false;
                        item->action_type = handle::slot_setting::action_type::instant;
                        break;
                }
                break;
            case position_type::left:
                switch (type) {
                    case slot_type::weapon:
                    case slot_type::magic:
                    case slot_type::shield:
                    case slot_type::light:
                        if (!two_handed) {
                            item->form = a_form;
                            item->type = type;
                            item->two_handed = two_handed;
                            item->left = true;
                            break;
                        }
                        break;
                }
                break;
            case position_type::total:
                break;
        }

        return item;
    }

    void game_menu_setting::write_notification(const std::string& a_string) { RE::DebugNotification(a_string.c_str()); }

    bool game_menu_setting::already_used(const RE::TESForm* a_form,
        const handle::position_setting::position_type a_position,
        const std::vector<data_helper*>& a_config_data) {
        if (!a_form) {
            return false;
        }
        //get pages and check for the form id in the position we are editing
        const auto* page_handle = handle::page_handle::get_singleton();

        uint32_t max_count = 1;
        uint32_t count = 0;
        if (a_form->IsWeapon() || a_form->IsArmor()) {
            //check item count in inventory
            max_count = util::player::get_inventory_count(a_form);
        }

        auto actor_value = RE::ActorValue::kNone;
        if (a_form->Is(RE::FormType::AlchemyItem)) {
            actor_value = util::helper::get_actor_value_effect_from_potion(const_cast<RE::TESForm*>(a_form));
        }

        auto pages = page_handle->get_pages();
        if (!pages.empty()) {
            for (auto& [page, page_settings] : pages) {
                for (auto [position, page_setting] : page_settings) {
                    if (position == a_position) {
                        for (const auto* setting : page_setting->slot_settings) {
                            if (setting &&
                                ((setting->form && setting->form->formID == a_form->formID) ||
                                    (setting->actor_value == actor_value && actor_value != RE::ActorValue::kNone))) {
                                count++;
                                if (max_count == count) {
                                    logger::trace("Item already {} time(s) used. return."sv, count);
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if (!a_config_data.empty()) {
            for (const auto* data_item : a_config_data) {
                if ((data_item->form && data_item->form->formID == a_form->formID) ||
                    (data_item->actor_value == actor_value && actor_value != RE::ActorValue::kNone)) {
                    count++;
                    if (max_count == count) {
                        logger::trace("Item already {} time(s) used. return."sv, count);
                        return true;
                    }
                }
            }
        }
        return false;
    }

    void game_menu_setting::add_empty_data(std::vector<data_helper*>& a_config_data) {
        const auto item_current = new data_helper();
        item_current->form = nullptr;
        item_current->left = false;
        item_current->type = slot_type::empty;
        item_current->action_type = handle::slot_setting::action_type::default_action;
        a_config_data.push_back(item_current);

        const auto item2_current = new data_helper();
        item2_current->form = nullptr;
        item2_current->left = true;
        item2_current->type = slot_type::empty;
        item2_current->action_type = handle::slot_setting::action_type::default_action;
        a_config_data.push_back(item2_current);
    }
}
