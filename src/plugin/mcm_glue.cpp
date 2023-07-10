#include "papyrus.h"

#include "control/binding.h"
#include "enums.h"
#include "processing/set_setting_data.h"
#include "setting/custom_setting.h"
#include "setting/file_setting.h"
#include "setting/mcm_setting.h"
#include "ui/ui_renderer.h"
#include "util/constant.h"
#include "util/helper.h"

static const char* mcm_name = "SoulsyHUD_MCM";

// TODO in this file: remove functions at the same time they're removed from the papyrus script
// trimming everything down to the 1 page of settings defined in the mcm config json
// rename as needed

void register_papyrus_functions()
{
	const auto* papyrus = SKSE::GetPapyrusInterface();
	papyrus->Register(MCMGlue::Register);
	logger::info("Registered papyrus functions. return."sv);
}

void MCMGlue::on_config_close(RE::TESQuest*)
{
	logger::info("on config close"sv);
	// handle configuration changes-- call out to Rust to inform it
	config::mcm_setting::read_setting();
	if (config::mcm_setting::get_elden_demon_souls())
	{
		helper::rewrite_settings();
	}
	processing::set_setting_data::read_and_set_data();
	processing::set_setting_data::get_actives_and_equip();
	control::binding::get_singleton()->set_all_keys();
	//In case the setting was changed
	// ceej: what's the side effect here? just a redraw?
	ui::ui_renderer::set_fade(true, 1.f);

	logger::debug("on config close done. return."sv);
}

RE::BSFixedString MCMGlue::get_resolution_width(RE::TESQuest*)
{
	return fmt::format(FMT_STRING("{:.2f}"), ui::ui_renderer::get_resolution_width());
}

RE::BSFixedString MCMGlue::get_resolution_height(RE::TESQuest*)
{
	return fmt::format(FMT_STRING("{:.2f}"), ui::ui_renderer::get_resolution_height());
}

std::vector<RE::BSFixedString> MCMGlue::get_section_names(RE::TESQuest*, uint32_t a_position)
{
	const auto sections = helper::get_configured_section_page_names(a_position);
	std::vector<RE::BSFixedString> sections_bs_string;
	sections_bs_string.reserve(sections.size());
	for (const auto& section : sections)
	{
		sections_bs_string.emplace_back(get_form_name_string_for_section(section));
	}
	logger::trace("Returning {} sections for Position {}"sv, sections_bs_string.size(), a_position);
	return sections_bs_string;
}

RE::BSFixedString MCMGlue::get_page(RE::TESQuest*, const uint32_t a_index, uint32_t a_position)
{
	logger::trace("page was requested for index {}"sv, a_index);
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		return std::to_string(config::custom_setting::get_page_by_section(section));
	}
	return "";
}

RE::BSFixedString MCMGlue::get_position(RE::TESQuest*, const uint32_t a_index, uint32_t a_position)
{
	logger::trace("position was requested for index {}"sv, a_index);
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		return std::to_string(config::custom_setting::get_position_by_section(section));
	}
	return "";
}

uint32_t MCMGlue::get_selection_type(RE::TESQuest*, const uint32_t a_index, const bool a_left, uint32_t a_position)
{
	uint32_t type = 0;
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		if (a_left)
		{
			type = config::custom_setting::get_type_left_by_section(section);
		}
		else
		{
			type = config::custom_setting::get_type_by_section(section);
		}
	}
	logger::trace("return type {} index {}"sv, type, a_index);
	return type;
}

RE::BSFixedString
	MCMGlue::get_form_string(RE::TESQuest*, const uint32_t a_index, const bool a_left, uint32_t a_position)
{
	std::string form_string;
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		if (a_left)
		{
			form_string = config::custom_setting::get_item_form_left_by_section(section);
		}
		else
		{
			form_string = config::custom_setting::get_item_form_by_section(section);
		}
	}
	return form_string;
}

uint32_t MCMGlue::get_slot_action(RE::TESQuest*, const uint32_t a_index, const bool a_left, uint32_t a_position)
{
	uint32_t action = 0;
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		if (a_left)
		{
			action = config::custom_setting::get_slot_action_left_by_section(section);
		}
		else
		{
			action = config::custom_setting::get_slot_action_by_section(section);
		}
	}
	logger::trace("return action {} index {}"sv, action, a_index);
	return action;
}

