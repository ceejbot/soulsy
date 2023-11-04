#include "equippable.h"

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

	rust::Box<SpellData> fillOutSpellData(bool two_handed, int32_t skill_level, const RE::EffectSetting* effect)
	{
		auto isHostile = effect->IsHostile();
		auto archetype = effect->data.archetype;
		auto resist    = effect->data.resistVariable;
		auto school    = effect->GetMagickSkill();

		rust::Box<SpellData> data = fill_out_spell_data(isHostile,
			static_cast<std::underlying_type_t<RE::ActorValue>>(resist),
			two_handed,
			static_cast<std::underlying_type_t<RE::ActorValue>>(school),
			skill_level,
			static_cast<std::underlying_type_t<RE::EffectSetting::Archetype>>(archetype));
		return data;
	}

	rust::Box<HudItem> hudItemFromForm(RE::TESForm* item_form)
	{
		if (!item_form) { return empty_huditem(); }

		KeywordAccumulator::clear();
		auto chonker            = helpers::chars_to_vec(item_form->GetName());
		std::string form_string = helpers::makeFormSpecString(item_form);
		bool two_handed         = requiresTwoHands(item_form);

		if (item_form->Is(RE::FormType::Ammo))
		{
			logger::info("making HudItem for ammo: '{}'"sv, item_form->GetName());
			const auto* ammo = item_form->As<RE::TESAmmo>()->AsKeywordForm();
			ammo->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;
			auto count     = player::getInventoryCountByForm(item_form);

			rust::Box<HudItem> item =
				hud_item_from_keywords(ItemCategory::Ammo, *keywords, std::move(chonker), form_string, count, false);
			return item;
		}

		if (item_form->IsWeapon())
		{
			const auto* weapon = item_form->As<RE::TESObjectWEAP>();
			if (weapon)
			{
				logger::info("making HudItem for weapon: '{}'"sv, item_form->GetName());
				weapon->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords = KeywordAccumulator::mKeywords;
				if (weapon->IsBound()) { keywords->push_back(std::string("OCF_InvColorBound")); }
				auto count              = player::getInventoryCountByForm(item_form);
				rust::Box<HudItem> item = hud_item_from_keywords(
					ItemCategory::Weapon, *keywords, std::move(chonker), form_string, count, two_handed);

				return item;
			}
		}

		if (item_form->IsArmor())
		{
			logger::info("making HudItem for armor: '{}'"sv, item_form->GetName());
			const auto* armor = item_form->As<RE::TESObjectARMO>();
			armor->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;
			auto count     = player::getInventoryCountByForm(item_form);
			rust::Box<HudItem> item =
				hud_item_from_keywords(ItemCategory::Armor, *keywords, std::move(chonker), form_string, count, false);

			return item;
		}

		// There are two kinds of lights: lights held in the hand like torches,
		// and wearable lights (usually lanterns). The wearable ones are armor, and
		// have just been taken care of in the previous block. This block handles
		// the other types. These go into the left hand!
		if (item_form->Is(RE::FormType::Light))
		{
			// This form type does not have keywords. This presents a problem. Cough.
			logger::info("making HudItem for light: '{}';"sv, item_form->GetName());
			const auto name = std::string(item_form->GetName());
			if (name.find("Lantern") != std::string::npos)  // yes, very limited in effectiveness
			{
				rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Lantern, std::move(chonker), form_string);
				return item;
			}
			rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Torch, std::move(chonker), form_string);
			return item;
		}

		if (item_form->Is(RE::FormType::Shout))
		{
			logger::info("making HudItem for shout: '{}'"sv, item_form->GetName());
			auto* shout = item_form->As<RE::TESShout>();

			if (!shout) return simple_from_formdata(ItemCategory::Shout, std::move(chonker), form_string);
			auto* spell = shout->variations[RE::TESShout::VariationIDs::kOne].spell;  // always the first to ID
			if (!spell) return simple_from_formdata(ItemCategory::Shout, std::move(chonker), form_string);

			const auto* effect = spell->GetCostliestEffectItem()->baseEffect;
			if (!effect) return simple_from_formdata(ItemCategory::Shout, std::move(chonker), form_string);
			effect->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;

			auto data               = fillOutSpellData(false, 1, effect);
			rust::Box<HudItem> item = magic_from_spelldata(
				ItemCategory::Shout, std::move(data), *keywords, std::move(chonker), form_string, 1);
			return item;
		}

		if (item_form->Is(RE::FormType::Spell))
		{
			auto* spell           = item_form->As<RE::SpellItem>();
			const auto spell_type = spell->GetSpellType();

			if (spell_type == RE::MagicSystem::SpellType::kLesserPower ||
				spell_type == RE::MagicSystem::SpellType::kPower)
			{
				logger::info("making HudItem for power: '{}'"sv, item_form->GetName());
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
			logger::info("making HudItem for spell: '{}'"sv, item_form->GetName());
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

		if (item_form->Is(RE::FormType::Scroll))
		{
			logger::info("making HudItem for scroll: '{}'"sv, item_form->GetName());
			auto* scroll = item_form->As<RE::ScrollItem>();
			if (scroll->GetCostliestEffectItem() && scroll->GetCostliestEffectItem()->baseEffect)
			{
				const auto effect = scroll->GetCostliestEffectItem()->baseEffect;
				effect->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords   = KeywordAccumulator::mKeywords;
				auto skill_level = effect->GetMinimumSkillLevel();

				auto data               = fillOutSpellData(two_handed, skill_level, effect);
				rust::Box<HudItem> item = magic_from_spelldata(
					ItemCategory::Scroll, std::move(data), *keywords, std::move(chonker), form_string, 1);
				return item;
			}
		}

		if (item_form->Is(RE::FormType::AlchemyItem))
		{
			auto count           = player::getInventoryCountByForm(item_form);
			auto* alchemy_potion = item_form->As<RE::AlchemyItem>();
			const auto* effect   = alchemy_potion->GetCostliestEffectItem()->baseEffect;
			auto actor_value     = effect->data.primaryAV;

			if (alchemy_potion->IsFood())
			{
				logger::info("making HudItem for food: '{}'"sv, item_form->GetName());
				alchemy_potion->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords          = KeywordAccumulator::mKeywords;
				rust::Box<HudItem> item = hud_item_from_keywords(
					ItemCategory::Food, *keywords, std::move(chonker), form_string, count, false);
				return item;
			}
			else
			{
				logger::info("making HudItem for potion: '{}'"sv, item_form->GetName());
				rust::Box<HudItem> item = potion_from_formdata(alchemy_potion->IsPoison(),
					static_cast<int32_t>(actor_value),
					count,
					std::move(chonker),
					form_string);
				return item;
			}
		}

		const auto formtype    = item_form->GetFormType();
		const auto formtypestr = RE::FormTypeToString(formtype);
		logger::debug("hudItemFromForm() fell all the way through; type={}; name='{}'; formspec='{}';",
			formtypestr,
			item_form->GetName(),
			form_string);
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
		if (!mKeywords) { logger::debug("no keywords to print"); }
		for (std::string kwd : *mKeywords) { logger::info("{}"sv, kwd); }
	}
}
