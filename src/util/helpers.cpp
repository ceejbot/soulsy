#include "helpers.h"

#include "constant.h"
#include "custom_setting.h"
#include "enums.h"
#include "gear.h"
#include "equippable.h"
#include "player.h"
#include "string_util.h"
#include "user_settings.h"
#include "ui_renderer.h"

namespace helpers
{
	using string_util = util::string_util;

	void notify_player(const std::string& message)
	{
		auto* msg = message.c_str();
		 RE::DebugNotification(msg);
	}

	void set_alpha_transition(const bool shift, const float target) {
		ui::ui_renderer::set_fade(shift, target);
	}

    bool get_is_transitioning() {
		return ui::ui_renderer::get_fade();
	}

	void toggle_hud_visibility() {
		ui::ui_renderer::toggle_show_ui();
	}

	data_helper* get_extra_data(RE::TESForm*& form)
	{
		const auto item       = new data_helper();
		const auto type       = equippable::get_type(form);
		const auto two_handed = equippable::is_two_handed(form);

		item->form       = form;
		item->type       = type;
		item->two_handed = two_handed;

		return item;
	}

	std::string get_form_spec(RE::TESForm*& form)
	{
		std::string form_string;
		logger::trace("entering get_form_spec()"sv);
		if (!form) {
			return form_string;
		}

		if (form->IsDynamicForm())
		{
			logger::trace("it is dynamic"sv);
			form_string =
				fmt::format("{}{}{}", util::dynamic_name, util::delimiter, string_util::int_to_hex(form->GetFormID()));
		}
		else
		{
			logger::trace("doing file search for form source"sv);
			//it is not, search for the file it is from
			auto* source_file = form->sourceFiles.array->front()->fileName;
			auto local_form   = form->GetLocalFormID();

			logger::trace("form is from {}, local id is {}, translated {}"sv,
				source_file,
				local_form,
				string_util::int_to_hex(local_form));

			form_string = fmt::format("{}{}{}", source_file, util::delimiter, string_util::int_to_hex(local_form));
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

		if (form->IsDynamicForm())
		{
			form_string =
				fmt::format("{}{}{}", util::dynamic_name, util::delimiter, string_util::int_to_hex(form->GetFormID()));
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

			form_string = fmt::format("{}{}{}", source_file, util::delimiter, string_util::int_to_hex(local_form));
		}

		return form_string;
	}

	std::vector<std::string> get_configured_section_page_names(uint32_t a_position)
	{
		//4 is all
		std::vector<std::string> names;
		for (const auto entries = config::custom_setting::get_sections(); const auto& entry : entries)
		{
			if (a_position == static_cast<uint32_t>(helpers::position_type::total))
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
		if (!a_str.find(util::delimiter))
		{
			return nullptr;
		}
		RE::TESForm* form;

		std::istringstream string_stream{ a_str };
		std::string plugin, id;

		std::getline(string_stream, plugin, *util::delimiter);
		std::getline(string_stream, id);
		RE::FormID form_id;
		std::istringstream(id) >> std::hex >> form_id;

		if (plugin.empty())
		{
			return nullptr;
		}

		if (plugin == util::dynamic_name)
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

	void rewrite_settings()
	{
		logger::trace("rewriting config ..."sv);
		std::map<uint32_t, uint32_t> next_page_for_position;

		for (auto i = 0; i < static_cast<int>(enums::position_type::total); ++i)
		{
			next_page_for_position[i] = 0;
		}
		std::vector<helpers::config_writer_helper*> configs;
		const auto sections = get_configured_section_page_names();
		logger::trace("got {} sections, rewrite that they are in consecutive pages"sv, sections.size());
		for (const auto& section : sections)
		{
			auto position        = config::custom_setting::get_position_by_section(section);
			const auto next_page = next_page_for_position[position];

			auto* config        = new helpers::config_writer_helper();
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
}