uint32_t MCMGlue::get_hand_selection(RE::TESQuest*, const uint32_t a_index, uint32_t a_position)
{
	uint32_t hand = 0;
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		hand = config::custom_setting::get_hand_selection_by_section(section);
	}
	logger::trace("return hand {} index {}"sv, hand, a_index);
	return hand;
}

RE::BSFixedString MCMGlue::get_form_name(RE::TESQuest*, const uint32_t a_index, const bool a_left, uint32_t a_position)
{
	std::string form_string;
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		if (a_left)
		{
			form_string = config::custom_setting::get_item_form_left_by_section(section);
		}
		else
		{
			form_string = config::custom_setting::get_item_form_by_section(section);
		}
	}

	if (form_string.empty())
	{
		return form_string;
	}

	const auto* form = helper::get_form_from_mod_id_string(form_string);
	if (!form)
	{
		return form_string;
	}

	return form->GetName();
}

void MCMGlue::reset_section(RE::TESQuest*, const uint32_t a_index, uint32_t a_position)
{
	logger::trace("reset section was called for index {}"sv, a_index);
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		config::custom_setting::reset_section(section);
	}
}

void MCMGlue::set_action_value(RE::TESQuest*,
	const uint32_t a_index,
	const bool a_left,
	const uint32_t a_value,
	uint32_t a_position)
{
	logger::trace("set action was called for index {}, left {}, value {}"sv, a_index, a_left, a_value);
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		if (a_left)
		{
			config::custom_setting::write_slot_action_left_by_section(section, a_value);
		}
		else
		{
			config::custom_setting::write_slot_action_by_section(section, a_value);
		}
	}
}

std::vector<RE::BSFixedString> MCMGlue::get_config_files(RE::TESQuest*, bool a_elden)
{
	logger::trace("getting config files for elden {}"sv, a_elden);
	auto files = search_for_config_files(a_elden);
	std::vector<RE::BSFixedString> file_list;
	file_list.reserve(files.size());
	for (const auto& file : files)
	{
		file_list.emplace_back(file);
	}

	return file_list;
}

RE::BSFixedString MCMGlue::get_active_config(RE::TESQuest*, bool a_elden)
{
	auto file = a_elden ? config::file_setting::get_config_elden() : config::file_setting::get_config_default();
	logger::trace("getting active Config File, Elden {}, File {}"sv, a_elden, file);
	return file;
}

void MCMGlue::set_config(RE::TESQuest*, bool a_elden, RE::BSFixedString a_name)
{  // NOLINT(performance-unnecessary-value-param)
	std::string name;
	if (a_elden)
	{
		name = util::ini_elden_name + "_" + a_name.data() + util::ini_ending;
		if (check_name(name))
		{
			config::file_setting::set_config_elden(name);
		}
		else
		{
			logger::warn("Did not set new file, already exists, name {}"sv, name);
		}
	}
	else
	{
		name = util::ini_default_name + "_" + a_name.data() + util::ini_ending;
		if (check_name(name))
		{
			config::file_setting::set_config_default(name);
		}
		else
		{
			logger::warn("Did not set new file, already exists, name {}"sv, name);
		}
	}
	logger::trace("set config elden {}, file {}"sv, a_elden, name);
}

void MCMGlue::set_active_config(RE::TESQuest*, bool a_elden, uint32_t a_index)
{
	auto files = search_for_config_files(a_elden);
	auto file  = a_elden ? util::ini_elden_name + util::ini_ending : util::ini_default_name + util::ini_ending;
	if (!files.empty() && is_size_ok(a_index, files.size()))
	{
		file = files.at(a_index);
	}

	if (a_elden)
	{
		config::file_setting::set_config_elden(file);
	}
	else
	{
		config::file_setting::set_config_default(file);
	}
}

