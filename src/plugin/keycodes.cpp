#include "keycodes.h"

namespace keycodes
{
	uint32_t keyID(const RE::ButtonEvent* button)
	{
		uint32_t key = button->idCode;
		switch (button->device.get())
		{
			case RE::INPUT_DEVICE::kMouse: key += kMouseOffset; break;
			case RE::INPUT_DEVICE::kKeyboard: key += kKeyboardOffset; break;
			case RE::INPUT_DEVICE::kGamepad: key = gamepadIndex(static_cast<RE::BSWin32GamepadDevice::Key>(key)); break;
			case RE::INPUT_DEVICE::kNone:
			case RE::INPUT_DEVICE::kVirtualKeyboard:
			// case RE::INPUT_DEVICE::kVRRight:
			// case RE::INPUT_DEVICE::kVRLeft:
			case RE::INPUT_DEVICE::kTotal: break;
		}

		return key;
	}

	uint32_t gamepadIndex(const RE::BSWin32GamepadDevice::Key a_key)
	{
		using key = RE::BSWin32GamepadDevice::Key;

		uint32_t index;
		switch (a_key)
		{
			case key::kUp: index = 0; break;
			case key::kDown: index = 1; break;
			case key::kLeft: index = 2; break;
			case key::kRight: index = 3; break;
			case key::kStart: index = 4; break;
			case key::kBack: index = 5; break;
			case key::kLeftThumb: index = 6; break;
			case key::kRightThumb: index = 7; break;
			case key::kLeftShoulder: index = 8; break;
			case key::kRightShoulder: index = 9; break;
			case key::kA: index = 10; break;
			case key::kB: index = 11; break;
			case key::kX: index = 12; break;
			case key::kY: index = 13; break;
			case key::kLeftTrigger: index = 14; break;
			case key::kRightTrigger: index = 15; break;
			default:  // NOLINT(clang-diagnostic-covered-switch-default)
				index = kInvalid;
				break;
		}

		return index != kInvalid ? index + kGamepadOffset : kInvalid;
	}
}
