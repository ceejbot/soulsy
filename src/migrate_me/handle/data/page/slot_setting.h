#pragma once
#include "include/enums.h"

namespace handle
{
	class slot_setting
	{
	public:
		RE::TESForm* form            = nullptr;
		enums::slot_type type        = enums::slot_type::empty;
		enums::action_type action    = enums::action_type::default_action;
		enums::hand_equip equip      = enums::hand_equip::total;
		RE::BGSEquipSlot* equip_slot = nullptr;
		int32_t item_count           = 0;
		RE::ActorValue actor_value   = RE::ActorValue::kNone;
		bool display_item_count      = false;
	};
}
