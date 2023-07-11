#pragma once

class PapyrusGlue
{
public:
	static void on_config_close(RE::TESQuest*);
	static RE::BSFixedString get_resolution_width(RE::TESQuest*);
	static RE::BSFixedString get_resolution_height(RE::TESQuest*);

	static bool Register(RE::BSScript::IVirtualMachine* a_vm);
};

void register_papyrus_functions();
