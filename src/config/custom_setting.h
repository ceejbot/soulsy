#pragma once

// These are item lists, aka what I'm calling cycle data. To be deleted later.

namespace config
{
	class custom_setting
	{
	public:
		static void read_setting();

		static CSimpleIniA::TNamesDepend get_sections();

		static uint32_t get_page_by_section(const std::string& a_section);
		static uint32_t get_position_by_section(const std::string& a_section);
		static uint32_t get_type_by_section(const std::string& a_section);
		static std::string get_item_form_by_section(const std::string& a_section);
		static uint32_t get_slot_action_by_section(const std::string& a_section);
		static uint32_t get_hand_selection_by_section(const std::string& a_section);
		static int get_effect_actor_value(const std::string& a_section);
		static uint32_t get_type_left_by_section(const std::string& a_section);
		static std::string get_item_form_left_by_section(const std::string& a_section);
		static uint32_t get_slot_action_left_by_section(const std::string& a_section);

		static void reset_section(const std::string& a_section);

		static void write_slot_action_by_section(const std::string& a_section, uint32_t a_action);
		static void write_slot_action_left_by_section(const std::string& a_section, uint32_t a_action);

		static void write_section_setting(const std::string& a_section,
			uint32_t a_page,
			uint32_t a_position,
			uint32_t a_type,
			const std::string& a_form,
			uint32_t a_action,
			uint32_t a_hand,
			uint32_t a_type_left,
			const std::string& a_form_left,
			uint32_t a_action_left,
			int a_effect_actor_value);

	private:
		static void save_setting();
	};
}
