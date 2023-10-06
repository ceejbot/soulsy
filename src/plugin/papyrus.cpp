
#include "papyrus.h"
#include "constant.h"
#include "helpers.h"
#include "ui_renderer.h"

#include "lib.rs.h"

namespace papyrus
{
	static const char* MCM_NAME = "SoulsyHUD_MCM";

	void registerPapyrusFunctions()
	{
		const auto* papyrus = SKSE::GetPapyrusInterface();
		papyrus->Register(Register);
	}

	bool Register(RE::BSScript::IVirtualMachine* a_vm)
	{
		a_vm->RegisterFunction("OnConfigClose", MCM_NAME, handleConfigClose);
		a_vm->RegisterFunction("ClearCycles", MCM_NAME, handleClearCycles);
		a_vm->RegisterFunction("GetResolutionWidth", MCM_NAME, get_resolution_width);
		a_vm->RegisterFunction("GetResolutionHeight", MCM_NAME, get_resolution_height);

		a_vm->RegisterFunction("HandleCreateEquipSet", MCM_NAME, handleCreateEquipSet);
		a_vm->RegisterFunction("HandleRenameEquipSet", MCM_NAME, handleRenameEquipSet);
		a_vm->RegisterFunction("HandleUpdateEquipSet", MCM_NAME, handleUpdateEquipSet);
		a_vm->RegisterFunction("HandleRemoveEquipSet", MCM_NAME, handleRemoveEquipSet);
		a_vm->RegisterFunction("GetEquipSetNames", MCM_NAME, getEquipSetNames);
		a_vm->RegisterFunction("GetEquipSetIDs", MCM_NAME, getEquipSetIDs);

		a_vm->RegisterFunction("GetCycleNames", MCM_NAME, getCycleNames);
		a_vm->RegisterFunction("GetCycleFormIDs", MCM_NAME, getCycleFormIDs);

		a_vm->RegisterFunction("GetResolutionWidth", MCM_NAME, get_resolution_width);
		a_vm->RegisterFunction("GetResolutionHeight", MCM_NAME, get_resolution_height);

		logger::info("Registered papyrus functions for the MCM; classname {}."sv, MCM_NAME);
		return true;
	}

	void handleConfigClose(RE::TESQuest*) { refresh_user_settings(); }

	void handleClearCycles(RE::TESQuest*) { clear_cycles(); }

	RE::BSTArray<RE::BSFixedString> getEquipSetNames(RE::TESQuest*)
	{
		auto names = get_equipset_names();
		auto array = RE::BSTArray<RE::BSFixedString>();
		for (auto name : names) { array.push_back(std::string(name)); }

		return array;
	}

	RE::BSTArray<uint32_t> getEquipSetIDs(RE::TESQuest*)
	{
		auto ids = get_equipset_ids();
		auto array = RE::BSTArray<uint32_t>();
		for (auto id : ids) { array.push_back(id); }

		return array;
	}

	bool handleCreateEquipSet(RE::StaticFunctionTag*, RE::BSFixedString fixed)
	{
		auto name = std::string(fixed);
		return handle_create_equipset(name);
	}

	bool handleRenameEquipSet(RE::StaticFunctionTag*, uint32_t setnum, RE::BSFixedString fixed)
	{
		auto name = std::string(fixed);
		return handle_rename_equipset(setnum, name);
	}

	bool handleUpdateEquipSet(RE::StaticFunctionTag*, uint32_t setnum)
	{
		return handle_update_equipset(setnum);
	}

	bool handleRemoveEquipSet(RE::StaticFunctionTag*, uint32_t setnum)
	{
		return handle_remove_equipset(setnum);
	}

	RE::BSTArray<RE::BSFixedString> getCycleNames(RE::TESQuest*, int inWhich)
	{
		int which                     = std::clamp(inWhich, 0, 3);
		rust::Vec<rust::String> names = get_cycle_names(which);
		auto array                    = RE::BSTArray<RE::BSFixedString>();
		for (auto name : names) { array.push_back(std::string(name)); }

		return array;
	}

	RE::BSTArray<RE::BSFixedString> getCycleFormIDs(RE::TESQuest*, int inWhich)
	{
		int which                   = std::clamp(inWhich, 0, 3);
		rust::Vec<rust::String> ids = get_cycle_formids(which);
		auto array                  = RE::BSTArray<RE::BSFixedString>();
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
