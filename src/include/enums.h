#pragma once

namespace enums
{
	enum class slot_type : std::uint32_t
	{
		weapon     = 0,
		magic      = 1,
		shield     = 2,
		shout      = 3,
		power      = 4,
		consumable = 5,
		armor      = 6,
		scroll     = 7,
		empty      = 8,
		misc       = 9,
		light      = 10,
		lantern    = 11,
		mask       = 12
	};

	enum class action_type : std::uint32_t
	{
		default_action = 0,
		instant        = 1,
		un_equip       = 2
	};

	enum class hand_equip : std::uint32_t
	{
		single = 0,
		both   = 1,
		total  = 2
	};

	// can get rid of this eventually
	enum class position_type : std::uint32_t
	{
		top    = 0,
		right  = 1,
		bottom = 2,
		left   = 3,
		total  = 4
	};


}
