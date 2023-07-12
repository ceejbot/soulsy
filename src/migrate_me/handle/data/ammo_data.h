#pragma once
#include "include/image_path.h"

namespace handle
{
	class ammo_data
	{
	public:
		RE::TESForm* form            = nullptr;
		int32_t item_count           = 0;
		uint32_t button_press_modify = ui::draw_full;
		bool highlight_slot          = false;
	};
}
