﻿#include "helpers.h"

#include "constant.h"
#include "equippable.h"
#include "gear.h"
#include "player.h"
#include "string_util.h"
#include "ui_renderer.h"

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

	std::string makeFormSpecString(RE::TESForm* form)
	{
		std::string form_string;
		if (!form) { return form_string; }

		if (form->IsDynamicForm())
		{
			// logger::trace("it is dynamic"sv);
			form_string =
				fmt::format("{}{}{}", util::dynamic_name, util::delimiter, string_util::int_to_hex(form->GetFormID()));
		}
		else
		{
			auto* source_file = form->sourceFiles.array->front()->fileName;
			auto local_form   = form->GetLocalFormID();

			logger::trace("form is from {}, local id is {}, hex 0x{}"sv,
				source_file,
				local_form,
				string_util::int_to_hex(local_form));

			form_string = fmt::format("{}{}{}", source_file, util::delimiter, string_util::int_to_hex(local_form));
		}

		return form_string;
	}

	RE::TESForm* formSpecToFormItem(const std::string& a_str)
	{
		if (!a_str.find(util::delimiter)) { return nullptr; }
		RE::TESForm* form;

		std::istringstream string_stream{ a_str };
		std::string plugin, id;

		std::getline(string_stream, plugin, *util::delimiter);
		std::getline(string_stream, id);
		RE::FormID form_id;
		std::istringstream(id) >> std::hex >> form_id;

		if (plugin.empty()) { return nullptr; }

		if (plugin == util::dynamic_name) { form = RE::TESForm::LookupByID(form_id); }
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
				inventory_menu->uiMovie->GetVariable(
					&result, "_root.Menu_mc.inventoryLists.itemList.selectedEntry.formId");
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
}