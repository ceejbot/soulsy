#pragma once

namespace papyrus
{
	void handleConfigClose(RE::TESQuest*);
	void handleClearCycles(RE::TESQuest*);
	RE::BSTArray<RE::BSFixedString> getPowerCycleNames(RE::TESQuest*);
	RE::BSTArray<RE::BSFixedString> getUtilityCycleNames(RE::TESQuest*);
	RE::BSTArray<RE::BSFixedString> getLeftCycleNames(RE::TESQuest*);
	RE::BSTArray<RE::BSFixedString> getRightCycleNames(RE::TESQuest*);

	RE::BSFixedString get_resolution_width(RE::TESQuest*);
	RE::BSFixedString get_resolution_height(RE::TESQuest*);

	bool buildIsPreAE(RE::TESQuest*);
	void setIsPreAEBuild(bool input);

	bool Register(RE::BSScript::IVirtualMachine* a_vm);

	void registerPapyrusFunctions();
};
