#pragma once

namespace keycodes
{
	// This namespace has convenience functions for getting usable keycodes
	// out of game button events.

	enum : uint32_t
	{
		k_invalid         = static_cast<uint32_t>(-1),
		k_keyboard_offset = 0,
		k_mouse_offset    = 256,
		k_gamepad_offset  = 266
	};

	uint32_t get_key_id(const RE::ButtonEvent* button);

	uint32_t get_gamepad_index(RE::BSWin32GamepadDevice::Key key);

}  // keycodes
