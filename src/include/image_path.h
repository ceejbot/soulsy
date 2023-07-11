#pragma once

namespace ui
{
	constexpr auto hud_name  = "soulsy_hud";
	constexpr auto draw_full = 255;

	static std::string icon_directory                = R"(.\Data\SKSE\Plugins\resources\icons)";
	static std::string img_directory                 = R"(.\Data\SKSE\Plugins\resources\img)";
	static std::string highlight_animation_directory = R"(.\Data\SKSE\Plugins\resources\animation\highlight)";

	enum class image_type
	{
		hud,
		round,
		key,
		total
	};

	inline static std::map<std::string, image_type> image_type_name_map = { { R"(hud.svg)", image_type::hud },
		{ R"(round.svg)", image_type::round },
		{ R"(key.svg)", image_type::key } };

	inline static std::map<std::string, EntryIcon> icon_type_name_map = { 
		{ R"(alteration.svg)", EntryIcon::Alteration },
		{ R"(armor_clothing.svg)", EntryIcon::ArmorClothing },
		{ R"(armor_heavy.svg)", EntryIcon::ArmorHeavy },
		{ R"(armor_light.svg)", EntryIcon::ArmorLight },
		{ R"(arrow.svg)", EntryIcon::Arrow },
		{ R"(axe_one_handed.svg)", EntryIcon::AxeOneHanded },
		{ R"(axe_two_handed.svg)", EntryIcon::AxeTwoHanded },
		{ R"(bow.svg)", EntryIcon::Bow },
		{ R"(claw.svg)", EntryIcon::Claw },
		{ R"(conjuration.svg)", EntryIcon::Conjuration },
		{ R"(crossbow.svg)", EntryIcon::Crossbow },
		{ R"(dagger.svg)", EntryIcon::Dagger },
		{ R"(destruction_fire.svg)", EntryIcon::DestructionFire },
		{ R"(destruction_frost.svg)", EntryIcon::DestructionFrost },
		{ R"(destruction_shock.svg)", EntryIcon::DestructionShock },
		{ R"(destruction.svg)", EntryIcon::Destruction },
		{ R"(food.svg)", EntryIcon::Food },
		{ R"(halberd.svg)", EntryIcon::Halberd },
		{ R"(hand_to_hand.svg)", EntryIcon::HandToHand },
		{ R"(icon_default.svg)", EntryIcon::IconDefault },
		{ R"(illusion.svg)", EntryIcon::Illusion },
		{ R"(katana.svg)", EntryIcon::Katana },
		{ R"(lantern.svg)", EntryIcon::Lantern },
		{ R"(mace.svg)", EntryIcon::Mace },
		{ R"(mask.svg)", EntryIcon::Mask } 
		{ R"(pike.svg)", EntryIcon::Pike },
		{ R"(poison_default.svg)", EntryIcon::PoisonDefault },
		{ R"(potion_default.svg)", EntryIcon::PotionDefault },
		{ R"(potion_fire_resist.svg)", EntryIcon::PotionFireResist },
		{ R"(potion_frost_resist.svg)", EntryIcon::PotionFrostResist },
		{ R"(potion_health.svg)", EntryIcon::PotionHealth },
		{ R"(potion_magic_resist.svg)", EntryIcon::PotionMagicResist },
		{ R"(potion_magicka.svg)", EntryIcon::PotionMagicka },
		{ R"(potion_shock_resist.svg)", EntryIcon::PotionShockResist },
		{ R"(potion_stamina.svg)", EntryIcon::PotionStamina },
		{ R"(power.svg)", EntryIcon::Power },
		{ R"(quarter_staff.svg)", EntryIcon::QuarterStaff },
		{ R"(rapier.svg)", EntryIcon::Rapier },
		{ R"(restoration.svg)", EntryIcon::Restoration },
		{ R"(scroll.svg)", EntryIcon::Scroll },
		{ R"(shield.svg)", EntryIcon::Shield },
		{ R"(shout.svg)", EntryIcon::Shout },
		{ R"(spell_default.svg)", EntryIcon::SpellDefault },
		{ R"(staff.svg)", EntryIcon::Staff },
		{ R"(sword_one_handed.svg)", EntryIcon::SwordOneHanded },
		{ R"(sword_two_handed.svg)", EntryIcon::SwordTwoHanded },
		{ R"(torch.svg)", EntryIcon::Torch },
		{ R"(sword_one_handed.svg)", EntryIcon::WeaponDefault },
		{ R"(whip.svg)", EntryIcon::Whip },
	};
}
