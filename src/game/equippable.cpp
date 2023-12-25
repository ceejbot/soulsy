#include "equippable.h"

#include "gear.h"
#include "helpers.h"
#include "player.h"

#include "lib.rs.h"

namespace equippable
{
	bool requiresTwoHands(RE::TESForm*& item_form)
	{
		if (!item_form) { return false; }
		if (item_form->Is(RE::FormType::Spell))
		{
			const auto* spell = item_form->As<RE::SpellItem>();
			return spell && spell->IsTwoHanded();
		}

		if (item_form->IsWeapon())
		{
			const auto* weapon = item_form->As<RE::TESObjectWEAP>();
			return weapon &&
			       (weapon->IsTwoHandedAxe() || weapon->IsTwoHandedSword() || weapon->IsBow() || weapon->IsCrossbow());
		}

		if (item_form->Is(RE::FormType::Scroll))
		{
			auto* scroll = item_form->As<RE::ScrollItem>();
			return scroll && scroll->IsTwoHanded();
		}

		return false;
	}

	RE::ActorValue getPotionEffect(RE::TESForm* a_form, bool a_check)
	{
		if (!a_form->Is(RE::FormType::AlchemyItem)) { return RE::ActorValue::kNone; }

		auto* alchemy_potion = a_form->As<RE::AlchemyItem>();
		if (alchemy_potion->IsFood() || alchemy_potion->IsPoison()) { return RE::ActorValue::kNone; }

		const auto* effect = alchemy_potion->GetCostliestEffectItem()->baseEffect;
		auto actor_value   = effect->GetMagickSkill();
		if (actor_value == RE::ActorValue::kNone) { actor_value = effect->data.primaryAV; }

		if (!a_check) { return actor_value; }

		if ((actor_value == RE::ActorValue::kHealth || actor_value == RE::ActorValue::kStamina ||
				actor_value == RE::ActorValue::kMagicka) &&
			effect->data.flags.none(RE::EffectSetting::EffectSettingData::Flag::kRecover))
		{
			return actor_value;
		}

		return RE::ActorValue::kNone;
	}

	rust::Box<SpellData> fillOutSpellData(bool twoHanded, int32_t skill_level, const RE::EffectSetting* effect)
	{
		auto isHostile = effect->IsHostile();
		auto archetype = effect->data.archetype;
		auto resist    = effect->data.resistVariable;
		auto school    = effect->GetMagickSkill();

		rust::Box<SpellData> data = fill_out_spell_data(isHostile,
			static_cast<std::underlying_type_t<RE::ActorValue>>(resist),
			twoHanded,
			static_cast<std::underlying_type_t<RE::ActorValue>>(school),
			skill_level,
			static_cast<std::underlying_type_t<RE::EffectSetting::Archetype>>(archetype));
		return data;
	}

	rust::Box<HudItem> nonInventoryHudItem(RE::TESForm* form)
	{
		if (!form) { return empty_huditem(); }

		auto loggerName         = game::displayName(form);
		auto chonker            = helpers::chars_to_vec(loggerName);
		std::string form_string = helpers::makeFormSpecString(form);
		bool two_handed         = requiresTwoHands(form);

		if (form->Is(RE::FormType::Shout))
		{
			rlog::debug("making HudItem for shout: '{}'"sv, loggerName);
			auto* shout = form->As<RE::TESShout>();

			if (!shout) return simple_from_formdata(ItemCategory::Shout, std::move(chonker), form_string);
			auto* spell = shout->variations[RE::TESShout::VariationIDs::kOne].spell;  // always the first to ID
			if (!spell) return simple_from_formdata(ItemCategory::Shout, std::move(chonker), form_string);

			spell->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;
			categorize_shout(*keywords, std::move(chonker), form_string);
		}

		if (form->Is(RE::FormType::Spell))
		{
			auto* spell           = form->As<RE::SpellItem>();
			const auto spell_type = spell->GetSpellType();

			if (spell_type == RE::MagicSystem::SpellType::kLesserPower ||
				spell_type == RE::MagicSystem::SpellType::kPower)
			{
				rlog::debug("making HudItem for power: '{}'"sv, loggerName);
				const auto* costliest = spell->GetCostliestEffectItem();
				if (costliest)
				{
					const auto* effect = costliest->baseEffect;
					if (effect)
					{
						effect->ForEachKeyword(KeywordAccumulator::collect);
						auto& keywords          = KeywordAccumulator::mKeywords;
						rust::Box<HudItem> item = hud_item_from_keywords(
							ItemCategory::Power, *keywords, std::move(chonker), form_string, 1, false);
						return item;
					}
				}
			}

			// Regular spells.
			rlog::debug("making HudItem for spell: '{}'"sv, loggerName);
			const auto* costliest = spell->GetCostliestEffectItem();
			if (costliest)
			{
				const auto* effect = costliest->baseEffect;
				if (effect)
				{
					effect->ForEachKeyword(KeywordAccumulator::collect);
					auto& keywords          = KeywordAccumulator::mKeywords;
					auto skill_level        = effect->GetMinimumSkillLevel();
					auto data               = fillOutSpellData(two_handed, skill_level, effect);
					rust::Box<HudItem> item = magic_from_spelldata(
						ItemCategory::Spell, std::move(data), *keywords, std::move(chonker), form_string, 1);
					return item;
				}
			}
		}

		const auto formtype    = form->GetFormType();
		const auto formtypestr = RE::FormTypeToString(formtype);
		rlog::debug("hudItemFromForm() fell all the way through; type={}; name='{}'; formspec='{}';",
			formtypestr,
			loggerName,
			form_string);
		return empty_huditem();
	}

