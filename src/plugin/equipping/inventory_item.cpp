#include "inventory_item.h"

#include "player.h"

namespace inventory_item
{
	rust::Box<CycleEntry> cycle_entry_from_form(RE::TESForm*& item_form)
	{
		auto item_type          = inventory_item::get_type(item_form);
		bool has_count          = (item_type == slot_type::consumable || item_type == slot_type::scroll);
		auto count              = player::get_inventory_count(item_form);
		bool two_handed         = inventory_item::is_two_handed(item_form);
		std::string form_string = helpers::get_form_spec(item_form);
		auto kind               = inventory_item::get_icon_type(item_type, item_form);
		std::string name        = item_form->GetName();

		rust::Box<CycleEntry> entry = create_cycle_entry(kind, two_handed, count, count, name, form_string);
		return entry;
	}

	bool is_two_handed(RE::TESForm*& item_form)
	{
		if (!item_form)
		{
			logger::warn("return false, form is null."sv);
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

	ui::icon_image_type get_icon_type(const enums::slot_type a_type, RE::TESForm*& a_form)
	{
		auto icon = icon_type::icon_default;
		switch (a_type)
		{
			case slot_type::weapon:
				get_weapon_type_icon(a_form, icon);
				break;
			case slot_type::magic:
				get_spell_icon(a_form, icon);
				break;
			case slot_type::shout:
				icon = icon_type::shout;
				break;
			case slot_type::power:
				icon = icon_type::power;
				break;
			case slot_type::consumable:
				get_consumable_icon(a_form, icon);
				break;
			case slot_type::shield:
				icon = icon_type::shield;
				break;
			case slot_type::armor:
				get_armor_icon(a_form, icon);
				break;
			case slot_type::scroll:
				icon = icon_type::scroll;
				break;
			case slot_type::light:
				icon = icon_type::torch;
				break;
			case slot_type::lantern:
				icon = icon_type::lantern;
				break;
			case slot_type::mask:
				icon = icon_type::mask;
				break;
			case slot_type::misc:
			case slot_type::empty:
				icon = icon_type::icon_default;
				break;
		}
		return icon;
	}

	void get_weapon_type_icon(RE::TESForm*& a_form, icon_type& a_icon)
	{
		if (!a_form || !a_form->IsWeapon())
		{
			a_icon = icon_type::icon_default;
			return;
		}
		switch (const auto* weapon = a_form->As<RE::TESObjectWEAP>(); weapon->GetWeaponType())
		{
			case RE::WEAPON_TYPE::kHandToHandMelee:
				a_icon = icon_type::hand_to_hand;
				break;
			case RE::WEAPON_TYPE::kOneHandSword:
				if (weapon->HasKeywordString("WeapTypeRapier"))
				{
					a_icon = icon_type::rapier;
				}
				else if (weapon->HasKeywordString("WeapTypeKatana"))
				{
					a_icon = icon_type::katana;
				}
				else
				{
					a_icon = icon_type::sword_one_handed;
				}
				break;
			case RE::WEAPON_TYPE::kOneHandDagger:
				if (weapon->HasKeywordString("WeapTypeClaw"))
				{
					a_icon = icon_type::claw;
				}
				else
				{
					a_icon = icon_type::dagger;
				}
				break;
			case RE::WEAPON_TYPE::kOneHandAxe:
				a_icon = icon_type::axe_one_handed;
				break;
			case RE::WEAPON_TYPE::kOneHandMace:
				if (weapon->HasKeywordString("WeapTypeWhip"))
				{
					a_icon = icon_type::whip;
				}
				else
				{
					a_icon = icon_type::mace;
				}
				break;
			case RE::WEAPON_TYPE::kTwoHandSword:
				if (weapon->HasKeywordString("WeapTypePike"))
				{
					a_icon = icon_type::pike;
				}
				else
				{
					a_icon = icon_type::sword_two_handed;
				}
				break;
			case RE::WEAPON_TYPE::kTwoHandAxe:
				if (weapon->HasKeywordString("WeapTypeHalberd"))
				{
					a_icon = icon_type::halberd;
				}
				else if (weapon->HasKeywordString("WeapTypeQtrStaff"))
				{
					a_icon = icon_type::quarter_staff;
				}
				else
				{
					a_icon = icon_type::axe_two_handed;
				}
				break;
			case RE::WEAPON_TYPE::kBow:
				a_icon = icon_type::bow;
				break;
			case RE::WEAPON_TYPE::kStaff:
				a_icon = icon_type::staff;
				break;
			case RE::WEAPON_TYPE::kCrossbow:
				a_icon = icon_type::crossbow;
				break;
		}
	}

	void get_spell_icon(RE::TESForm*& a_form, icon_type& a_icon)
	{
		if (!a_form && !a_form->Is(RE::FormType::Spell))
		{
			return;
		}
		auto* spell        = a_form->As<RE::SpellItem>();
		const auto* effect = spell->GetCostliestEffectItem()->baseEffect;
		auto actor_value   = effect->GetMagickSkill();
		if (actor_value == RE::ActorValue::kNone)
		{
			actor_value = effect->data.primaryAV;
		}

		switch (actor_value)
		{
			case RE::ActorValue::kAlteration:
				a_icon = icon_type::alteration;
				break;
			case RE::ActorValue::kConjuration:
				a_icon = icon_type::conjuration;
				break;
			case RE::ActorValue::kDestruction:
				switch (effect->data.resistVariable)
				{
					case RE::ActorValue::kResistFire:
						a_icon = icon_type::destruction_fire;
						break;
					case RE::ActorValue::kResistFrost:
						a_icon = icon_type::destruction_frost;
						break;
					case RE::ActorValue::kResistShock:
						a_icon = icon_type::destruction_shock;
						break;
					default:
						a_icon = icon_type::destruction;
				}
				break;
			case RE::ActorValue::kIllusion:
				a_icon = icon_type::illusion;
				break;
			case RE::ActorValue::kRestoration:
				//might not fit all spells
				a_icon = icon_type::restoration;
				break;
			default:
				a_icon = icon_type::spell_default;
		}
	}

	void get_consumable_icon(RE::TESForm*& a_form, icon_type& a_icon)
	{
		if (!a_form || !a_form->Is(RE::FormType::AlchemyItem))
		{
			return;
		}
		auto* alchemy_potion = a_form->As<RE::AlchemyItem>();

		if (alchemy_potion->IsFood())
		{
			a_icon = icon_type::food;
			return;
		}
		if (alchemy_potion->IsPoison())
		{
			a_icon = icon_type::poison_default;
			return;
		}

		auto actor_value = helpers::get_actor_value_effect_from_potion(alchemy_potion, false);
		get_consumable_icon_by_actor_value(actor_value, a_icon);
	}

	void get_armor_icon(RE::TESForm*& a_form, icon_type& a_icon)
	{
		if (!a_form && !a_form->IsArmor())
		{
			return;
		}
		switch (const auto* armor = a_form->As<RE::TESObjectARMO>(); armor->GetArmorType())
		{
			case RE::BIPED_MODEL::ArmorType::kLightArmor:
				a_icon = icon_type::armor_light;
				break;
			case RE::BIPED_MODEL::ArmorType::kHeavyArmor:
				a_icon = icon_type::armor_heavy;
				break;
			case RE::BIPED_MODEL::ArmorType::kClothing:
				a_icon = icon_type::armor_clothing;
				break;
		}
	}

	void get_consumable_icon_by_actor_value(RE::ActorValue& a_actor_value, icon_type& a_icon)
	{
		switch (a_actor_value)
		{
			case RE::ActorValue::kHealth:
			case RE::ActorValue::kHealRateMult:
			case RE::ActorValue::kHealRate:
				a_icon = icon_type::potion_health;
				break;
			case RE::ActorValue::kStamina:
			case RE::ActorValue::kStaminaRateMult:
			case RE::ActorValue::kStaminaRate:
				a_icon = icon_type::potion_stamina;
				break;
			case RE::ActorValue::kMagicka:
			case RE::ActorValue::kMagickaRateMult:
			case RE::ActorValue::kMagickaRate:
				a_icon = icon_type::potion_magicka;
				break;
			case RE::ActorValue::kResistFire:
				a_icon = icon_type::potion_fire_resist;
				break;
			case RE::ActorValue::kResistShock:
				a_icon = icon_type::potion_shock_resist;
				break;
			case RE::ActorValue::kResistFrost:
				a_icon = icon_type::potion_frost_resist;
				break;
			case RE::ActorValue::kResistMagic:
				a_icon = icon_type::potion_magic_resist;
				break;
			default:
				a_icon = icon_type::potion_default;
		}
	}


}
