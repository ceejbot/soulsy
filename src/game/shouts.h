#pragma once

namespace game
{
	void equipShoutByForm(RE::TESForm* a_form, RE::PlayerCharacter*& a_player);
	void unequipShoutSlot(RE::PlayerCharacter*& a_player);

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