	rust::Box<HudItem> hudItemFromForm(RE::TESForm* form)
	{
		if (!form) { return empty_huditem(); }

		KeywordAccumulator::clear();
		if (!form->IsInventoryObject()) { return nonInventoryHudItem(form); }

		auto* thePlayer                    = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* boundObject    = nullptr;
		game::EquippableItemData* itemData = nullptr;
		game::boundObjectForForm(form, boundObject, itemData);

		if (!itemData || !boundObject)
		{
			rlog::warn("Inventory object not found in inventory. {}", rlog::formatAsHex(form->GetFormID()));
			return empty_huditem();
		}

		auto loggerName      = game::displayName(form);
		auto chonker         = helpers::chars_to_vec(loggerName);
		std::string formSpec = helpers::makeFormSpecString(boundObject);
		bool twoHanded       = requiresTwoHands(form);

		if (form->Is(RE::FormType::Ammo))
		{
			rlog::debug("making HudItem for ammo: '{}'"sv, loggerName);
			const auto* ammo = form->As<RE::TESAmmo>()->AsKeywordForm();
			ammo->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;

			rust::Box<HudItem> item = hud_item_from_keywords(
				ItemCategory::Ammo, *keywords, std::move(chonker), formSpec, itemData->count, false);
			return item;
		}

		if (form->IsWeapon())
		{
			const auto* weapon = form->As<RE::TESObjectWEAP>();
			if (weapon)
			{
				rlog::debug("making HudItem for weapon: '{}'"sv, loggerName);
				weapon->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords = KeywordAccumulator::mKeywords;
				if (weapon->IsBound()) { keywords->push_back(std::string("OCF_InvColorBound")); }
				rust::Box<HudItem> item = hud_item_from_keywords(
					ItemCategory::Weapon, *keywords, std::move(chonker), formSpec, itemData->count, twoHanded);

				return item;
			}
		}

		if (form->IsArmor())
		{
			rlog::debug("making HudItem for armor: '{}'"sv, loggerName);
			const auto* armor = form->As<RE::TESObjectARMO>();
			armor->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords          = KeywordAccumulator::mKeywords;
			rust::Box<HudItem> item = hud_item_from_keywords(
				ItemCategory::Armor, *keywords, std::move(chonker), formSpec, itemData->count, false);

			return item;
		}

		// There are two kinds of lights: lights held in the hand like torches,
		// and wearable lights (usually lanterns). The wearable ones are armor, and
		// have just been taken care of in the previous block. This block handles
		// the other types. These go into the left hand!
		if (form->Is(RE::FormType::Light))
		{
			// This form type does not have keywords. This presents a problem. Cough.
			rlog::debug("making HudItem for light: '{}';"sv, loggerName);
			const auto name = std::string(form->GetName());
			if (name.find("Lantern") != std::string::npos)  // yes, very limited in effectiveness; TODO
			{
				rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Lantern, std::move(chonker), formSpec);
				return item;
			}
			rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Torch, std::move(chonker), formSpec);
			return item;
		}


		if (form->Is(RE::FormType::Scroll))
		{
			rlog::debug("making HudItem for scroll: '{}'"sv, loggerName);
			auto* scroll = form->As<RE::ScrollItem>();
			if (scroll->GetCostliestEffectItem() && scroll->GetCostliestEffectItem()->baseEffect)
			{
				const auto effect = scroll->GetCostliestEffectItem()->baseEffect;
				effect->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords  = KeywordAccumulator::mKeywords;
				auto skillLevel = effect->GetMinimumSkillLevel();

				auto data               = fillOutSpellData(twoHanded, skillLevel, effect);
				rust::Box<HudItem> item = magic_from_spelldata(
					ItemCategory::Scroll, std::move(data), *keywords, std::move(chonker), formSpec, itemData->count);
				return item;
			}
		}

		if (form->Is(RE::FormType::AlchemyItem))
		{
			auto* alchemy_potion = form->As<RE::AlchemyItem>();

			if (alchemy_potion->IsFood())
			{
				rlog::debug("making HudItem for food: '{}'"sv, loggerName);
				alchemy_potion->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords          = KeywordAccumulator::mKeywords;
				rust::Box<HudItem> item = hud_item_from_keywords(
					ItemCategory::Food, *keywords, std::move(chonker), formSpec, itemData->count, false);
				return item;
			}
			else
			{
				rlog::debug("making HudItem for potion: '{}'"sv, loggerName);
				const auto* effect      = alchemy_potion->GetCostliestEffectItem()->baseEffect;
				auto actor_value        = effect->data.primaryAV;
				rust::Box<HudItem> item = potion_from_formdata(alchemy_potion->IsPoison(),
					static_cast<int32_t>(actor_value),
					itemData->count,
					std::move(chonker),
					formSpec);
				return item;
			}
		}

		const auto formtype    = form->GetFormType();
		const auto formtypestr = RE::FormTypeToString(formtype);
		rlog::debug("hudItemFromForm() fell all the way through; type={}; name='{}'; formspec='{}';",
			formtypestr,
			loggerName,
			formSpec);
		return empty_huditem();
	}

	RE::BSContainer::ForEachResult KeywordAccumulator::collect(RE::BGSKeyword& kwd)
	{
		if (!mKeywords) { mKeywords = new std::vector<std::string>(); }

		auto id  = kwd.GetFormEditorID();
		auto str = std::string(id);
		mKeywords->push_back(str);
		return RE::BSContainer::ForEachResult::kContinue;
	}

	void KeywordAccumulator::printKeywords()
	{
		if (!mKeywords) { rlog::debug("no keywords to print"); }
		for (std::string kwd : *mKeywords) { rlog::info("{}"sv, kwd); }
	}
}
