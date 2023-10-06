#pragma once

namespace papyrus
{
	void handleConfigClose(RE::TESQuest*);
	void handleClearCycles(RE::TESQuest*);

	RE::BSTArray<RE::BSFixedString> getCycleNames(RE::TESQuest*, int which);
	RE::BSTArray<RE::BSFixedString> getCycleFormIDs(RE::TESQuest*, int which);

	RE::BSFixedString get_resolution_width(RE::TESQuest*);
	RE::BSFixedString get_resolution_height(RE::TESQuest*);


	bool handleCreateEquipSet(RE::StaticFunctionTag*, RE::BSFixedString name);
	bool handleRenameEquipSet(RE::StaticFunctionTag*, uint32_t setnum, RE::BSFixedString name);
	bool handleUpdateEquipSet(RE::StaticFunctionTag*, uint32_t setnum);
	bool handleRemoveEquipSet(RE::StaticFunctionTag*, uint32_t setnum);
	RE::BSTArray<RE::BSFixedString> getEquipSetNames(RE::TESQuest*);
	RE::BSTArray<uint32_t> getEquipSetIDs(RE::TESQuest*);

	bool buildIsPreAE(RE::TESQuest*);
	void setIsPreAEBuild(bool input);

	bool Register(RE::BSScript::IVirtualMachine* a_vm);
	void registerPapyrusFunctions();
};
