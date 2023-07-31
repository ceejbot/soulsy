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
			if (const auto* spell = item_form->As<RE::SpellItem>(); spell->IsTwoHanded()) { return true; }
		}
		else if (item_form->IsWeapon())
		{
			if (const auto* weapon = item_form->As<RE::TESObjectWEAP>();
				weapon->IsTwoHandedAxe() || weapon->IsTwoHandedSword() || weapon->IsBow() || weapon->IsCrossbow())
			{
				return true;
			}
		}
		else if (item_form->Is(RE::FormType::Scroll)) { 
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
			else if (armor->HasKeywordString("BOS_DisplayMaskKeyword")) { return ItemKind::Mask; }

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
		// ItemKind::Hammer, ItemKind::Lance, Scythe, Scimitar ?
		if (!item_form || !item_form->IsWeapon()) { return ItemKind::IconDefault; }

		const auto* weapon = item_form->As<RE::TESObjectWEAP>();

		// Bullet-proofing ourselves against mods doing different base weapon types.
		if (weapon->HasKeywordString("WeapTypeClaw")) { return ItemKind::Claw; }
		if (weapon->HasKeywordString("WeapTypeGun")) { return ItemKind::Gun; }
		if (weapon->HasKeywordString("WeapTypeHalberd")) { return ItemKind::Halberd; }
		if (weapon->HasKeywordString("WeapTypeKatana")) { return ItemKind::Katana; }
		if (weapon->HasKeywordString("WeapTypePike")) { return ItemKind::Pike; }
		if (weapon->HasKeywordString("WeapTypeQtrStaff")) { return ItemKind::QuarterStaff; }
		if (weapon->HasKeywordString("WeapTypeRapier")) { return ItemKind::Rapier; }
		if (weapon->HasKeywordString("WeapTypeWhip")) { return ItemKind::Whip; }

		switch (weapon->GetWeaponType())
		{
			case RE::WEAPON_TYPE::kBow: return ItemKind::Bow;
			case RE::WEAPON_TYPE::kCrossbow: return ItemKind::Crossbow;
			case RE::WEAPON_TYPE::kHandToHandMelee: return ItemKind::HandToHand;
			case RE::WEAPON_TYPE::kOneHandAxe: return ItemKind::AxeOneHanded;
			case RE::WEAPON_TYPE::kOneHandDagger: return ItemKind::Dagger;
			case RE::WEAPON_TYPE::kOneHandMace: return ItemKind::Mace;
			case RE::WEAPON_TYPE::kOneHandSword: return ItemKind::SwordOneHanded;
			case RE::WEAPON_TYPE::kStaff: return ItemKind::Staff;
			case RE::WEAPON_TYPE::kTwoHandAxe: return ItemKind::AxeTwoHanded;
			case RE::WEAPON_TYPE::kTwoHandSword: return ItemKind::SwordTwoHanded;
		}

		return ItemKind::WeaponDefault;
	}

	using BipedObjectSlot = RE::BIPED_MODEL::BipedObjectSlot;

	bool isAmulet(const RE::TESObjectARMO* armor) { return armor->HasPartOf(BipedObjectSlot::kAmulet); }

	bool isBackpack(const RE::TESObjectARMO* armor)
	{
		// backpacks are 47
		return armor->HasPartOf(BipedObjectSlot::kModBack);
	}
	bool isCloak(const RE::TESObjectARMO* armor)
	{
		// cloaks are 46
		return armor->HasPartOf(BipedObjectSlot::kModBack);
	}

	bool isFeet(const RE::TESObjectARMO* armor)
	{
		return armor->HasPartOf(BipedObjectSlot::kFeet) || armor->HasPartOf(BipedObjectSlot::kCalves);
	}
	bool isHands(const RE::TESObjectARMO* armor)
	{
		return armor->HasPartOf(BipedObjectSlot::kHands) || armor->HasPartOf(BipedObjectSlot::kForearms);
	}
	bool isHead(const RE::TESObjectARMO* armor)
	{
		return armor->HasPartOf(BipedObjectSlot::kHead) || armor->HasPartOf(BipedObjectSlot::kHair) ||
		       armor->HasPartOf(BipedObjectSlot::kCirclet);
	}
	bool isRing(const RE::TESObjectARMO* armor) { return armor->HasPartOf(BipedObjectSlot::kRing); }

	ItemKind subKindForArmor(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->IsArmor()) { return ItemKind::IconDefault; }
		const auto* armor = item_form->As<RE::TESObjectARMO>();

		if (isRing(armor)) { return ItemKind::ArmorRing; }
		else if (isAmulet(armor)) { return ItemKind::ArmorAmulet; }
		else if (isCloak(armor)) { return ItemKind::ArmorCloak; }

		if (armor->IsClothing())
		{
			if (isHead(armor)) { return ItemKind::ArmorClothingHead; }
			else if (isHands(armor)) { return ItemKind::ArmorClothingHands; }
			else if (isFeet(armor)) { return ItemKind::ArmorClothingFeet; }
			else { return ItemKind::ArmorClothing; }
		}
		else if (armor->IsLightArmor())
		{
			if (isHead(armor)) { return ItemKind::ArmorLightHead; }
			else if (isHands(armor)) { return ItemKind::ArmorLightHands; }
			else if (isFeet(armor)) { return ItemKind::ArmorLightFeet; }
			else { return ItemKind::ArmorLight; }
		}
		else if (armor->IsHeavyArmor())
		{
			if (isHead(armor)) { return ItemKind::ArmorHeavyHead; }
			else if (isHands(armor)) { return ItemKind::ArmorHeavyHands; }
			else if (isFeet(armor)) { return ItemKind::ArmorHeavyFeet; }
			else { return ItemKind::ArmorHeavy; }
		}

		return ItemKind::ArmorHeavy;
	}

	ItemKind subKindForMagic(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->Is(RE::FormType::Spell)) { return ItemKind::IconDefault; }

		auto* spell        = item_form->As<RE::SpellItem>();
		const auto* effect = spell->GetCostliestEffectItem()->baseEffect;
		auto skill_level   = effect->GetMinimumSkillLevel();
		auto magic_school  = effect->GetMagickSkill();
		if (magic_school == RE::ActorValue::kNone) { magic_school = effect->data.primaryAV; }

		/*
		kMagicka = 25,
		kStamina = 26,
		kHealRate = 27,
		kMagickaRate = 28,
		kStaminaRate = 29,
		kParalysis = 53,
		kInvisibility = 54,
		kWaterBreathing = 57,
		kWaterWalking = 58,
		kTelekinesis = 88,

		kCalm = 6,
		kFrenzy = 8,
		kDisarm = 9,
		kCommandSummoned = 10,
		kInvisibility = 11,
		kDarkness = 13,
		kNightEye = 14,
		kTelekinesis = 20,
		kParalysis = 21,
		kTurnUndead = 24,
 */

		switch (effect->data.archetype)
		{
			case RE::EffectArchetypes::ArchetypeID::kBoundWeapon: return ItemKind::ConjurationBoundWeapon;
			// case RE::EffectArchetypes::ArchetypeID::kCalm: return ItemKind::ConjurationSoulTrap;  // no
			case RE::EffectArchetypes::ArchetypeID::kCureDisease: return ItemKind::RestorationCure;
			case RE::EffectArchetypes::ArchetypeID::kDemoralize: return ItemKind::IllusionDemoralize;
			case RE::EffectArchetypes::ArchetypeID::kDetectLife: return ItemKind::AlterationDetect;
			// case RE::EffectArchetypes::ArchetypeID::kFrenzy: return ItemKind::ConjurationSoulTrap;  // no
			case RE::EffectArchetypes::ArchetypeID::kGuide: return ItemKind::IllusionClairvoyance;
			// case RE::EffectArchetypes::ArchetypeID::kInvisibility: return ItemKind::ConjurationSoulTrap;  // no
			case RE::EffectArchetypes::ArchetypeID::kLight: return ItemKind::AlterationLight;
			case RE::EffectArchetypes::ArchetypeID::kReanimate: return ItemKind::ConjurationZombie;
			case RE::EffectArchetypes::ArchetypeID::kSoulTrap: return ItemKind::ConjurationSoulTrap;
			case RE::EffectArchetypes::ArchetypeID::kTurnUndead: return ItemKind::RestorationSunDamage;
		}

		switch (magic_school)
		{
			case RE::ActorValue::kAlteration:
				{
					switch (effect->data.primaryAV)
					{
						case RE::ActorValue::kCarryWeight: return ItemKind::AlterationFeather;
						case RE::ActorValue::kSpeedMult: return ItemKind::AlterationWind;
						case RE::ActorValue::kWaterBreathing: return ItemKind::Alteration;
						case RE::ActorValue::kWaterWalking: return ItemKind::Alteration;
					}
					return ItemKind::Alteration;
				}

			case RE::ActorValue::kConjuration:
				{
					return ItemKind::Conjuration;
				}

			case RE::ActorValue::kDestruction:
				switch (effect->data.resistVariable)
				{
					case RE::ActorValue::kResistFire:
						{
							if (skill_level == 100) return ItemKind::DestructionFireMaster;
							else if (skill_level >= 75) return ItemKind::DestructionFireExpert;
							else if (skill_level >= 50) return ItemKind::DestructionFireAdept;
							else if (skill_level >= 25) return ItemKind::DestructionFireApprentice;
							else return ItemKind::DestructionFire;
						}
					case RE::ActorValue::kResistFrost:
						{
							if (skill_level == 100) return ItemKind::DestructionFrostMaster;
							else if (skill_level >= 75) return ItemKind::DestructionFrostExpert;
							else if (skill_level >= 50) return ItemKind::DestructionFrostAdept;
							else if (skill_level >= 25) return ItemKind::DestructionFrostApprentice;
							else return ItemKind::DestructionFrost;
						}

					case RE::ActorValue::kResistShock:
						{
							if (skill_level == 100) return ItemKind::DestructionShockMaster;
							else if (skill_level >= 75) return ItemKind::DestructionShockExpert;
							else if (skill_level >= 50) return ItemKind::DestructionShockAdept;
							else if (skill_level >= 25) return ItemKind::DestructionShockApprentice;
							else return ItemKind::DestructionShock;
						}

					default: return ItemKind::Destruction;
				}
			case RE::ActorValue::kIllusion:
				{
					switch (effect->data.primaryAV)
					{
						case RE::ActorValue::kInvisibility: return ItemKind::Illusion;
						case RE::ActorValue::kMovementNoiseMult: return ItemKind::IllusionMuffle;
						case RE::ActorValue::kParalysis: return ItemKind::SpellParalyze;
						case RE::ActorValue::kReflectDamage: return ItemKind::SpellReflect;
					}

					return ItemKind::Illusion;
				}
			case RE::ActorValue::kRestoration:
				{
					switch (effect->data.primaryAV)
					{
						case RE::ActorValue::kHealth: return ItemKind::RestorationHeal;
						case RE::ActorValue::kWardPower: return ItemKind::RestorationWard;
						case RE::ActorValue::kPoisonResist: return ItemKind::RestorationPoison;
					}

					return ItemKind::Restoration;
				}

			default: return ItemKind::SpellDefault;
		}
	}

	ItemKind subKindForConsumable(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->Is(RE::FormType::AlchemyItem)) { return ItemKind::IconDefault; }

		auto* alchemy_potion = item_form->As<RE::AlchemyItem>();
		if (alchemy_potion->IsFood()) { return ItemKind::Food; }  // TODO soup, water, meat, veggies
		if (alchemy_potion->IsPoison()) { return ItemKind::PoisonDefault; }

		auto actor_value = getPotionEffect(alchemy_potion, false);
		return subKindForConsumableByEffect(actor_value);
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
