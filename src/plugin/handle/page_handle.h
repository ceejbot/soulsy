#pragma once
#include "./data/page/position_setting.h"
#include "./key_position_handle.h"
#include "include/enums.h"
#include "include/helper.h"
#include "include/image_path.h"

namespace handle
{
	class page_handle
	{
	public:
		using hand_equip    = enums::hand_equip;
		using position_type = enums::position_type;
		using slot_type     = enums::slot_type;
		using icon_type     = ui::icon_image_type;

		static page_handle* get_singleton();
		void init_page(uint32_t a_page,
			position_type a_position,
			const std::vector<helpers::data_helper*>& data_helpers,
			hand_equip a_hand,
			key_position_handle*& a_key_pos);
		void init_actives(uint32_t a_page, position_type a_position);
		void set_active_page(uint32_t a_page) const;
		void set_active_page_position(uint32_t a_page, position_type a_pos) const;
		void set_highest_page_position(int a_page, position_type a_pos) const;
		[[nodiscard]] position_setting* get_page_setting(uint32_t a_page, position_type a_position) const;
		[[nodiscard]] std::map<uint32_t, std::map<position_type, position_setting*>> get_pages() const;
		[[nodiscard]] std::map<position_type, position_setting*> get_active_page() const;
		[[nodiscard]] uint32_t get_active_page_id() const;
		[[nodiscard]] uint32_t get_next_page_id() const;
		[[nodiscard]] uint32_t get_active_page_id_position(position_type a_position) const;
		[[nodiscard]] uint32_t get_next_page_id_position(position_type a_position) const;
		[[nodiscard]] uint32_t get_next_non_empty_setting_for_position(position_type a_position) const;
		//int for now, because also 0 can be unset
		[[nodiscard]] int get_highest_page_id_position(position_type a_position) const;

		page_handle(const page_handle&) = delete;
		page_handle(page_handle&&)      = delete;

		page_handle& operator=(const page_handle&) const = delete;
		page_handle& operator=(page_handle&&) const      = delete;

	private:
		page_handle() : data_(nullptr) {}
		~page_handle() = default;

		static void get_offset_values(position_type a_position,
			float a_setting_x,
			float a_setting_y,
			float& offset_x,
			float& offset_y);
		static void get_equip_slots(slot_type a_type, hand_equip a_hand, RE::BGSEquipSlot*& a_slot, bool a_left);
		static void get_item_count(RE::TESForm*& a_form, int32_t& a_count, slot_type a_type);
		static void get_item_icon(RE::TESForm*& a_form, icon_type& a_icon);
		static void get_consumable_icon_by_actor_value(RE::ActorValue& a_actor_value, icon_type& a_icon);
		static void get_consumable_item_count(RE::ActorValue& a_actor_value, int32_t& a_count);

		struct page_handle_data
		{
			std::map<uint32_t, std::map<position_type, position_setting*>> page_settings;
			uint32_t active_page = 0;
			std::map<position_type, uint32_t> active_page_per_position;
			std::map<position_type, int> highest_set_page_per_position;
		};

		page_handle_data* data_;
	};
}
