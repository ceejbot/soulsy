#pragma once

namespace papyrus
{
	void on_config_close(RE::TESQuest*);
	RE::BSFixedString get_resolution_width(RE::TESQuest*);
	RE::BSFixedString get_resolution_height(RE::TESQuest*);

	bool Register(RE::BSScript::IVirtualMachine* a_vm);

	void register_papyrus_functions();
};
