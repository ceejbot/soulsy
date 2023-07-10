#pragma once

class MCMGlue
{
public:
	static void on_config_close(RE::TESQuest*);
	static RE::BSFixedString get_resolution_width(RE::TESQuest*);
	static RE::BSFixedString get_resolution_height(RE::TESQuest*);

	static std::vector<RE::BSFixedString> get_section_names(RE::TESQuest*, uint32_t a_position);
	static RE::BSFixedString get_page(RE::TESQuest*, uint32_t a_index, uint32_t a_position);
	static RE::BSFixedString get_position(RE::TESQuest*, uint32_t a_index, uint32_t a_position);
	static uint32_t get_selection_type(RE::TESQuest*, uint32_t a_index, bool a_left, uint32_t a_position);
	static RE::BSFixedString get_form_string(RE::TESQuest*, uint32_t a_index, bool a_left, uint32_t a_position);
	static uint32_t get_slot_action(RE::TESQuest*, uint32_t a_index, bool a_left, uint32_t a_position);
	static uint32_t get_hand_selection(RE::TESQuest*, uint32_t a_index, uint32_t a_position);
	static RE::BSFixedString get_form_name(RE::TESQuest*, uint32_t a_index, bool a_left, uint32_t a_position);
	static void reset_section(RE::TESQuest*, uint32_t a_index, uint32_t a_position);
	static void set_action_value(RE::TESQuest*, uint32_t a_index, bool a_left, uint32_t a_value, uint32_t a_position);
	static std::vector<RE::BSFixedString> get_config_files(RE::TESQuest*, bool a_elden);
	static RE::BSFixedString get_active_config(RE::TESQuest*, bool a_elden);
	static void set_config(RE::TESQuest*, bool a_elden, RE::BSFixedString a_name);
	static void set_active_config(RE::TESQuest*, bool a_elden, uint32_t a_index);
	static void add_unarmed_setting(RE::TESQuest*, uint32_t a_position);
	static RE::BSFixedString get_actor_value(RE::TESQuest*, uint32_t a_index, uint32_t a_position);

	static bool Register(RE::BSScript::IVirtualMachine* a_vm);

private:
	static bool is_size_ok(uint32_t a_idx, uint64_t a_size);
	static std::string get_section_by_index(uint32_t a_index, uint32_t a_position);
	static bool check_name(const std::string& a_name);
	static std::vector<std::string> search_for_config_files(bool a_elden);
	static std::string get_form_name_string_for_section(const std::string& a_str);
};

void registerGlue();
