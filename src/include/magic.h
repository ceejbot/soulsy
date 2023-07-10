#pragma once

#include "enums.h"

namespace equip
{
	class magic
	{
	public:
		using action_type = enums::action_type;

		static void cast_magic(RE::TESForm* a_form,
			action_type a_action,
			const RE::BGSEquipSlot* a_slot,
			RE::PlayerCharacter*& a_player);
		static void cast_scroll(const RE::TESForm* a_form, action_type a_action, RE::PlayerCharacter*& a_player);
		static void equip_or_cast_power(RE::TESForm* a_form, action_type a_action, RE::PlayerCharacter*& a_player);
		static void equip_shout(RE::TESForm* a_form, RE::PlayerCharacter*& a_player);

	private:
		static RE::MagicSystem::CastingSource get_casting_source(const RE::BGSEquipSlot* a_slot);
		static bool can_dual_cast(float a_cost, float a_magicka, float a_multiplier);
		static void flash_hud_meter(RE::ActorValue a_actor_value);
		static void send_spell_casting_sound_alert(RE::MagicCaster* a_magic_caster, RE::SpellItem* a_spell_item);
	};
}
