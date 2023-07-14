#include "equippable.h"

#include "player.h"
#include "lib.rs.h"

namespace equippable
{
	using slot_type = enums::slot_type;

	rust::Box<CycleEntry> cycle_entry_from_form(RE::TESForm*& item_form)
	{
		auto item_type          = equippable::get_type(item_form);
		bool has_count          = (item_type == slot_type::consumable || item_type == slot_type::scroll);
		auto count              = player::get_inventory_count(item_form);
		bool two_handed         = equippable::is_two_handed(item_form);
		std::string form_string = helpers::get_form_spec(item_form);
		auto kind               = equippable::get_icon_type(item_type, item_form);
		std::string name        = item_form->GetName();

		auto entry = create_cycle_entry(kind, two_handed, has_count, count, name, form_string);
		return entry;
	}

	bool can_instant_cast(RE::TESForm* item_form, const slot_type item_type)
	{
		if (item_type == slot_type::magic)
		{
			const auto* spell = item_form->As<RE::SpellItem>();
			if (spell->GetSpellType() == RE::MagicSystem::SpellType::kSpell ||
				spell->GetSpellType() == RE::MagicSystem::SpellType::kLeveledSpell)
			{
				if (spell->GetCastingType() != RE::MagicSystem::CastingType::kConcentration)
				{
					return true;
				}
			}
			return false;
		}

		return (item_type == slot_type::scroll);
	}

	bool is_two_handed(RE::TESForm*& item_form)
	{
		if (!item_form)
		{
			return false;
		}

		auto two_handed = false;
		if (item_form->Is(RE::FormType::Spell))
		{
			if (const auto* spell = item_form->As<RE::SpellItem>(); spell->IsTwoHanded())
			{
				two_handed = true;
			}
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

	slot_type get_type(RE::TESForm*& item_form)
	{
		if (!item_form)
		{
			return slot_type::empty;
		}

		if (item_form->IsWeapon())
		{
			if (const auto* weapon = item_form->As<RE::TESObjectWEAP>(); !weapon->IsBound())
			{
				return slot_type::weapon;
			}
		}

		if (item_form->IsArmor())
		{
			const auto* armor = item_form->As<RE::TESObjectARMO>();
			//GetSlotMask 49
			if (armor->IsShield())
			{
				return slot_type::shield;
			}
			else if (armor->IsClothing() &&
					 (armor->HasKeywordString("_WL_Lantern") &&
							 armor->HasPartOf(RE::BIPED_MODEL::BipedObjectSlot::kNone) &&
							 !armor->HasPartOf(RE::BIPED_MODEL::BipedObjectSlot::kModFaceJewelry) ||
						 armor->HasPartOf(RE::BIPED_MODEL::BipedObjectSlot::kModPelvisPrimary)))
			{
				//Wearable Lanterns got keyword _WL_Lantern
				//Simple Wearable Lanterns do not have a keyword, but will be equipped on 49 (30+19)
				return slot_type::lantern;
			}
			else if (armor->IsClothing() && armor->HasKeywordString("BOS_DisplayMaskKeyword"))
			{
				return slot_type::mask;
			}
			return slot_type::armor;
		}

		if (item_form->Is(RE::FormType::Spell))
		{
			const auto spell_type = item_form->As<RE::SpellItem>()->GetSpellType();
			if (spell_type == RE::MagicSystem::SpellType::kSpell ||
				spell_type == RE::MagicSystem::SpellType::kLeveledSpell)
			{
				return slot_type::magic;
			}
			if (spell_type == RE::MagicSystem::SpellType::kLesserPower ||
				spell_type == RE::MagicSystem::SpellType::kPower)
			{
				return slot_type::power;
			}
		}

		if (item_form->Is(RE::FormType::Shout))
		{
			return slot_type::shout;
		}

		if (item_form->Is(RE::FormType::AlchemyItem))
		{
			return slot_type::consumable;
		}

		if (item_form->Is(RE::FormType::Scroll))
		{
			return slot_type::scroll;
		}

		if (item_form->Is(RE::FormType::Ammo))
		{
			return slot_type::misc;
		}

		if (item_form->Is(RE::FormType::Light))
		{
			return slot_type::light;
		}

		return slot_type::misc;
	}

	EntryKind get_icon_type(const enums::slot_type item_type, RE::TESForm*& item_form)
	{
		switch (item_type)
		{
			case slot_type::weapon:
				return get_weapon_type_icon(item_form);
			case slot_type::magic:
				return get_spell_icon(item_form);
			case slot_type::shout:
				return EntryKind::Shout;
			case slot_type::power:
				return EntryKind::Power;
			case slot_type::consumable:
				return get_consumable_icon(item_form);
			case slot_type::shield:
				return EntryKind::Shield;
			case slot_type::armor:
				return get_armor_icon(item_form);
			case slot_type::scroll:
				return EntryKind::Scroll;
			case slot_type::light:
				return EntryKind::Torch;
			case slot_type::lantern:
				return EntryKind::Lantern;
			case slot_type::mask:
				return EntryKind::Mask;
			default:
				return EntryKind::IconDefault;
		}
	}

	EntryKind get_weapon_type_icon(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->IsWeapon())
		{
			return EntryKind::IconDefault;
		}
		switch (const auto* weapon = item_form->As<RE::TESObjectWEAP>(); weapon->GetWeaponType())
		{
			case RE::WEAPON_TYPE::kHandToHandMelee:
				return EntryKind::HandToHand;

			case RE::WEAPON_TYPE::kOneHandSword:
				if (weapon->HasKeywordString("WeapTypeRapier"))
				{
					return EntryKind::Rapier;
				}
				else if (weapon->HasKeywordString("WeapTypeKatana"))
				{
					return EntryKind::Katana;
				}
				return EntryKind::SwordOneHanded;

			case RE::WEAPON_TYPE::kOneHandDagger:
				if (weapon->HasKeywordString("WeapTypeClaw"))
				{
					return EntryKind::Claw;
				}
				return EntryKind::Dagger;

			case RE::WEAPON_TYPE::kOneHandAxe:
				return EntryKind::AxeOneHanded;

			case RE::WEAPON_TYPE::kOneHandMace:
				if (weapon->HasKeywordString("WeapTypeWhip"))
				{
					return EntryKind::Whip;
				}
				return EntryKind::Mace;

			case RE::WEAPON_TYPE::kTwoHandSword:
				if (weapon->HasKeywordString("WeapTypePike"))
				{
					return EntryKind::Pike;
				}
				return EntryKind::SwordTwoHanded;

			case RE::WEAPON_TYPE::kTwoHandAxe:
				if (weapon->HasKeywordString("WeapTypeHalberd"))
				{
					return EntryKind::Halberd;
				}
				else if (weapon->HasKeywordString("WeapTypeQtrStaff"))
				{
					return EntryKind::QuarterStaff;
				}
				return EntryKind::AxeTwoHanded;

			case RE::WEAPON_TYPE::kBow:
				return EntryKind::Bow;

			case RE::WEAPON_TYPE::kStaff:
				return EntryKind::Staff;

			case RE::WEAPON_TYPE::kCrossbow:
				return EntryKind::Crossbow;
		}

		return EntryKind::IconDefault;
	}

	EntryKind get_spell_icon(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->Is(RE::FormType::Spell))
		{
			return EntryKind::IconDefault;
		}

		auto* spell        = item_form->As<RE::SpellItem>();
		const auto* effect = spell->GetCostliestEffectItem()->baseEffect;
		auto actor_value   = effect->GetMagickSkill();
		if (actor_value == RE::ActorValue::kNone)
		{
			actor_value = effect->data.primaryAV;
		}

		switch (actor_value)
		{
			case RE::ActorValue::kAlteration:
				return EntryKind::Alteration;

			case RE::ActorValue::kConjuration:
				return EntryKind::Conjuration;

			case RE::ActorValue::kDestruction:
				switch (effect->data.resistVariable)
				{
					case RE::ActorValue::kResistFire:
						return EntryKind::DestructionFire;
					case RE::ActorValue::kResistFrost:
						return EntryKind::DestructionFrost;
					case RE::ActorValue::kResistShock:
						return EntryKind::DestructionShock;
					default:
						return EntryKind::Destruction;
				}
			case RE::ActorValue::kIllusion:
				return EntryKind::Illusion;

			case RE::ActorValue::kRestoration:
				//might not fit all spells
				return EntryKind::Restoration;

			default:
				return EntryKind::SpellDefault;
		}
	}

