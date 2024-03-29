#include "shouts.h"

#include "offset.h"
#include "player.h"

// For game implementation reasons, this also includes spells.
// Lesser powers are spells that go into the shout slot, IIUC.

namespace shouts
{
	bool has_shout(RE::Actor* a_actor, RE::TESShout* a_shout)
	{
		using func_t = decltype(&has_shout);
		REL::Relocation<func_t> func{ offset::has_shout };
		return func(a_actor, a_shout);
	}

	void unequip_spell(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::SpellItem* a_spell,
		uint32_t slot)
	{
		using func_t = decltype(&unequip_spell);
		const REL::Relocation<func_t> func{ REL::ID(offset::get_un_equip_spell) };
		func(a_vm, a_stack_id, a_actor, a_spell, slot);
	}

	void un_equip_shout(RE::BSScript::IVirtualMachine* a_vm,
		RE::VMStackID a_stack_id,
		RE::Actor* a_actor,
		RE::TESShout* a_shout)
	{
		using func_t = decltype(&un_equip_shout);
		const REL::Relocation<func_t> func{ REL::ID(offset::get_un_equipShout) };
		func(a_vm, a_stack_id, a_actor, a_shout);
	}

	void unequipShoutSlot(RE::PlayerCharacter*& player)
	{
		auto* selected_power = player->GetActorRuntimeData().selectedPower;
		if (selected_power)
		{
			auto* task = SKSE::GetTaskInterface();
			if (!task) return;
			rlog::trace("unequipping shout/power formID={:#08x};"sv, selected_power->formID);
			if (selected_power->Is(RE::FormType::Shout))
			{
				task->AddTask(
					[=]()
					{
						un_equip_shout(nullptr, 0, player, selected_power->As<RE::TESShout>());
						;
					});
			}
			else if (selected_power->Is(RE::FormType::Spell))
			{
				//power
				//2=other
				task->AddTask([=]() { unequip_spell(nullptr, 0, player, selected_power->As<RE::SpellItem>(), 2); });
			}
		}
	}

	void equipShoutByForm(RE::TESForm* form, RE::PlayerCharacter*& player)
	{
		// rlog::trace("tring to equip shout; name='{}';"sv, helpers::nameAsUtf8(form));
		if (const auto selected_power = player->GetActorRuntimeData().selectedPower; selected_power)
		{
			rlog::trace("current power:  name='{}'; is-shout={}; is-spell={};"sv,
				helpers::nameAsUtf8(selected_power),
				selected_power->Is(RE::FormType::Shout),
				selected_power->Is(RE::FormType::Spell));
			if (selected_power->formID == form->formID)
			{
				rlog::trace("shout already equipped; moving on."sv, helpers::nameAsUtf8(form));
				return;
			}
		}

		auto* task = SKSE::GetTaskInterface();
		if (!task) { return; }

		if (form->Is(RE::FormType::Spell))
		{
			rlog::debug("equipping lesser power name='{}';"sv, helpers::nameAsUtf8(form));
			auto* spell = form->As<RE::SpellItem>();
			if (!player->HasSpell(spell))
			{
				rlog::warn("player does not know lesser power; name='{}';"sv, helpers::nameAsUtf8(spell));
				return;
			}

			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipSpell(player, spell); });
			return;
		}

		auto* shout = form->As<RE::TESShout>();
		if (!has_shout(player, shout))
		{
			rlog::warn("player does not know shout; name='{}';"sv, helpers::nameAsUtf8(shout));
			return;
		}

		task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipShout(player, shout); });
		rlog::debug("shout equipped! name='{}'"sv, helpers::nameAsUtf8(form));
	}
}
