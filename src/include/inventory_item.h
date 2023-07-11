#pragma once

#include "enums.h"
#include "image_path.h"

#include "lib.rs.h"

namespace inventory_item
{
	using slot_type = enums::slot_type;
    using icon_type = ui::icon_image_type;

    rust::Box<CycleEntry> cycle_entry_from_form(RE::TESForm*& item_form);

	slot_type get_type(RE::TESForm*& item_form);
	bool is_two_handed(RE::TESForm*& item_form);
	bool can_instant_cast(RE::TESForm* item_form, slot_type item_type);

    ui::icon_image_type get_icon_type(const slot_type item_type, RE::TESForm*& item_form);

    void get_weapon_type_icon(RE::TESForm*& form, icon_type& icon_img_type);
    void get_spell_icon(RE::TESForm*& form, icon_type& icon_img_type);
    void get_consumable_icon(RE::TESForm*& form, icon_type& icon_img_type);
    void get_armor_icon(RE::TESForm*& form, icon_type& icon_img_type);
    void get_consumable_icon_by_actor_value(RE::ActorValue& actor_value, icon_type& icon_img_type);
}
