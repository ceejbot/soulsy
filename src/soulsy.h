#pragma once

// Forward declarations of Rust types used by C++.

namespace soulsy
{
	enum class Action : ::std::uint8_t;
	struct HudItem;
	struct EquippedData;
struct SpellData;
struct Color;

}

using namespace soulsy;
