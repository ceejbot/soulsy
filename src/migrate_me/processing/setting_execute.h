#pragma once

#include "enums.h"
#include "handle/data/page/position_setting.h"
#include "handle/data/page/slot_setting.h"

namespace processing
{
	class setting_execute
	{
	public:
		using position_type = enums::position_type;
		using slot_type     = enums::slot_type;
		using action_type   = enums::action_type;

		static void activate(const std::vector<handle::slot_setting*>& a_slots,
			bool a_only_equip   = false,
			bool a_only_instant = false);
		static handle::position_setting* get_position_setting_for_key(uint32_t a_key);
		static void execute_ammo(const RE::TESForm* a_form);
		static void reequip_left_hand_if_needed(handle::position_setting* a_setting);

	private:
		static void execute_setting(handle::slot_setting*& a_slot, RE::PlayerCharacter*& a_player);
	};
}
