#include "equippable.h"

#include "helpers.h"
#include "lib.rs.h"
#include "player.h"

namespace equippable
{
	rust::Box<ItemData> makeItemDataFromForm(RE::TESForm* item_form)
	{
		if (!item_form) {
			logger::warn("Called makeItemDataFromForm() with null pointer.");
			return empty_itemdata();
		}
		logger::info("making itemdata for '{}'"sv, item_form->GetName());
		bool two_handed         = equippable::requiresTwoHands(item_form);
		std::string form_string = helpers::makeFormSpecString(item_form);
		auto kind               = equippable::itemKindFromForm(item_form);
		auto count              = player::getInventoryCountByForm(item_form);
		bool show_count         = kind_has_count(kind);
		auto chonker            = helpers::chars_to_vec(item_form->GetName());

		return itemdata_from_formdata(kind, two_handed, show_count, count, std::move(chonker), form_string);
	}

	bool canInstantCast(RE::TESForm* item_form, const ItemKind kind)
	{
		if (kind_is_magic(kind))
		{
			const auto* spell = item_form->As<RE::SpellItem>();
			if (spell->GetSpellType() == RE::MagicSystem::SpellType::kSpell ||
				spell->GetSpellType() == RE::MagicSystem::SpellType::kLeveledSpell)
			{
				if (spell->GetCastingType() != RE::MagicSystem::CastingType::kConcentration) { return true; }
			}
			return false;
		}

		return (kind == ItemKind::Scroll);
	}

	bool requiresTwoHands(RE::TESForm*& item_form)
	{
		if (!item_form) { return false; }

		auto two_handed = false;
		if (item_form->Is(RE::FormType::Spell))
		{
			if (const auto* spell = item_form->As<RE::SpellItem>(); spell->IsTwoHanded()) { two_handed = true; }
		}
		else if (item_form->IsWeapon())
		{
			if (const auto* weapon = item_form->As<RE::TESObjectWEAP>();
				weapon->IsTwoHandedAxe() || weapon->IsTwoHandedSword() || weapon->IsBow() || weapon->IsCrossbow())
			{
				two_handed = true;
			}
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

	ItemKind itemKindFromForm(RE::TESForm*& item_form)
	{
		if (!item_form) { return ItemKind::NotFound; }

		if (item_form->IsWeapon())
		{
			if (const auto* weapon = item_form->As<RE::TESObjectWEAP>(); !weapon->IsBound())
			{
				return subKindForWeapon(item_form);
			}
		}

		if (item_form->IsArmor())
		{
			const auto* armor = item_form->As<RE::TESObjectARMO>();
			//GetSlotMask 49
			if (armor->IsShield()) { return ItemKind::Shield; }
			else if (armor->IsClothing() &&
					 (armor->HasKeywordString("_WL_Lantern") &&
							 armor->HasPartOf(RE::BIPED_MODEL::BipedObjectSlot::kNone) &&
							 !armor->HasPartOf(RE::BIPED_MODEL::BipedObjectSlot::kModFaceJewelry) ||
						 armor->HasPartOf(RE::BIPED_MODEL::BipedObjectSlot::kModPelvisPrimary)))
			{
				//Wearable Lanterns got keyword _WL_Lantern
				//Simple Wearable Lanterns do not have a keyword, but will be equipped on 49 (30+19)
				return ItemKind::Lantern;
			}
			else if (armor->IsClothing() && armor->HasKeywordString("BOS_DisplayMaskKeyword"))
			{
				return ItemKind::Mask;
			}

			return subKindForArmor(item_form);
		}

		if (item_form->Is(RE::FormType::Spell))
		{
			const auto spell_type = item_form->As<RE::SpellItem>()->GetSpellType();
			if (spell_type == RE::MagicSystem::SpellType::kSpell ||
				spell_type == RE::MagicSystem::SpellType::kLeveledSpell)
			{
				return subKindForMagic(item_form);
				;
			}
			if (spell_type == RE::MagicSystem::SpellType::kLesserPower ||
				spell_type == RE::MagicSystem::SpellType::kPower)
			{
				return ItemKind::Power;
			}
		}

		if (item_form->Is(RE::FormType::Shout)) { return ItemKind::Shout; }

		if (item_form->Is(RE::FormType::AlchemyItem)) { return subKindForConsumable(item_form); }

		if (item_form->Is(RE::FormType::Scroll)) { return ItemKind::Scroll; }

		if (item_form->Is(RE::FormType::Ammo)) { return ItemKind::Arrow; }

		if (item_form->Is(RE::FormType::Light)) { return ItemKind::Torch; }

		return ItemKind::IconDefault;
	}

	ItemKind subKindForWeapon(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->IsWeapon()) { return ItemKind::IconDefault; }
		switch (const auto* weapon = item_form->As<RE::TESObjectWEAP>(); weapon->GetWeaponType())
		{
			case RE::WEAPON_TYPE::kHandToHandMelee: return ItemKind::HandToHand;

			case RE::WEAPON_TYPE::kOneHandSword:
				if (weapon->HasKeywordString("WeapTypeRapier")) { return ItemKind::Rapier; }
				else if (weapon->HasKeywordString("WeapTypeKatana")) { return ItemKind::Katana; }
				return ItemKind::SwordOneHanded;

			case RE::WEAPON_TYPE::kOneHandDagger:
				if (weapon->HasKeywordString("WeapTypeClaw")) { return ItemKind::Claw; }
				return ItemKind::Dagger;

			case RE::WEAPON_TYPE::kOneHandAxe: return ItemKind::AxeOneHanded;

			case RE::WEAPON_TYPE::kOneHandMace:
				if (weapon->HasKeywordString("WeapTypeWhip")) { return ItemKind::Whip; }
				return ItemKind::Mace;

			case RE::WEAPON_TYPE::kTwoHandSword:
				if (weapon->HasKeywordString("WeapTypePike")) { return ItemKind::Pike; }
				return ItemKind::SwordTwoHanded;

			case RE::WEAPON_TYPE::kTwoHandAxe:
				if (weapon->HasKeywordString("WeapTypeHalberd")) { return ItemKind::Halberd; }
				else if (weapon->HasKeywordString("WeapTypeQtrStaff")) { return ItemKind::QuarterStaff; }
				return ItemKind::AxeTwoHanded;

			case RE::WEAPON_TYPE::kBow: return ItemKind::Bow;

			case RE::WEAPON_TYPE::kStaff: return ItemKind::Staff;

			case RE::WEAPON_TYPE::kCrossbow: return ItemKind::Crossbow;
		}

		return ItemKind::IconDefault;
	}

	ItemKind subKindForMagic(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->Is(RE::FormType::Spell)) { return ItemKind::IconDefault; }

		auto* spell        = item_form->As<RE::SpellItem>();
		const auto* effect = spell->GetCostliestEffectItem()->baseEffect;
		auto actor_value   = effect->GetMagickSkill();
		if (actor_value == RE::ActorValue::kNone) { actor_value = effect->data.primaryAV; }

		switch (actor_value)
		{
			case RE::ActorValue::kAlteration: return ItemKind::Alteration;

			case RE::ActorValue::kConjuration: return ItemKind::Conjuration;

			case RE::ActorValue::kDestruction:
				switch (effect->data.resistVariable)
				{
					case RE::ActorValue::kResistFire: return ItemKind::DestructionFire;
					case RE::ActorValue::kResistFrost: return ItemKind::DestructionFrost;
					case RE::ActorValue::kResistShock: return ItemKind::DestructionShock;
					default: return ItemKind::Destruction;
				}
			case RE::ActorValue::kIllusion: return ItemKind::Illusion;

			case RE::ActorValue::kRestoration:
				//might not fit all spells
				return ItemKind::Restoration;

			default: return ItemKind::SpellDefault;
		}
	}

