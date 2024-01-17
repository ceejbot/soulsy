#include "equippable.h"

#include "gear.h"
#include "helpers.h"

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

	rust::Box<HudItem> hudItemFromForm(RE::TESForm* form)
	{
		if (!form) { return empty_huditem(); }

		RE::TESBoundObject* boundObject = nullptr;
		RE::ExtraDataList* extraData    = nullptr;
		const auto count                = gear::boundObjectForForm(form, boundObject, extraData);

		auto safename = boundObject ? helpers::displayNameAsUtf8(boundObject) : helpers::displayNameAsUtf8(form);
		std::string formSpec =
			boundObject ? helpers::makeFormSpecString(boundObject) : helpers::makeFormSpecString(form);
		bool twoHanded = requiresTwoHands(form);

		KeywordAccumulator::clear();

		if (form->Is(RE::FormType::Shout))
		{
			rlog::trace("making HudItem for shout: '{}'"sv, safename);
			auto* shout = form->As<RE::TESShout>();

			// Fall back to something if we can't find it.
			if (!shout) return simple_from_formdata(ItemCategory::Shout, std::move(safename), formSpec);

			auto* spell = shout->variations[RE::TESShout::VariationIDs::kOne].spell;  // always the first to ID
			if (!spell) return simple_from_formdata(ItemCategory::Shout, std::move(safename), formSpec);

			spell->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;
			return categorize_shout(*keywords, std::move(safename), formSpec);
		}

		if (form->Is(RE::FormType::Spell))
		{
			auto* spell           = form->As<RE::SpellItem>();
			const auto spell_type = spell->GetSpellType();

			if (spell_type == RE::MagicSystem::SpellType::kLesserPower ||
				spell_type == RE::MagicSystem::SpellType::kPower)
			{
				rlog::trace("making HudItem for power: '{}'"sv, safename);
				const auto* costliest = spell->GetCostliestEffectItem();
				if (costliest)
				{
					const auto* effect = costliest->baseEffect;
					if (effect)
					{
						effect->ForEachKeyword(KeywordAccumulator::collect);
						auto& keywords          = KeywordAccumulator::mKeywords;
						rust::Box<HudItem> item = hud_item_from_keywords(
							ItemCategory::Power, *keywords, std::move(safename), formSpec, 1, false);
						return item;
					}
				}
			}

			// Regular spells.
			rlog::trace("making HudItem for spell: '{}'"sv, safename);
			const auto* costliest = spell->GetCostliestEffectItem();
			if (costliest)
			{
				const auto* effect = costliest->baseEffect;
				if (effect)
				{
					effect->ForEachKeyword(KeywordAccumulator::collect);
					auto& keywords          = KeywordAccumulator::mKeywords;
					auto skill_level        = effect->GetMinimumSkillLevel();
					auto data               = fillOutSpellData(twoHanded, skill_level, effect);
					rust::Box<HudItem> item = magic_from_spelldata(
						ItemCategory::Spell, std::move(data), *keywords, std::move(safename), formSpec, 1);
					return item;
				}
			}
		}

		if (form->Is(RE::FormType::Ammo))
		{
			rlog::trace("making HudItem for ammo: '{}'"sv, safename);
			const auto* ammo = form->As<RE::TESAmmo>()->AsKeywordForm();
			ammo->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;

			rust::Box<HudItem> item =
				hud_item_from_keywords(ItemCategory::Ammo, *keywords, std::move(safename), formSpec, count, false);
			return item;
		}

		if (form->IsWeapon())
		{
			const auto* weapon = form->As<RE::TESObjectWEAP>();
			if (weapon)
			{
				rlog::trace("making HudItem for weapon: '{}'"sv, safename);
				weapon->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords = KeywordAccumulator::mKeywords;
				if (weapon->IsBound()) { keywords->push_back(std::string("OCF_InvColorBound")); }
				rust::Box<HudItem> item = hud_item_from_keywords(
					ItemCategory::Weapon, *keywords, std::move(safename), formSpec, count, twoHanded);

				return item;
			}
		}

		if (form->IsArmor())
		{
			rlog::trace("making HudItem for armor: '{}'"sv, safename);
			const auto* armor = form->As<RE::TESObjectARMO>();
			armor->ForEachKeyword(KeywordAccumulator::collect);
			auto& keywords = KeywordAccumulator::mKeywords;
			rust::Box<HudItem> item =
				hud_item_from_keywords(ItemCategory::Armor, *keywords, std::move(safename), formSpec, count, false);

			return item;
		}

		// There are two kinds of lights: lights held in the hand like torches,
		// and wearable lights (usually lanterns). The wearable ones are armor, and
		// have just been taken care of in the previous block. This block handles
		// the other types. These go into the left hand!
		if (form->Is(RE::FormType::Light))
		{
			// This form type does not have keywords. This presents a problem. Cough.
			rlog::trace("making HudItem for light: '{}';"sv, safename);
			const auto name = std::string(form->GetName());  // this use of GetName() is okay
			if (name.find("Lantern") != std::string::npos)   // yes, very limited in effectiveness; TODO
			{
				rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Lantern, std::move(safename), formSpec);
				return item;
			}
			rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Torch, std::move(safename), formSpec);
			return item;
		}


		if (form->Is(RE::FormType::Scroll))
		{
			rlog::trace("making HudItem for scroll: '{}'"sv, safename);
			auto* scroll = form->As<RE::ScrollItem>();
			if (scroll->GetCostliestEffectItem() && scroll->GetCostliestEffectItem()->baseEffect)
			{
				const auto effect = scroll->GetCostliestEffectItem()->baseEffect;
				effect->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords  = KeywordAccumulator::mKeywords;
				auto skillLevel = effect->GetMinimumSkillLevel();

				auto data               = fillOutSpellData(twoHanded, skillLevel, effect);
				rust::Box<HudItem> item = magic_from_spelldata(
					ItemCategory::Scroll, std::move(data), *keywords, std::move(safename), formSpec, count);
				return item;
			}
		}

		if (form->Is(RE::FormType::AlchemyItem))
		{
			auto* alchemy_potion = form->As<RE::AlchemyItem>();

			if (alchemy_potion->IsFood())
			{
				rlog::trace("making HudItem for food: '{}'"sv, safename);
				alchemy_potion->ForEachKeyword(KeywordAccumulator::collect);
				auto& keywords = KeywordAccumulator::mKeywords;
				rust::Box<HudItem> item =
					hud_item_from_keywords(ItemCategory::Food, *keywords, std::move(safename), formSpec, count, false);
				return item;
			}
			else
			{
				rlog::trace("making HudItem for potion: '{}'"sv, safename);
				const auto* effect      = alchemy_potion->GetCostliestEffectItem()->baseEffect;
				auto actor_value        = effect->data.primaryAV;
				rust::Box<HudItem> item = potion_from_formdata(alchemy_potion->IsPoison(),
					static_cast<int32_t>(actor_value),
					count,
					std::move(safename),
					formSpec);
				return item;
			}
		}

		if (form->Is(RE::FormType::Book))
		{
			rlog::trace("making HudItem for boook: '{}';"sv, safename);
			rust::Box<HudItem> item = simple_from_formdata(ItemCategory::Book, std::move(safename), formSpec);
			return item;
		}

		const auto formtype    = form->GetFormType();
		const auto formtypestr = RE::FormTypeToString(formtype);
		rlog::debug("hudItemFromForm() fell all the way through; type={}; name='{}'; formspec='{}';",
			formtypestr,
			safename,
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
