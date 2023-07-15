#pragma once

#include "enums.h"

// Shouts, spells, scrolls, minor powers.
namespace magic
{
	using action_type = enums::action_type;

	void cast_magic(RE::TESForm* a_form,
		action_type a_action,
		const RE::BGSEquipSlot* a_slot,
		RE::PlayerCharacter*& a_player);
	void cast_scroll(const RE::TESForm* a_form, action_type a_action, RE::PlayerCharacter*& a_player);
	void equip_or_cast_power(RE::TESForm* a_form, action_type a_action, RE::PlayerCharacter*& a_player);

	void equipShout(RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void unequipShoutSlot(RE::PlayerCharacter*& a_player);


	RE::MagicSystem::CastingSource get_casting_source(const RE::BGSEquipSlot* a_slot);
	bool can_dual_cast(float a_cost, float a_magicka, float a_multiplier);
	void flash_hud_meter(RE::ActorValue a_actor_value);
	void send_spell_casting_sound_alert(RE::MagicCaster* a_magic_caster, RE::SpellItem* a_spell_item);

	// Implementation details.
	void unequip_spell(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::SpellItem* a_spell,
		uint32_t a_slot);
	void un_equip_shout(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::TESShout* a_shout);
}
