#pragma once

#include "keycodes.h"

namespace hooks
{
	void install_hooks();

	class MenuHook : public RE::MenuControls
	{
	public:
		static void install();

	private:
		RE::BSEventNotifyControl process_event(RE::InputEvent** a_event, RE::BSTEventSource<RE::InputEvent*>* a_source);

		using process_event_type =
			decltype(static_cast<RE::BSEventNotifyControl (RE::MenuControls::*)(RE::InputEvent* const*,
					RE::BSTEventSource<RE::InputEvent*>*)>(&RE::MenuControls::ProcessEvent));
		static inline REL::Relocation<process_event_type> process_event_;
	};

	class PlayerHook
	{
	public:
		static void install();

	private:
		static void add_object_to_container(RE::Actor* a_this,
			RE::TESBoundObject* a_object,
			RE::ExtraDataList* a_extra_list,
			int32_t a_count,
			RE::TESObjectREFR* a_from_refr);
		static inline REL::Relocation<decltype(add_object_to_container)> add_object_to_container_;

		static void pick_up_object(RE::Actor* a_this,
			RE::TESObjectREFR* a_object,
			uint32_t a_count,
			bool a_arg3,
			bool a_play_sound);
		static inline REL::Relocation<decltype(pick_up_object)> pick_up_object_;

		static RE::ObjectRefHandle remove_item(RE::Actor* a_this,
			RE::TESBoundObject* a_item,
			std::int32_t a_count,
			RE::ITEM_REMOVE_REASON a_reason,
			RE::ExtraDataList* a_extra_list,
			RE::TESObjectREFR* a_move_to_ref,
			const RE::NiPoint3* a_drop_loc,
			const RE::NiPoint3* a_rotate);
		static inline REL::Relocation<decltype(remove_item)> remove_item_;


		static void
			add_item_functor(RE::TESObjectREFR* a_this, RE::TESObjectREFR* a_object, int32_t a_count, bool a4, bool a5);
		static inline REL::Relocation<decltype(add_item_functor)> add_item_functor_;
	};

}
