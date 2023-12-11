#pragma once

namespace keycodes
{
	// This namespace has convenience functions for getting usable keycodes
	// out of game button events.

	enum : uint32_t
	{
		kInvalid        = static_cast<uint32_t>(-1),
		kKeyboardOffset = 0,
		kMouseOffset    = 256,
		kGamepadOffset  = 266
	};

	uint32_t keyID(const RE::ButtonEvent* button);

	uint32_t gamepadIndex(RE::BSWin32GamepadDevice::Key key);

}  // keycodes
