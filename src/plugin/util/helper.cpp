#include "include/helper.h"
#include "include/constant.h"
#include "include/gear.h"
#include "include/user_settings.h"
#include "include/custom_setting.h"
#include "include/string_util.h"
#include "include/player.h"
// #include "data/config_writer_helper.h"


namespace helpers
{
	data_helper* get_extra_data(RE::TESForm*& form)
	{
		const auto item       = new data_helper();
		const auto type       = helpers::get_type(form);
		const auto two_handed = helpers::is_two_handed(form);

		item->form       = form;
		item->type       = type;
		item->two_handed = two_handed;

		return item;
	}

	ItemData* buildCycleEntry(RE::TESForm*& form)
	{
		auto* item       = new ItemData();
		item->form       = form;
		item->type       = helpers::get_type(form);
		item->two_handed = helpers::is_two_handed(form);
		item->formspec   = get_form_spec(form);
		// action_type action_type    = action_type::default_action;
		// bool has_count             = false;
		// RE::ActorValue actor_value = RE::ActorValue::kNone;
		// RE::BGSEquipSlot* slot     = nullptr;
	}

	std::string get_form_spec(const RE::TESForm& form)
	{
		if (form->IsDynamicForm())
		{
			form_string = fmt::format("{}{}{}", dynamic_name, delimiter, string_util::int_to_hex(form->GetFormID()));
		}
		else
		{
			//it is not, search for the file it is from
			auto* source_file = form->sourceFiles.array->front()->fileName;
			auto local_form   = form->GetLocalFormID();

			logger::trace("form is from {}, local id is {}, translated {}"sv,
				source_file,
				local_form,
				string_util::int_to_hex(local_form));

			form_string = fmt::format("{}{}{}", source_file, delimiter, string_util::int_to_hex(local_form));
		}

		return form_string;
	}

	std::string get_mod_and_form(const RE::FormID& a_form_id)
	{
		std::string form_string;
		if (!a_form_id)
		{
			return form_string;
		}

		const auto* form = RE::TESForm::LookupByID(a_form_id);
		logger::trace("Item is {}, formid {}, formid not translated {}. return."sv,
			form->GetName(),
			string_util::int_to_hex(form->GetFormID()),
			form->GetFormID());

		return get_form_spec(form);
	}

	std::vector<std::string> get_configured_section_page_names(uint32_t a_position)
	{
		//4 is all
		std::vector<std::string> names;
		for (const auto entries = config::custom_setting::get_sections(); const auto& entry : entries)
		{
			if (a_position == static_cast<uint32_t>(handle::position_setting::position_type::total))
			{
				names.emplace_back(entry.pItem);
			}
			else
			{
				auto section_position = config::custom_setting::get_position_by_section(entry.pItem);
				if (section_position == a_position)
				{
					names.emplace_back(entry.pItem);
				}
			}
		}
		logger::trace("got {} sections, for position {}"sv, names.size(), a_position);
		return names;
	}

	RE::TESForm* get_form_from_mod_id_string(const std::string& a_str)
	{
		if (!a_str.find(delimiter))
		{
			return nullptr;
		}
		RE::TESForm* form;

		std::istringstream string_stream{ a_str };
		std::string plugin, id;

		std::getline(string_stream, plugin, *delimiter);
		std::getline(string_stream, id);
		RE::FormID form_id;
		std::istringstream(id) >> std::hex >> form_id;

		if (plugin.empty())
		{
			return nullptr;
		}

		if (plugin == dynamic_name)
		{
			form = RE::TESForm::LookupByID(form_id);
		}
		else
		{
			logger::trace("checking mod {} for form {}"sv, plugin, form_id);

			const auto data_handler = RE::TESDataHandler::GetSingleton();
			form                    = data_handler->LookupForm(form_id, plugin);
		}

		if (form != nullptr)
		{
			logger::trace("got form id {}, name {}", string_util::int_to_hex(form->GetFormID()), form->GetName());
		}

		return form;
	}

