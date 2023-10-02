
#include "papyrus.h"
#include "constant.h"
#include "helpers.h"
#include "ui_renderer.h"

#include "lib.rs.h"

namespace papyrus
{
	static const char* mcm_name = "SoulsyHUD_MCM";

	void registerPapyrusFunctions()
	{
		const auto* papyrus = SKSE::GetPapyrusInterface();
		papyrus->Register(Register);
	}

	bool Register(RE::BSScript::IVirtualMachine* a_vm)
	{
		a_vm->RegisterFunction("OnConfigClose", mcm_name, handleConfigClose);
		a_vm->RegisterFunction("ClearCycles", mcm_name, handleClearCycles);

		a_vm->RegisterFunction("GetCycleNames", mcm_name, getCycleNames);
		a_vm->RegisterFunction("GetCycleFormIDs", mcm_name, getCycleFormIDs);

		a_vm->RegisterFunction("GetResolutionWidth", mcm_name, get_resolution_width);
		a_vm->RegisterFunction("GetResolutionHeight", mcm_name, get_resolution_height);

		logger::info("Registered papyrus functions for the MCM; classname {}."sv, mcm_name);
		return true;
	}

	void handleConfigClose(RE::TESQuest*) { refresh_user_settings(); }

	void handleClearCycles(RE::TESQuest*) { clear_cycles(); }

	RE::BSTArray<RE::BSFixedString> getCycleNames(RE::TESQuest*, int inWhich)
	{
		int which = std::clamp(inWhich, 0, 3);
		rust::Vec<rust::String> names = get_cycle_names(which);
		auto array = RE::BSTArray<RE::BSFixedString>();
		for (auto name : names) { array.push_back(std::string(name)); }

		return array;
	}

	RE::BSTArray<RE::BSFixedString> getCycleFormIDs(RE::TESQuest*, int inWhich)
	{
		int which = std::clamp(inWhich, 0, 3);
		rust::Vec<rust::String> ids = get_cycle_formids(which);
		auto array = RE::BSTArray<RE::BSFixedString>();
		for (auto id : ids) { array.push_back(std::string(id)); }

		return array;
	}

	RE::BSFixedString get_resolution_width(RE::TESQuest*)
	{
		return fmt::format(FMT_STRING("{:.2f}"), ui::resolutionWidth());
	}

	RE::BSFixedString get_resolution_height(RE::TESQuest*)
	{
		return fmt::format(FMT_STRING("{:.2f}"), ui::resolutionHeight());
	}

}
