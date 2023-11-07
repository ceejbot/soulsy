#pragma once

namespace ui
{
	constexpr auto HUD_NAME  = "soulsy_hud";
	constexpr auto draw_full = 255;

	static std::string icon_directory                = R"(.\Data\SKSE\Plugins\resources\icons)";
	static std::string img_directory                 = R"(.\Data\SKSE\Plugins\resources\backgrounds)";
	static std::string highlight_animation_directory = R"(.\Data\SKSE\Plugins\resources\animations\highlight)";

	enum class image_type
	{
		hud,
		slot,
		key,
		total
	};

	inline static std::map<std::string, image_type> ImageFileToType = { { R"(hud_bg.svg)", image_type::hud },
		{ R"(slot_bg.svg)", image_type::slot },
		{ R"(key_bg.svg)", image_type::key } };
}