	bool is_two_handed(RE::TESForm*& a_form)
	{
		if (!a_form)
		{
			logger::warn("return false, form is null."sv);
			return false;
		}

		auto two_handed = false;
		if (a_form->Is(RE::FormType::Spell))
		{
			if (const auto* spell = a_form->As<RE::SpellItem>(); spell->IsTwoHanded())
			{
				two_handed = true;
			}
		}
		else if (a_form->IsWeapon())
		{
			if (const auto* weapon = a_form->As<RE::TESObjectWEAP>();
				weapon->IsTwoHandedAxe() || weapon->IsTwoHandedSword() || weapon->IsBow() || weapon->IsCrossbow())
			{
				two_handed = true;
			}
		}

		//logger::trace("form {}, two handed {}"sv, a_form->GetName(), two_handed);
		return two_handed;
	}

	slot_type get_type(RE::TESForm*& a_form)
	{
		if (!a_form)
		{
			return slot_type::empty;
		}

		if (a_form->IsWeapon())
		{
			if (const auto* weapon = a_form->As<RE::TESObjectWEAP>(); !weapon->IsBound())
			{
				return slot_type::weapon;
			}
		}

		if (a_form->IsArmor())
		{
			const auto* armor = a_form->As<RE::TESObjectARMO>();
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

		if (a_form->Is(RE::FormType::Spell))
		{
			const auto spell_type = a_form->As<RE::SpellItem>()->GetSpellType();
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

		if (a_form->Is(RE::FormType::Shout))
		{
			return slot_type::shout;
		}

		if (a_form->Is(RE::FormType::AlchemyItem))
		{
			return slot_type::consumable;
		}

		if (a_form->Is(RE::FormType::Scroll))
		{
			return slot_type::scroll;
		}

		if (a_form->Is(RE::FormType::Ammo))
		{
			return slot_type::misc;
		}

		if (a_form->Is(RE::FormType::Light))
		{
			return slot_type::light;
		}

		return slot_type::misc;
	}

	void rewrite_settings()
	{
		logger::trace("rewriting config ..."sv);
		std::map<uint32_t, uint32_t> next_page_for_position;

		for (auto i = 0; i < static_cast<int>(handle::position_setting::position_type::total); ++i)
		{
			next_page_for_position[i] = 0;
		}
		std::vector<config_writer_helper*> configs;
		const auto sections = get_configured_section_page_names();
		logger::trace("got {} sections, rewrite that they are in consecutive pages"sv, sections.size());
		for (const auto& section : sections)
		{
			auto position        = config::custom_setting::get_position_by_section(section);
			const auto next_page = next_page_for_position[position];

			auto* config        = new config_writer_helper();
			config->section     = section;
			config->page        = next_page;
			config->position    = position;
			config->form        = config::custom_setting::get_item_form_by_section(section);
			config->type        = config::custom_setting::get_type_by_section(section);
			config->hand        = config::custom_setting::get_hand_selection_by_section(section);
			config->action      = config::custom_setting::get_slot_action_by_section(section);
			config->form_left   = config::custom_setting::get_item_form_left_by_section(section);
			config->type_left   = config::custom_setting::get_type_left_by_section(section);
			config->action_left = config::custom_setting::get_slot_action_left_by_section(section);
			config->actor_value = config::custom_setting::get_effect_actor_value(section);

			configs.push_back(config);
			next_page_for_position[position] = next_page + 1;
		}

		logger::trace("start writing config, got {} items"sv, configs.size());

		for (const auto config : configs)
		{
			config::custom_setting::reset_section(config->section);
			const auto section = get_section_name_for_page_position(config->page, config->position);

			config::custom_setting::write_section_setting(section,
				config->page,
				config->position,
				config->type,
				config->form,
				config->action,
				config->hand,
				config->type_left,
				config->form_left,
				config->action_left,
				config->actor_value);
		}

		next_page_for_position.clear();
		configs.clear();
		logger::trace("done rewriting."sv);
	}

	std::string get_section_name_for_page_position(const uint32_t a_page, const uint32_t a_position)
	{
		//for now, I will just generate it
		return fmt::format("Page{}Position{}", a_page, a_position);
	}

	RE::ActorValue get_actor_value_effect_from_potion(RE::TESForm* a_form, bool a_check)
	{
		if (!a_form->Is(RE::FormType::AlchemyItem) || (!config::mcm_setting::get_group_potions() && a_check))
		{
			return RE::ActorValue::kNone;
		}

		auto* alchemy_potion = a_form->As<RE::AlchemyItem>();
		if (alchemy_potion->IsFood() || alchemy_potion->IsPoison())
		{
			return RE::ActorValue::kNone;
		}

		const auto* effect = alchemy_potion->GetCostliestEffectItem()->baseEffect;
		auto actor_value   = effect->GetMagickSkill();
		if (actor_value == RE::ActorValue::kNone)
		{
			actor_value = effect->data.primaryAV;
		}

		if (!a_check)
		{
			return actor_value;
		}

		if ((actor_value == RE::ActorValue::kHealth || actor_value == RE::ActorValue::kStamina ||
				actor_value == RE::ActorValue::kMagicka) &&
			effect->data.flags.none(RE::EffectSetting::EffectSettingData::Flag::kRecover))
		{
			return actor_value;
		}

		return RE::ActorValue::kNone;
	}

	void write_setting_to_file(const uint32_t a_page,
		const uint32_t a_position,
		const std::vector<data_helper*>& a_data,
		const uint32_t a_hand)
	{
		const auto section = get_section_name_for_page_position(a_page, a_position);
		auto type          = static_cast<uint32_t>(slot_type::empty);
		std::string form_string;
		uint32_t action            = 0;
		RE::ActorValue actor_value = RE::ActorValue::kNone;

		auto type_left = static_cast<uint32_t>(slot_type::empty);
		std::string form_string_left;
		uint32_t action_left = 0;

		if (a_data.empty())
		{
			return;
		}

		if (config::mcm_setting::get_elden_demon_souls())
		{
			if (!a_data.empty())
			{
				if (a_data[0]->left)
				{
					type_left = static_cast<uint32_t>(a_data[0]->type);
					if (a_data[0]->form)
					{
						form_string_left = get_mod_and_form(a_data[0]->form->formID);
					}
					else
					{
						form_string_left = "";
					}
					action_left = static_cast<uint32_t>(a_data[0]->action_type);
				}
				else
				{
					type = static_cast<uint32_t>(a_data[0]->type);
					if (a_data[0]->form)
					{
						form_string = get_mod_and_form(a_data[0]->form->formID);
					}
					else
					{
						form_string = "";
					}
					action = static_cast<uint32_t>(a_data[0]->action_type);
				}
				actor_value = a_data[0]->actor_value;
			}
		}
		else
		{
			if (!a_data.empty())
			{
				type = static_cast<uint32_t>(a_data[0]->type);
				if (a_data[0]->form)
				{
					form_string = get_mod_and_form(a_data[0]->form->formID);
				}
				else
				{
					form_string = "";
				}
				action      = static_cast<uint32_t>(a_data[0]->action_type);
				actor_value = a_data[0]->actor_value;
			}


			if (a_data.size() == 2)
			{
				type_left = static_cast<uint32_t>(a_data[1]->type);
				if (a_data[1]->form)
				{
					form_string_left = get_mod_and_form(a_data[1]->form->formID);
				}
				else
				{
					form_string_left = "";
				}
				action_left = static_cast<uint32_t>(a_data[1]->action_type);
				actor_value = a_data[1]->actor_value;
			}
		}
		config::mcm_setting::read_setting();

		config::custom_setting::write_section_setting(section,
			a_page,
			a_position,
			type,
			form_string,
			action,
			a_hand,
			type_left,
			form_string_left,
			action_left,
			static_cast<int>(actor_value));
		config::custom_setting::read_setting();
	}

	bool can_instant_cast(RE::TESForm* a_form, const slot_type a_type)
	{
		if (a_type == slot_type::magic)
		{
			const auto* spell = a_form->As<RE::SpellItem>();
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

		if (a_type == slot_type::power)
		{
			return false;
		}

		if (a_type == slot_type::scroll)
		{
			return true;
		}

		if (a_type == slot_type::shout)
		{
			return false;
		}

		return false;
	}
}
