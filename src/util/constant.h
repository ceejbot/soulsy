#pragma once

namespace util
{
	constexpr auto dynamic_name = "dynamic";
	constexpr auto delimiter    = "|";

	constexpr RE::FormID unarmed = 0x000001F4;

	inline static std::map<RE::ActorValue, RE::FormID> actor_value_to_base_potion_map_ = { { RE::ActorValue::kHealth,
																							   0x0003EADE },
		{ RE::ActorValue::kStamina, 0x00039BE8 },
		{ RE::ActorValue::kMagicka, 0x0003EAE1 } };

	constexpr RE::FormID bound_arrow = 0x0010b0a7;
}
