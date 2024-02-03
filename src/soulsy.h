#pragma once

// Forward declarations of Rust types used by C++.

namespace soulsy
{
	enum class Action : ::std::uint8_t;
	enum class Align : ::std::uint8_t;
	struct Color;
	struct EquippedData;
	struct HudItem;
	struct HudLayout;
	struct LoadedImage;
	struct Point;
	struct RelevantExtraData;
	struct SlotFlattened;
	struct SlotLayout;
	struct SpellData;
	struct TextFlattened;
}

using namespace soulsy;