void MCMGlue::add_unarmed_setting(RE::TESQuest*, uint32_t a_position)
{
	auto elden = config::mcm_setting::get_elden_demon_souls();
	logger::trace("Try to add Unarmed for Position {}, Elden {}"sv, a_position, elden);
	auto* page_handle = handle::page_handle::get_singleton();
	auto position     = static_cast<handle::position_setting::position_type>(a_position);
	std::vector<data_helper*> data;
	auto next_page = 0;
	if (elden && (a_position == static_cast<uint32_t>(handle::position_setting::position_type::right) ||
					 a_position == static_cast<uint32_t>(handle::position_setting::position_type::left)))
	{
		auto left      = a_position == static_cast<uint32_t>(handle::position_setting::position_type::left);
		auto max_pages = config::mcm_setting::get_max_page_count();

		auto highest_page = page_handle->get_highest_page_id_position(position);
		if (static_cast<int>(max_pages) == highest_page)
		{
			logger::warn("can not add Unarmed already enough settings"sv);
			return;
		}

		for (auto i = 0; i <= highest_page; ++i)
		{
			auto* page = page_handle->get_page_setting(i, position);
			//in theory in elden there should be just one setting in the list
			auto* setting = page->slot_settings.front();
			if (setting->form && setting->form->formID == util::unarmed)
			{
				logger::warn("Already got a Unarmed Setting in this Position. Return"sv);
				return;
			}
		}

		next_page         = highest_page + 1;
		const auto item   = new data_helper();
		item->type        = enums::slot_type::weapon;
		item->left        = left;
		item->form        = RE::TESForm::LookupByID(util::unarmed);  //unarmed
		item->two_handed  = false;
		item->action_type = enums::action_type::default_action;
		data.push_back(item);
	}
	else
	{
		next_page = static_cast<int>(page_handle->get_active_page_id());

		const auto item   = new data_helper();
		item->form        = RE::TESForm::LookupByID(util::unarmed);
		item->left        = false;
		item->type        = enums::slot_type::weapon;
		item->action_type = enums::action_type::default_action;
		data.push_back(item);

		const auto item2   = new data_helper();
		item2->form        = RE::TESForm::LookupByID(util::unarmed);
		item2->left        = true;
		item2->type        = enums::slot_type::weapon;
		item2->action_type = enums::action_type::default_action;
		data.push_back(item2);
	}
	processing::set_setting_data::set_single_slot(next_page, position, data);
	logger::trace("Added Unarmed Setting Page {}, Position {}, Setting Count {}"sv, next_page, a_position, data.size());
}

RE::BSFixedString MCMGlue::get_actor_value(RE::TESQuest*, uint32_t a_index, uint32_t a_position)
{
	std::string form_string;
	if (const auto section = get_section_by_index(a_index, a_position); !section.empty())
	{
		form_string = std::to_string(config::custom_setting::get_effect_actor_value(section));
	}
	return form_string;
}

bool MCMGlue::Register(RE::BSScript::IVirtualMachine* a_vm)
{
	a_vm->RegisterFunction("OnConfigClose", mcm_name, on_config_close);
	a_vm->RegisterFunction("GetResolutionWidth", mcm_name, get_resolution_width);
	a_vm->RegisterFunction("GetResolutionHeight", mcm_name, get_resolution_height);

	a_vm->RegisterFunction("GetSectionNames", mcm_name, get_section_names);
	a_vm->RegisterFunction("GetPage", mcm_name, get_page);
	a_vm->RegisterFunction("GetPosition", mcm_name, get_position);
	a_vm->RegisterFunction("GetSelectionType", mcm_name, get_selection_type);
	a_vm->RegisterFunction("GetFormString", mcm_name, get_form_string);
	a_vm->RegisterFunction("GetSlotAction", mcm_name, get_slot_action);
	a_vm->RegisterFunction("GetHandSelection", mcm_name, get_hand_selection);
	a_vm->RegisterFunction("GetFormName", mcm_name, get_form_name);
	a_vm->RegisterFunction("ResetSection", mcm_name, reset_section);
	a_vm->RegisterFunction("SetActionValue", mcm_name, set_action_value);
	a_vm->RegisterFunction("GetConfigFiles", mcm_name, get_config_files);
	a_vm->RegisterFunction("GetActiveConfig", mcm_name, get_active_config);
	a_vm->RegisterFunction("SetConfig", mcm_name, set_config);
	a_vm->RegisterFunction("SetActiveConfig", mcm_name, set_active_config);
	a_vm->RegisterFunction("AddUnarmedSetting", mcm_name, add_unarmed_setting);
	a_vm->RegisterFunction("GetActorValue", mcm_name, get_actor_value);

	logger::info("Registered {} class. return."sv, mcm_name);
	return true;
}

