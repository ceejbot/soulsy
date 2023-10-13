#pragma once

namespace papyrus
{
	void handleConfigClose(RE::TESQuest*);
	void handleClearCycles(RE::TESQuest*);

	RE::BSTArray<RE::BSFixedString> getCycleNames(RE::TESQuest*, int which);
	RE::BSTArray<RE::BSFixedString> getCycleFormIDs(RE::TESQuest*, int which);

	RE::BSFixedString get_resolution_width(RE::TESQuest*);
	RE::BSFixedString get_resolution_height(RE::TESQuest*);

	int stringToInt(RE::TESQuest*, RE::BSFixedString number);

	bool handleCreateEquipSet(RE::TESQuest*, RE::BSFixedString name);
	bool handleRenameEquipSet(RE::TESQuest*, uint32_t setnum, RE::BSFixedString name);
	bool handleUpdateEquipSet(RE::TESQuest*, uint32_t setnum);
	bool handleRemoveEquipSet(RE::TESQuest*, uint32_t setnum);
	RE::BSTArray<RE::BSFixedString> getEquipSetNames(RE::TESQuest*);
	RE::BSTArray<RE::BSFixedString> getEquipSetIDs(RE::TESQuest*);
	int findSelectedSetByName(RE::TESQuest*, RE::BSFixedString name);
	RE::BSTArray<RE::BSFixedString> getEquipSetItemNames(RE::TESQuest*, uint32_t id);
	bool setItemAsEquipSetIcon(RE::TESQuest*, uint32_t id, RE::BSFixedString fixed);

	bool Register(RE::BSScript::IVirtualMachine* a_vm);
	void registerPapyrusFunctions();
};
