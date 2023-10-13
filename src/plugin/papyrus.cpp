
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
		a_vm->RegisterFunction("GetEquipSetItemNames", MCM_NAME, getEquipSetItemNames);
		a_vm->RegisterFunction("SetItemAsEquipSetIcon", MCM_NAME, setItemAsEquipSetIcon);
		a_vm->RegisterFunction("FindSelectedSetID", MCM_NAME, findSelectedSetByName);

		a_vm->RegisterFunction("StringToInt", MCM_NAME, stringToInt);

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

	RE::BSTArray<RE::BSFixedString> getEquipSetIDs(RE::TESQuest*)
	{
		auto ids   = get_equipset_ids();
		auto array = RE::BSTArray<RE::BSFixedString>();
		for (auto id : ids) { array.push_back(std::string(id)); }

		return array;
	}

	int stringToInt(RE::TESQuest*, RE::BSFixedString number)
	{
		auto numstr = std::string(number);
		return string_to_int(numstr);
	}

	int equipSetIndexToID(RE::TESQuest*, RE::BSFixedString idx)
	{
		// Menus return string values. That value is an index into the name array.
		// We would like to turn that into an integer equip set ID.
		auto indexstr = std::string(idx);
		return equipset_index_to_id(indexstr);
	}

	int findSelectedSetByName(RE::TESQuest*, RE::BSFixedString name)
	{
		return look_up_equipset_by_name(std::string(name));
	}

	RE::BSTArray<RE::BSFixedString> getEquipSetItemNames(RE::TESQuest*, uint32_t id)
	{
		auto names = get_equipset_item_names(id);
		auto array = RE::BSTArray<RE::BSFixedString>();
		for (auto name : names) { array.push_back(std::string(name)); }

		return array;
	}

	bool setItemAsEquipSetIcon(RE::TESQuest*, uint32_t id, RE::BSFixedString fixed)
	{
		return set_equipset_icon(id, std::string(fixed));
	}

	bool handleCreateEquipSet(RE::TESQuest*, RE::BSFixedString fixed)
	{
		auto name = std::string(fixed);
		return handle_create_equipset(name);
	}

	bool handleRenameEquipSet(RE::TESQuest*, uint32_t id, RE::BSFixedString fixed)
	{
		auto name = std::string(fixed);
		logger::debug("handleRenameEquipSet(): id={}; new name='{}';", id, name);
		return handle_rename_equipset(id, name);
	}

	bool handleUpdateEquipSet(RE::TESQuest*, uint32_t id) { return handle_update_equipset(id); }

	bool handleRemoveEquipSet(RE::TESQuest*, uint32_t id) { return handle_remove_equipset(id); }

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
