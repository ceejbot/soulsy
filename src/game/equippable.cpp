#include "equippable.h"

#include "helpers.h"
#include "player.h"

#include "lib.rs.h"

namespace equippable
{
	bool requiresTwoHands(RE::TESForm*& item_form)
	{
		if (!item_form) { return false; }

		auto two_handed = false;
		if (item_form->Is(RE::FormType::Spell))
		{
			if (const auto* spell = item_form->As<RE::SpellItem>(); spell->IsTwoHanded()) { return true; }
		}

		if (item_form->IsWeapon())
		{
			if (const auto* weapon = item_form->As<RE::TESObjectWEAP>();
				weapon->IsTwoHandedAxe() || weapon->IsTwoHandedSword() || weapon->IsBow() || weapon->IsCrossbow())
			{
				return true;
			}
		}
		else if (item_form->Is(RE::FormType::Scroll))
		{
			auto* scroll = item_form->As<RE::ScrollItem>();
			return scroll->IsTwoHanded();
		}


		if (item_form->Is(RE::FormType::Scroll))
		{
			auto* scroll = item_form->As<RE::ScrollItem>();
			return scroll->IsTwoHanded();
		}

		//logger::trace("form {}, two handed {}"sv, item_form->GetName(), two_handed);
		return two_handed;
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
		auto archetype        = effect->data.archetype;
		auto primary_effect   = effect->data.primaryAV;
		auto secondary_effect = effect->data.secondaryAV;
		auto resist           = effect->data.resistVariable;
		auto school           = effect->GetMagickSkill();
		auto assoc            = effect->data.associatedForm;

		std::string assoc_formspec;
		if (assoc) { assoc_formspec = helpers::makeFormSpecString(assoc); }
		else { assoc_formspec = std::string(""); }

		rust::Box<SpellData> data =
			fill_out_spell_data(static_cast<std::underlying_type_t<RE::ActorValue>>(primary_effect),
				static_cast<std::underlying_type_t<RE::ActorValue>>(secondary_effect),
				static_cast<std::underlying_type_t<RE::ActorValue>>(resist),
				two_handed,
				static_cast<std::underlying_type_t<RE::ActorValue>>(school),
				skill_level,
				static_cast<std::underlying_type_t<RE::EffectSetting::Archetype>>(archetype),
				assoc_formspec);
		return data;
	}

	rust::Box<HudItem> hudItemFromForm(RE::TESForm* item_form)
	{
		if (!item_form) { return empty_huditem(); }

		KeywordAccumulator::keywords = new std::vector<std::string>();
		auto chonker                 = helpers::chars_to_vec(item_form->GetName());
		std::string form_string      = helpers::makeFormSpecString(item_form);
		bool two_handed              = requiresTwoHands(item_form);

		if (item_form->Is(RE::FormType::Light))
		{
			logger::info("making HudItem for torch: '{}'"sv, item_form->GetName());
			rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Light, std::move(chonker), form_string);
			return item;
		}

		if (item_form->Is(RE::FormType::Ammo))
		{
			logger::info("making HudItem for ammo: '{}'"sv, item_form->GetName());
			const auto* ammo = item_form->As<RE::TESAmmo>()->AsKeywordForm();
			ammo->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::keywords;
			auto count     = player::getInventoryCountByForm(item_form);

			rust::Box<HudItem> item =
				hud_item_from_keywords(ItemCategory::Ammo, *keywords, std::move(chonker), form_string, count, false);
			return item;
		}

		if (item_form->IsWeapon())
		{
			if (const auto* weapon = item_form->As<RE::TESObjectWEAP>(); !weapon->IsBound())
			{
				logger::info("making HudItem for weapon: '{}'"sv, item_form->GetName());
				weapon->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords          = KeywordAccumulator::keywords;
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
			auto& keywords = KeywordAccumulator::keywords;
			auto count     = player::getInventoryCountByForm(item_form);
			rust::Box<HudItem> item =
				hud_item_from_keywords(ItemCategory::Armor, *keywords, std::move(chonker), form_string, count, false);

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

			auto data = fillOutSpellData(false, 1, effect);
			rust::Box<HudItem> item =
				magic_from_spelldata(ItemCategory::Shout, std::move(data), std::move(chonker), form_string, 1);
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
				rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Power, std::move(chonker), form_string);
				return item;
			}

			// Regular spells.
			logger::info("making HudItem for spell: '{}'"sv, item_form->GetName());
			const auto* effect = spell->GetCostliestEffectItem()->baseEffect;
			if (effect)
			{
				auto skill_level = effect->GetMinimumSkillLevel();
				auto data        = fillOutSpellData(two_handed, skill_level, effect);
				rust::Box<HudItem> item =
					magic_from_spelldata(ItemCategory::Spell, std::move(data), std::move(chonker), form_string, 1);
				return item;
			}
		}

		if (item_form->Is(RE::FormType::Scroll))
		{
			logger::info("making HudItem for scroll: '{}'"sv, item_form->GetName());

			auto* scroll      = item_form->As<RE::ScrollItem>();
			const auto effect = scroll->GetCostliestEffectItem()->baseEffect;
			auto skill_level  = effect->GetMinimumSkillLevel();

			auto data               = fillOutSpellData(two_handed, skill_level, effect);
			rust::Box<HudItem> item = magic_from_spelldata(
				ItemCategory::Scroll, std::move(data), std::move(chonker), form_string, 1);
			return item;
		}

		if (item_form->Is(RE::FormType::AlchemyItem))
		{
			logger::info("making HudItem for alchemy item: '{}'"sv, item_form->GetName());

			auto count           = player::getInventoryCountByForm(item_form);
			auto* alchemy_potion = item_form->As<RE::AlchemyItem>();
			const auto* effect   = alchemy_potion->GetCostliestEffectItem()->baseEffect;
			auto actor_value     = effect->data.primaryAV;

			if (alchemy_potion->IsFood())
			{
				// TODO soup, water, meat, veggies
				// categorize drinks vs food
				logger::info("making HudItem for food: '{}'"sv, item_form->GetName());
				rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Food, std::move(chonker), form_string);
				return item;
			}
			else
			{
				// TODO xfer more data
				rust::Box<HudItem> item = potion_from_formdata(alchemy_potion->IsPoison(),
					static_cast<int32_t>(actor_value),
					count,
					std::move(chonker),
					form_string);
				return item;
			}
		}

		return empty_huditem();
	}

	RE::BSContainer::ForEachResult KeywordAccumulator::collect(RE::BGSKeyword& kwd)
	{
		if (!keywords) { keywords = new std::vector<std::string>(); }

		auto id  = kwd.GetFormEditorID();
		auto str = std::string(id);
		keywords->push_back(str);
		return RE::BSContainer::ForEachResult::kContinue;
	}

	void KeywordAccumulator::printKeywords()
	{
		if (!keywords) { logger::info("no keywords"); }
		for (std::string kwd : *keywords) { logger::info("{}"sv, kwd); }
	}
}
