#pragma once

namespace papyrus
{
	void handleConfigClose(RE::TESQuest*);
	void handleClearCycles(RE::TESQuest*);
	
	RE::BSTArray<RE::BSFixedString> getCycleNames(RE::TESQuest*, int which);
	RE::BSTArray<RE::BSFixedString> getCycleFormIDs(RE::TESQuest*, int which);

	RE::BSFixedString get_resolution_width(RE::TESQuest*);
	RE::BSFixedString get_resolution_height(RE::TESQuest*);

	bool handleSaveEquipSet(RE::StaticFunctionTag*, std::string* name, uint32_t setnum);
	bool handleRemoveEquipSet(RE::StaticFunctionTag*, std::string* name, uint32_t setnum);
	bool buildIsPreAE(RE::TESQuest*);
	void setIsPreAEBuild(bool input);

	bool Register(RE::BSScript::IVirtualMachine* a_vm);

	void registerPapyrusFunctions();
};