	ItemKind subKindForConsumable(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->Is(RE::FormType::AlchemyItem)) { return ItemKind::IconDefault; }

		auto* alchemy_potion = item_form->As<RE::AlchemyItem>();
		if (alchemy_potion->IsFood()) { return ItemKind::Food; }
		if (alchemy_potion->IsPoison()) { return ItemKind::PoisonDefault; }

		auto actor_value = getPotionEffect(alchemy_potion, false);
		return subKindForConsumableByEffect(actor_value);
	}

	ItemKind subKindForArmor(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->IsArmor()) { return ItemKind::IconDefault; }
		switch (const auto* armor = item_form->As<RE::TESObjectARMO>(); armor->GetArmorType())
		{
			case RE::BIPED_MODEL::ArmorType::kLightArmor: return ItemKind::ArmorLight;
			case RE::BIPED_MODEL::ArmorType::kHeavyArmor: return ItemKind::ArmorHeavy;
			case RE::BIPED_MODEL::ArmorType::kClothing: return ItemKind::ArmorClothing;
		}

		return ItemKind::IconDefault;
	}

	ItemKind subKindForConsumableByEffect(RE::ActorValue& actor_value)
	{
		switch (actor_value)
		{
			case RE::ActorValue::kHealth:
			case RE::ActorValue::kHealRateMult:
			case RE::ActorValue::kHealRate: return ItemKind::PotionHealth;

			case RE::ActorValue::kStamina:
			case RE::ActorValue::kStaminaRateMult:
			case RE::ActorValue::kStaminaRate: return ItemKind::PotionStamina;

			case RE::ActorValue::kMagicka:
			case RE::ActorValue::kMagickaRateMult:
			case RE::ActorValue::kMagickaRate: return ItemKind::PotionMagicka;

			case RE::ActorValue::kResistFire: return ItemKind::PotionFireResist;

			case RE::ActorValue::kResistShock: return ItemKind::PotionShockResist;

			case RE::ActorValue::kResistFrost: return ItemKind::PotionFrostResist;

			case RE::ActorValue::kResistMagic: return ItemKind::PotionMagicResist;

			default: return ItemKind::PotionDefault;
		}
	}
}