	EntryKind get_consumable_icon(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->Is(RE::FormType::AlchemyItem))
		{
			return EntryKind::IconDefault;
		}

		auto* alchemy_potion = item_form->As<RE::AlchemyItem>();
		if (alchemy_potion->IsFood())
		{
			return EntryKind::Food;
		}
		if (alchemy_potion->IsPoison())
		{
			return EntryKind::PoisonDefault;
		}

		auto actor_value = helpers::get_actor_value_effect_from_potion(alchemy_potion, false);
		return get_consumable_icon_by_actor_value(actor_value);
	}

	EntryKind get_armor_icon(RE::TESForm*& item_form)
	{
		if (!item_form || !item_form->IsArmor())
		{
			return EntryKind::IconDefault;
		}
		switch (const auto* armor = item_form->As<RE::TESObjectARMO>(); armor->GetArmorType())
		{
			case RE::BIPED_MODEL::ArmorType::kLightArmor:
				return EntryKind::ArmorLight;

			case RE::BIPED_MODEL::ArmorType::kHeavyArmor:
				return EntryKind::ArmorHeavy;

			case RE::BIPED_MODEL::ArmorType::kClothing:
				return EntryKind::ArmorClothing;
		}

		return EntryKind::IconDefault;
	}

	EntryKind get_consumable_icon_by_actor_value(RE::ActorValue& actor_value)
	{
		switch (actor_value)
		{
			case RE::ActorValue::kHealth:
			case RE::ActorValue::kHealRateMult:
			case RE::ActorValue::kHealRate:
				return EntryKind::PotionHealth;

			case RE::ActorValue::kStamina:
			case RE::ActorValue::kStaminaRateMult:
			case RE::ActorValue::kStaminaRate:
				return EntryKind::PotionStamina;

			case RE::ActorValue::kMagicka:
			case RE::ActorValue::kMagickaRateMult:
			case RE::ActorValue::kMagickaRate:
				return EntryKind::PotionMagicka;

			case RE::ActorValue::kResistFire:
				return EntryKind::PotionFireResist;

			case RE::ActorValue::kResistShock:
				return EntryKind::PotionShockResist;

			case RE::ActorValue::kResistFrost:
				return EntryKind::PotionFrostResist;

			case RE::ActorValue::kResistMagic:
				return EntryKind::PotionMagicResist;

			default:
				return EntryKind::PotionDefault;
		}
	}
}
