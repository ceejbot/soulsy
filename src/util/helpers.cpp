#include "helpers.h"

#include "constant.h"
#include "enums.h"
#include "equippable.h"
#include "gear.h"
#include "player.h"
#include "string_util.h"
#include "ui_renderer.h"
#include "user_settings.h"

#include "lib.rs.h"
namespace helpers
{
	using string_util = util::string_util;

	void notify_player(const std::string& message)
	{
		auto* msg = message.c_str();
		RE::DebugNotification(msg);
	}

	void set_alpha_transition(const bool shift, const float target) { ui::ui_renderer::set_fade(shift, target); }

	bool get_is_transitioning() { return ui::ui_renderer::get_fade(); }

	void toggle_hud_visibility() { ui::ui_renderer::toggle_show_ui(); }

	void show_hud() { ui::ui_renderer::set_show_ui(true); }

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

	std::string get_form_spec(RE::TESForm* form)
	{
		std::string form_string;
		if (!form)
		{
			return form_string;
		}

		if (form->IsDynamicForm())
		{
			// logger::trace("it is dynamic"sv);
			form_string =
				fmt::format("{}{}{}", util::dynamic_name, util::delimiter, string_util::int_to_hex(form->GetFormID()));
		}
		else
		{
			//  logger::trace("doing file search for form source"sv);
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

	uint32_t getSelectedFormFromMenu(RE::UI*& a_ui)
	{
		uint32_t menu_form = 0;
		if (a_ui->IsMenuOpen(RE::InventoryMenu::MENU_NAME))
		{
			auto* inventory_menu = static_cast<RE::InventoryMenu*>(a_ui->GetMenu(RE::InventoryMenu::MENU_NAME).get());
			if (inventory_menu)
			{
				RE::GFxValue result;
				//inventory_menu->uiMovie->SetPause(true);
				inventory_menu->uiMovie->GetVariable(&result,
					"_root.Menu_mc.inventoryLists.itemList.selectedEntry.formId");
				if (result.GetType() == RE::GFxValue::ValueType::kNumber)
				{
					menu_form = static_cast<std::uint32_t>(result.GetNumber());
					logger::trace("formid {}"sv, util::string_util::int_to_hex(menu_form));
				}
			}
		}

		if (a_ui->IsMenuOpen(RE::MagicMenu::MENU_NAME))
		{
			auto* magic_menu = static_cast<RE::MagicMenu*>(a_ui->GetMenu(RE::MagicMenu::MENU_NAME).get());
			if (magic_menu)
			{
				RE::GFxValue result;
				magic_menu->uiMovie->GetVariable(&result, "_root.Menu_mc.inventoryLists.itemList.selectedEntry.formId");
				if (result.GetType() == RE::GFxValue::ValueType::kNumber)
				{
					menu_form = static_cast<std::uint32_t>(result.GetNumber());
					logger::trace("formid {}"sv, util::string_util::int_to_hex(menu_form));
				}
			}
		}

		if (a_ui->IsMenuOpen(RE::FavoritesMenu::MENU_NAME))
		{
			auto* favorite_menu = static_cast<RE::FavoritesMenu*>(a_ui->GetMenu(RE::FavoritesMenu::MENU_NAME).get());
			if (favorite_menu)
			{
				RE::GFxValue result;
				favorite_menu->uiMovie->GetVariable(&result, "_root.MenuHolder.Menu_mc.itemList.selectedEntry.formId");
				if (result.GetType() == RE::GFxValue::ValueType::kNumber)
				{
					menu_form = static_cast<std::uint32_t>(result.GetNumber());
					logger::trace("formid {}"sv, util::string_util::int_to_hex(menu_form));
				}
			}
		}

		return menu_form;
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

}