bool MCMGlue::is_size_ok(uint32_t a_idx, uint64_t a_size)
{
	if (a_idx > a_size)
	{
		logger::warn("Index is {} but size is just {}, does not fit. return."sv, a_idx, a_size);
		return false;
	}
	return true;
}

std::string MCMGlue::get_section_by_index(const uint32_t a_index, uint32_t a_position)
{
	std::string section;
	if (const auto sections = helper::get_configured_section_page_names(a_position);
		!sections.empty() && is_size_ok(a_index, sections.size()))
	{
		section = sections.at(a_index);
	}
	logger::trace("got section {} for index {}"sv, section, a_index);
	return section;
}
bool MCMGlue::check_name(const std::string& a_name)
{
	//check if the file exists
	auto files = search_for_config_files(true);
	if (!files.empty() && std::find(files.begin(), files.end(), a_name) != files.end())
	{
		return false;
	}
	files = search_for_config_files(false);
	if (!files.empty() && std::find(files.begin(), files.end(), a_name) != files.end())
	{
		return false;
	}
	if (a_name == util::ini_elden_name || a_name == util::ini_default_name)
	{
		return false;
	}

	return true;
}

std::vector<std::string> MCMGlue::search_for_config_files(bool a_elden)
{
	std::vector<std::string> file_list;
	auto file_name = util::ini_default_name;
	if (a_elden)
	{
		file_name = util::ini_elden_name;
	}

	logger::trace("Will start looking in Path {}"sv, util::ini_path);
	if (std::filesystem::is_directory(util::ini_path))
	{
		for (const auto& entry : std::filesystem::directory_iterator(util::ini_path))
		{
			if (is_regular_file(entry) && entry.path().extension() == util::ini_ending &&
				entry.path().filename().string().starts_with(file_name))
			{
				logger::trace("found file {}, path {}"sv, entry.path().filename().string(), entry.path().string());
				if (!a_elden && entry.path().filename().string().starts_with(util::ini_elden_name))
				{
					logger::warn("Skipping File {}, because it would also match for Elden"sv,
						entry.path().filename().string());
					continue;
				}
				file_list.push_back(entry.path().filename().string());
			}
		}
	}
	logger::trace("Got {} Files to return in Path"sv, file_list.size());
	return file_list;
}

std::string MCMGlue::get_form_name_string_for_section(const std::string& a_str)
{
	std::string display_string;
	auto form_string      = config::custom_setting::get_item_form_by_section(a_str);
	auto form_string_left = config::custom_setting::get_item_form_left_by_section(a_str);

	RE::TESForm* form = nullptr;
	if (!form_string.empty())
	{
		form = helper::get_form_from_mod_id_string(form_string);
	}
	RE::TESForm* form_left = nullptr;
	if (!form_string_left.empty())
	{
		form_left = helper::get_form_from_mod_id_string(form_string_left);
	}

	//if form is null check if av is set
	//if form is 1f4
	if (form && form->formID == util::unarmed)
	{
		display_string = util::unarmed_mcm_text;
	}
	else
	{
		display_string = form ? form->GetName() : "";
	}

	if (form_left)
	{
		if (!display_string.empty())
		{
			display_string = display_string + util::delimiter;
		}
		if (form_left->formID == util::unarmed)
		{
			display_string = display_string + util::unarmed_mcm_text;
		}
		else
		{
			display_string = display_string + form_left->GetName();
		}
	}

	if (display_string.empty())
	{
		auto actor_value = static_cast<RE::ActorValue>(config::custom_setting::get_effect_actor_value(a_str));
		if (util::actor_value_to_base_potion_map_.contains(actor_value))
		{
			auto* potion_form = RE::TESForm::LookupByID(util::actor_value_to_base_potion_map_[actor_value]);
			display_string    = potion_form ? potion_form->GetName() : "";
		}
	}

	return display_string.empty() ? a_str : display_string;
}
