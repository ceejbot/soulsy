#pragma once

#define IMGUI_DEFINE_MATH_OPERATORS
#define WIN32_LEAN_AND_MEAN
#define NOMINMAX

#include <RE/Skyrim.h>
#include <SKSE/SKSE.h>

#include <algorithm>
#include <cctype>
#include <d3d11.h>
#include <dxgi.h>
#include <imgui.h>
#include <imgui_impl_dx11.h>
#include <imgui_impl_win32.h>
#include <imgui_internal.h>
#include <locale>
#include <windows.h>
#include <winuser.h>

// Templates for sending C++ logging to Rust.
#include "log.h"

using namespace std::literals;

namespace stl
{
	using namespace SKSE::stl;

	template <class T>
	void write_thunk_call()
	{
		auto& trampoline = SKSE::GetTrampoline();
		const REL::Relocation<std::uintptr_t> hook{ T::id, T::offset };
		T::func = trampoline.write_call<5>(hook.address(), T::thunk);
	}
}

#define EXTERN_C extern "C"

#include "Version.h"
