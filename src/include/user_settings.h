#pragma once

namespace config
{
	class mcm_setting
	{
	public:
		static void read_setting();

		static uint32_t get_top_action_key();
		static uint32_t get_right_action_key();
		static uint32_t get_bottom_action_key();
		static uint32_t get_left_action_key();
		static uint32_t get_toggle_key();
		static uint32_t get_show_hide_key();
		static bool get_key_press_to_enter_edit();
		static uint32_t get_edit_key();
		static uint32_t get_left_or_overwrite_edit_key();
		static uint32_t get_remove_key();
		static bool get_bottom_execute_key_combo_only();
		static uint32_t get_controller_set();

		static float get_hud_image_scale_width();
		static float get_hud_image_scale_height();
		static float get_hud_image_position_width();
		static float get_hud_image_position_height();
		static float get_hud_slot_position_offset_x();
		static float get_hud_slot_position_offset_y();
		static float get_hud_key_position_offset();
		static float get_icon_scale_width();
		static float get_icon_scale_height();
		static float get_key_icon_scale_width();
		static float get_key_icon_scale_height();
		static float get_hud_arrow_image_scale_width();
		static float get_hud_arrow_image_scale_height();
		static float get_arrow_icon_scale_width();
		static float get_arrow_icon_scale_height();
		[[maybe_unused]] static float get_master_scale();
		static float get_arrow_slot_count_text_offset();
		static float get_slot_count_text_offset();
		static bool get_draw_toggle_button();
		static float get_toggle_key_offset_x();
		static float get_toggle_key_offset_y();
		static float get_current_items_offset_x();
		static float get_current_items_offset_y();
		static float get_slot_item_name_offset_horizontal_x();
		static float get_slot_item_name_offset_horizontal_y();
		static float get_slot_item_name_offset_vertical_x();
		static float get_slot_item_name_offset_vertical_y();
		static float get_arrow_slot_offset_x();
		static float get_arrow_slot_offset_y();
		static float get_current_shout_offset_x();
		static float get_current_shout_offset_y();

		static uint32_t get_background_transparency();
		static uint32_t get_background_icon_transparency();
		static uint32_t get_icon_transparency();
		static uint32_t get_key_transparency();
		static uint32_t get_current_items_transparency();
		static uint32_t get_current_shout_transparency();
		static uint32_t get_slot_count_transparency();
		static uint32_t get_slot_item_name_transparency();
		static uint32_t get_icon_transparency_blocked();
		static float get_slot_count_text_font_size();
		static float get_current_items_font_size();
		static float get_arrow_count_font_size();
		static uint32_t get_current_items_red();
		static uint32_t get_current_items_green();
		static uint32_t get_current_items_blue();
		static uint32_t get_slot_count_red();
		static uint32_t get_slot_count_green();
		static uint32_t get_slot_count_blue();
		static uint32_t get_slot_item_red();
		static uint32_t get_slot_item_green();
		static uint32_t get_slot_item_blue();

		static uint32_t get_slot_button_feedback();
		static bool get_draw_current_items_text();
		static bool get_draw_item_name_text();
		static bool get_draw_current_shout_text();
		static float get_current_shout_font_size();
		static float get_item_name_font_size();
		static bool get_draw_page_id();

		static uint32_t get_alpha_slot_animation();
		static float get_duration_slot_animation();

		static bool get_action_check();
		static bool get_empty_hand_setting();
		static bool get_hide_outside_combat();
		static float get_fade_timer_outside_combat();
		static bool get_disable_input_quick_loot();
		static bool get_elden_demon_souls();
		static uint32_t get_max_page_count();
		static uint32_t get_max_ammunition_type();
		static bool get_check_duplicate_items();
		static bool get_un_equip_ammo();
		static bool get_only_favorite_ammo();
		static bool get_prevent_consumption_of_last_dynamic_potion();
		static bool get_group_potions();
		static float get_potion_min_perfect();
		static float get_potion_max_perfect();
		static bool get_disable_re_equip_of_actives();
		static bool get_sort_arrow_by_quantity();
		static bool get_overwrite_poison_dose();
		static uint32_t get_apply_poison_dose();
		static bool get_try_dual_cast_top_spell();

		static bool get_auto_cleanup();
		static bool get_clean_armor();
		static bool get_clean_weapon();
		static bool get_clean_spell();
		static bool get_clean_alchemy_item();
		static bool get_clean_shout();
		static bool get_clean_light();
		static bool get_clean_scroll();
	};
}
