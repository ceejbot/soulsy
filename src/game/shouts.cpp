#include "shouts.h"

#include "offset.h"
#include "player.h"

// For game implementation reasons, this also includes spells.
// Lesser powers are spells that go into the shout slot, IIUC.

namespace game
{
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
			rlog::trace("unequipping shout/power formID={};"sv, rlog::formatAsHex(selected_power->formID));
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
		// rlog::trace("tring to equip shout; name='{}';"sv, form->GetName());
		if (const auto selected_power = player->GetActorRuntimeData().selectedPower; selected_power)
		{
			rlog::trace("current power:  name='{}'; is-shout={}; is-spell={};"sv,
				selected_power->GetName(),
				selected_power->Is(RE::FormType::Shout),
				selected_power->Is(RE::FormType::Spell));
			if (selected_power->formID == form->formID)
			{
				rlog::trace("shout already equipped; moving on."sv, form->GetName());
				return;
			}
		}

		auto* task = SKSE::GetTaskInterface();
		if (!task) { return; }

		if (form->Is(RE::FormType::Spell))
		{
			rlog::debug("equipping lesser power name='{}';"sv, form->GetName());
			auto* spell = form->As<RE::SpellItem>();
			if (!player->HasSpell(spell))
			{
				rlog::warn("player does not know lesser power; name='{}';"sv, spell->GetName());
				return;
			}

			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipSpell(player, spell); });
			return;
		}

		auto* shout = form->As<RE::TESShout>();
		if (!player::has_shout(player, shout))
		{
			rlog::warn("player does not know shout; name='{}';"sv, shout->GetName());
			return;
		}

		task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipShout(player, shout); });
		rlog::debug("shout equipped! name='{}'"sv, form->GetName());
	}
}
