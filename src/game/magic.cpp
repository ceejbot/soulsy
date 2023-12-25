#include "magic.h"

#include "RE/A/Actor.h"
#include "gear.h"
#include "offset.h"
#include "player.h"

namespace game
{
	void cast_magic(RE::TESForm* a_form,
		action_type a_action,
		const RE::BGSEquipSlot* a_slot,
		RE::PlayerCharacter*& player)
	{
		auto left = a_slot == game::left_hand_equip_slot();
		rlog::trace(
			"try to work spell {}, action {}, left {}"sv, a_form->GetName(), static_cast<uint32_t>(a_action), left);

		if (!a_form->Is(RE::FormType::Spell))
		{
			rlog::warn("object {} is not a spell. return."sv, a_form->GetName());
			return;
		}

		auto* spell = a_form->As<RE::SpellItem>();

		if (!player->HasSpell(spell))
		{
			rlog::warn("player does not have spell {}. return."sv, spell->GetName());
			return;
		}

		//maybe check if the spell is already equipped
		auto casting_type = spell->GetCastingType();
		rlog::trace("spell {} is type {}"sv, spell->GetName(), static_cast<uint32_t>(casting_type));
		if (a_action == action_type::instant && casting_type != RE::MagicSystem::CastingType::kConcentration)
		{
			if (true)
			{
				auto selected_power = player->GetActorRuntimeData().selectedPower;
				if (selected_power)
				{
					rlog::warn(
						"power/shout {} is equipped, will only cast spell in elden mode if shout slot is empty. return."sv,
						selected_power->GetName());
					RE::DebugNotification("Shout Slot not Empty, Skipping Spellcast");
					return;
				}
			}
			auto* actor  = player->As<RE::Actor>();
			auto* caster = actor->GetMagicCaster(get_casting_source(a_slot));

			//might cost nothing if nothing has been equipped into tha hands after start, so it seems
			auto cost = spell->CalculateMagickaCost(actor);
			rlog::trace("spell cost for {} is {}"sv, spell->GetName(), fmt::format(FMT_STRING("{:.2f}"), cost));

			auto current_magicka = actor->AsActorValueOwner()->GetActorValue(RE::ActorValue::kMagicka);
			auto dual_cast       = false;
			if (!spell->IsTwoHanded())
			{
				auto* game_setting             = RE::GameSettingCollection::GetSingleton();
				auto dual_cast_cost_multiplier = game_setting->GetSetting("fMagicDualCastingCostMult")->GetFloat();
				rlog::trace("dual cast, multiplier {}"sv, fmt::format(FMT_STRING("{:.2f}"), dual_cast_cost_multiplier));
				dual_cast = can_dual_cast(cost, current_magicka, dual_cast_cost_multiplier);
				if (dual_cast)
				{
					cost = cost * dual_cast_cost_multiplier;
					caster->SetDualCasting(true);
				}
			}
			rlog::trace("got temp magicka {}, cost {}, can dual cast {}"sv, current_magicka, cost, dual_cast);

			if (current_magicka < cost)
			{
				if (!RE::UI::GetSingleton()->GetMenu<RE::HUDMenu>())
				{
					rlog::warn("Will not flash HUD Menu, because I could not find it.");
				}
				else { flash_hud_meter(RE::ActorValue::kMagicka); }
				rlog::warn("not enough magicka for spell {}, magicka {}, cost {} return."sv,
					a_form->GetName(),
					current_magicka,
					cost);
				return;
			}

			actor->AsActorValueOwner()->RestoreActorValue(
				RE::ACTOR_VALUE_MODIFIER::kDamage, RE::ActorValue::kMagicka, -cost);

			//could trigger an animation here
			//might need to set some things
			//player->NotifyAnimationGraph("IdleMagic_01"); //works
			auto is_self_target = spell->GetDelivery() == RE::MagicSystem::Delivery::kSelf;
			auto* target        = is_self_target ? actor : actor->GetActorRuntimeData().currentCombatTarget.get().get();

			auto magnitude     = 1.f;
			auto effectiveness = 1.f;
			if (auto* effect = spell->GetCostliestEffectItem()) { magnitude = effect->GetMagnitude(); }
			rlog::trace("casting spell {}, magnitude {}, effectiveness {}"sv,
				spell->GetName(),
				fmt::format(FMT_STRING("{:.2f}"), magnitude),
				fmt::format(FMT_STRING("{:.2f}"), effectiveness));
			caster->CastSpellImmediate(
				spell, false, target, effectiveness, false, magnitude, is_self_target ? nullptr : actor);
			//tested with adamant, works with the silent casting perk as well
			send_spell_casting_sound_alert(caster, spell);
		}
		else
		{
			const auto* obj_right = player->GetActorRuntimeData().currentProcess->GetEquippedRightHand();
			const auto* obj_left  = player->GetActorRuntimeData().currentProcess->GetEquippedLeftHand();
			if (left && obj_left && obj_left->formID == spell->formID)
			{
				rlog::debug(
					"Object Left {} is already where it should be already equipped. return."sv, spell->GetName());
				return;
			}
			if (!left && obj_right && obj_right->formID == spell->formID)
			{
				rlog::debug(
					"Object Right {} is already where it should be already equipped. return."sv, spell->GetName());
				return;
			}

			rlog::trace("calling equip spell {}, left {}"sv, spell->GetName(), left);
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipSpell(player, spell, a_slot); });
			}
		}

		rlog::trace("worked spell {}, action {}. return."sv, a_form->GetName(), static_cast<uint32_t>(a_action));
	}

	void cast_scroll(const RE::TESForm* form, action_type a_action, RE::PlayerCharacter*& player)
	{
		rlog::trace("start casting scroll; name='{}'; action {}"sv, form->GetName(), static_cast<uint32_t>(a_action));

		if (!form->Is(RE::FormType::Scroll))
		{
			rlog::warn("object {} is not a scroll. return."sv, form->GetName());
			return;
		}

		RE::TESBoundObject* obj      = nullptr;
		RE::ExtraDataList* extraData = nullptr;
		auto item_count              = boundObjectForForm(form, obj, extraData);

		if (!obj || item_count == 0)
		{
			rlog::warn("scroll not found in inventory"sv);
			return;
		}

		if (a_action == action_type::instant)
		{
			auto* actor  = player->As<RE::Actor>();
			auto* scroll = obj->As<RE::ScrollItem>();
			actor->GetMagicCaster(RE::MagicSystem::CastingSource::kInstant)
				->CastSpellImmediate(scroll, false, actor, 1.0f, false, 0.0f, nullptr);
			actor->RemoveItem(scroll, 1, RE::ITEM_REMOVE_REASON::kRemove, nullptr, nullptr);
		}
		else
		{
			auto* task = SKSE::GetTaskInterface();
			if (task)
			{
				task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipObject(player, obj, extraData); });
			}
		}

		rlog::trace("used scroll {}, action {}. return."sv, form->GetName(), static_cast<uint32_t>(a_action));
	}

	void equip_or_cast_power(RE::TESForm* a_form, action_type a_action, RE::PlayerCharacter*& player)
	{
		rlog::trace("try to work power {}, action {}"sv, a_form->GetName(), static_cast<uint32_t>(a_action));

		if (!a_form->Is(RE::FormType::Spell))
		{
			rlog::warn("object {} is not a spell. return."sv, a_form->GetName());
			return;
		}

		if (const auto* selected_power = player->GetActorRuntimeData().selectedPower;
			selected_power && a_action != action_type::instant)
		{
			rlog::trace("current selected power is {}, is shout {}, is spell {}"sv,
				selected_power->GetName(),
				selected_power->Is(RE::FormType::Shout),
				selected_power->Is(RE::FormType::Spell));
			if (selected_power->formID == a_form->formID)
			{
				rlog::debug("no need to equip power {}, it is already equipped. return."sv, a_form->GetName());
				return;
			}
		}

		auto* spell = a_form->As<RE::SpellItem>();
		if (!player->HasSpell(spell))
		{
			rlog::warn("player does not have spell {}. return."sv, spell->GetName());
			return;
		}

		auto* task = SKSE::GetTaskInterface();
		if (task)
		{
			task->AddTask([=]() { RE::ActorEquipManager::GetSingleton()->EquipSpell(player, spell); });
		}

		rlog::trace("worked power {} action {}. return."sv, a_form->GetName(), static_cast<uint32_t>(a_action));
	}

	RE::MagicSystem::CastingSource get_casting_source(const RE::BGSEquipSlot* a_slot)
	{
		if (a_slot == game::right_hand_equip_slot()) { return RE::MagicSystem::CastingSource::kRightHand; }
		if (a_slot == game::left_hand_equip_slot()) { return RE::MagicSystem::CastingSource::kLeftHand; }
		return RE::MagicSystem::CastingSource::kOther;
	}

	bool can_dual_cast(float a_cost, float a_magicka, float a_multiplier)
	{
		if ((a_cost * a_multiplier) < a_magicka) { return true; }
		return false;
	}

	void flash_hud_meter(RE::ActorValue a_actor_value)
	{
		static REL::Relocation<decltype(flash_hud_meter)> flash_hud_meter{ REL::ID(offset::get_flash_hud_meter) };
		return flash_hud_meter(a_actor_value);
	}

	void send_spell_casting_sound_alert(RE::MagicCaster* a_magic_caster, RE::SpellItem* a_spell_item)
	{
		static REL::Relocation<decltype(send_spell_casting_sound_alert)> send_spell_casting_sound_alert{ REL::ID(
			offset::send_spell_casting_sound_alert) };
		return send_spell_casting_sound_alert(a_magic_caster, a_spell_item);
	}

}
