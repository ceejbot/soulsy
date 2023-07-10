#pragma once
#include "handle/data/data_helper.h"
#include "handle/data/page/position_setting.h"
#include "handle/key_position_handle.h"
#include "handle/page_handle.h"

namespace processing {
    class set_setting_data {
    public:
        using position_type = handle::position_setting::position_type;
        using slot_type = handle::slot_setting::slot_type;

        static void read_and_set_data();
        static void set_new_item_count_if_needed(RE::TESBoundObject* a_object, int32_t a_count);
        static void set_single_slot(uint32_t a_page, position_type a_position, const std::vector<data_helper*>& a_data);
        static void set_queue_slot(position_type a_pos, const std::vector<data_helper*>& a_data);
        static void get_actives_and_equip();
        static void check_if_location_needs_block(RE::TESForm*& a_form, bool a_equipped);
        static void check_config_data();
        static void default_remove(RE::TESForm* a_form);

    private:
        static void set_empty_slot(int a_page, int a_pos, handle::key_position_handle*& a_key_pos);
        static void set_slot(uint32_t a_page,
            position_type a_position,
            const std::string& a_form,
            uint32_t a_type,
            uint32_t a_hand,
            uint32_t a_action,
            const std::string& a_form_left,
            uint32_t a_type_left,
            uint32_t a_action_left,
            RE::ActorValue a_actor_value,
            handle::key_position_handle*& a_key_pos,
            const std::string& a_section);
        static void set_new_item_count(RE::TESBoundObject* a_object, int32_t a_count);
        static void set_active_and_equip(handle::page_handle*& a_page_handle);
        static void process_config_data();
        static void write_empty_config_and_init_active();
        static void clear_hands();
        static void block_location(handle::position_setting* a_position_setting, bool a_condition);
        static void look_for_ammo(bool a_crossbow);
        //easier to have both, first only needed to get the page, position. the second, so it is easier to have the form
        static void do_cleanup(handle::position_setting*& a_position_setting, handle::slot_setting*& a_slot_setting);
        static bool clean_type_allowed(slot_type a_type);
    };
}
