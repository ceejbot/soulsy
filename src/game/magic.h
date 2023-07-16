#pragma once

// TODO: Rewrite if adding a cast-from-hud feature. Otherwise remove.
namespace magic
{
	enum class action_type : std::uint32_t
	{
		default_action = 0,
		instant        = 1,
		un_equip       = 2
	};

	void cast_magic(RE::TESForm* a_form,
		action_type a_action,
		const RE::BGSEquipSlot* a_slot,
		RE::PlayerCharacter*& a_player);
	void cast_scroll(const RE::TESForm* a_form, action_type a_action, RE::PlayerCharacter*& a_player);
	void equip_or_cast_power(RE::TESForm* a_form, action_type a_action, RE::PlayerCharacter*& a_player);


	RE::MagicSystem::CastingSource get_casting_source(const RE::BGSEquipSlot* a_slot);
	bool can_dual_cast(float a_cost, float a_magicka, float a_multiplier);
	void flash_hud_meter(RE::ActorValue a_actor_value);
	void send_spell_casting_sound_alert(RE::MagicCaster* a_magic_caster, RE::SpellItem* a_spell_item);
}
