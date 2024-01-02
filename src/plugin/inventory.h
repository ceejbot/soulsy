#pragma once

inline const std::set<RE::FormType> RELEVANT_FORMTYPES_INVENTORY{
	RE::FormType::AlchemyItem,
	RE::FormType::Ammo,
	RE::FormType::Armor,
	RE::FormType::Light,
	RE::FormType::Scroll,
	RE::FormType::Weapon,
};

// Hooking the functions that let us listen for player inventory changes.
class PlayerHook
{
public:
	static void install();

private:
	static void notifyInventoryChanged(RE::TESForm* item_form);

	static void itemAdded(RE::Actor* a_this,
		RE::TESBoundObject* a_object,
		RE::ExtraDataList* a_extra_list,
		int32_t a_count,
		RE::TESObjectREFR* a_from_refr);
	static inline REL::Relocation<decltype(itemAdded)> add_object_to_container_;

	static void
		itemPickedUp(RE::Actor* a_this, RE::TESObjectREFR* a_object, uint32_t a_count, bool a_arg3, bool a_play_sound);
	static inline REL::Relocation<decltype(itemPickedUp)> pick_up_object_;

	static RE::ObjectRefHandle itemRemoved(RE::Actor* a_this,
		RE::TESBoundObject* a_item,
		std::int32_t a_count,
		RE::ITEM_REMOVE_REASON a_reason,
		RE::ExtraDataList* a_extra_list,
		RE::TESObjectREFR* a_move_to_ref,
		const RE::NiPoint3* a_drop_loc,
		const RE::NiPoint3* a_rotate);
	static inline REL::Relocation<decltype(itemRemoved)> remove_item_;


	static void
		add_item_functor(RE::TESObjectREFR* a_this, RE::TESObjectREFR* a_object, int32_t a_count, bool a4, bool a5);
	static inline REL::Relocation<decltype(add_item_functor)> add_item_functor_;
};
