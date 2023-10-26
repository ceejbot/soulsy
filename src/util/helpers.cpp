#include "helpers.h"

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

	// play a denied/failure/no sound
	void honk()
	{
		// UIActivateFail   0x0006d1c6
		// UIMenuInactiveSD 0x00057f93
		// UIAlchemyFail    0x000c8c72
		auto* the_player = RE::PlayerCharacter::GetSingleton();
		auto* form       = RE::TESForm::LookupByID(0x0006d1c6);
		if (form)
		{
			auto* sound = form->As<RE::BGSSoundDescriptorForm>();
			if (sound) { player::play_sound(sound->soundDescriptor, the_player); }
		}
	}

	// How you know I've been replaced by a pod person: if I ever declare that
	// I love dealing with strings in systems programming languages.

	std::vector<uint8_t> chars_to_vec(const char* input)
	{
		auto incoming_len = strlen(input);
		if (incoming_len == 0) { return std::move(std::vector<uint8_t>()); }

		std::vector<uint8_t> result;
		result.reserve(incoming_len + 1);  // null terminator
		for (auto* ptr = input; *ptr != 0; ptr++) { result.push_back(static_cast<uint8_t>(*ptr)); }
		result.push_back(0x00);  // there it is
		return std::move(result);
	}

	std::string vec_to_stdstring(rust::Vec<uint8_t> input)
	{
		if (input.size() == 0) { return std::move(std::string()); }
		auto chars  = new char[input.size()];  // the vec has a null byte terminator already
		int counter = 0;
		for (auto byte : input) { chars[counter++] = static_cast<char>(byte); }
		auto result = std::string(chars);
		delete chars;

		return std::move(result);
	}

	// See UserEvents.h -- this is kMovement | kActivate | kMenu
	// Handles photo mode and possibly others.
	static constexpr auto requiredControlFlags = static_cast<RE::ControlMap::UEFlag>(1036);

	bool ignoreKeyEvents()
	{
		// We pay attention to keypress events when:
		// - we are in normal gameplace mode
		// - the item, magic, or favorites menus are visible
		// We ignore them when other menus are up or when controls are disabled for quest reasons.

		// If we can't ask questions about the state of the UI, we respectfully decline to act.
		auto* ui = RE::UI::GetSingleton();
		if (!ui) { return true; }

		// We only want to act on button presses when in gameplay, not menus of any kind.
		if (ui->GameIsPaused() || ui->IsMenuOpen("LootMenu")) return true;
		if (!ui->IsCursorHiddenWhenTopmost() || !ui->IsShowingMenus() || !ui->GetMenu<RE::HUDMenu>()) { return true; }

		// If we're not in control of the player character or otherwise not in gameplay, move on.
		const auto* control_map = RE::ControlMap::GetSingleton();
		if (!control_map || !control_map->IsMovementControlsEnabled() ||
			!control_map->AreControlsEnabled(requiredControlFlags) || !control_map->IsActivateControlsEnabled() ||
			control_map->contextPriorityStack.back() != RE::UserEvents::INPUT_CONTEXT_ID::kGameplay)
		{
			return true;
		}

		return false;
	}

	bool gamepadInUse()
	{
		auto* inputManager = RE::BSInputDeviceManager::GetSingleton();
		return inputManager->IsGamepadEnabled() && inputManager->IsGamepadConnected();
	}

	bool relevantMenuOpen()
	{
		auto* ui = RE::UI::GetSingleton();
		return ui->IsMenuOpen(RE::InventoryMenu::MENU_NAME) || ui->IsMenuOpen(RE::MagicMenu::MENU_NAME) ||
		       ui->IsMenuOpen(RE::FavoritesMenu::MENU_NAME);
	}

	bool hudAllowedOnScreen()
	{
		// There are some circumstances where we never want to draw it.
		auto* ui              = RE::UI::GetSingleton();
		bool hudInappropriate = !ui || ui->GameIsPaused() || !ui->IsCursorHiddenWhenTopmost() ||
		                        !ui->IsShowingMenus() || !ui->GetMenu<RE::HUDMenu>() ||
		                        ui->IsMenuOpen(RE::LoadingMenu::MENU_NAME);
		if (hudInappropriate) { return false; }

		const auto* control_map = RE::ControlMap::GetSingleton();
		bool playerNotInControl =
			!control_map || !control_map->IsMovementControlsEnabled() ||
			control_map->contextPriorityStack.back() != RE::UserEvents::INPUT_CONTEXT_ID::kGameplay;
		if (playerNotInControl) { return false; }

		return true;
	}

	bool hudShouldAutoFadeIn() { return user_settings()->autofade(); }

	bool hudShouldAutoFadeOut()
	{
		if (!user_settings()->autofade()) { return false; }

		const auto player       = RE::PlayerCharacter::GetSingleton();
		const bool inCombat     = player->IsInCombat();
		const auto weaponsDrawn = player->AsActorState()->IsWeaponDrawn();

		return !inCombat && !weaponsDrawn;
	}

	void notifyPlayer(const std::string& message)
	{
		auto* msg = message.c_str();
		RE::DebugNotification(msg);
	}

	rust::String lookupTranslation(const std::string& key)
	{
		std::string translated = std::string();
		SKSE::Translation::Translate(key, translated);
		return translated;
	}

	void startAlphaTransition(const bool shift, const float target)
	{
		ui::ui_renderer::startAlphaTransition(shift, target);
	}

	void show_briefly() { ui::ui_renderer::show_briefly(); }

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

			const auto hexified = string_util::int_to_hex(local_form);
			// logger::trace("source file='{}'; local id={}'; hex={};"sv, source_file, local_form, hexified);
			form_string = fmt::format("{}{}{}", source_file, util::delimiter, hexified);
		}

		return form_string;
	}

	RE::TESForm* formSpecToFormItem(const std::string& a_str)
	{
		if (a_str.empty())
		{
			// logger::debug("formSpecToFormItem() got empty string; this can never return an item.");
			return nullptr;
		}
		if (!a_str.find(util::delimiter)) { return nullptr; }
		RE::TESForm* form;

		std::istringstream string_stream{ a_str };
		std::string plugin, id;

		std::getline(string_stream, plugin, *util::delimiter);
		std::getline(string_stream, id);
		RE::FormID form_id;
		// strip off 0x if present
		auto formline = std::istringstream(id);
		formline.ignore(2, 'x');
		formline >> std::hex >> form_id;

		if (plugin.empty())
		{
			logger::warn("malformed form spec? spec={};"sv, a_str);
			return nullptr;
		}

		if (plugin == util::dynamic_name) { form = RE::TESForm::LookupByID(form_id); }
		else
		{
			const auto data_handler = RE::TESDataHandler::GetSingleton();
			form                    = data_handler->LookupForm(form_id, plugin);
		}

		// if (form != nullptr)
		// {
		// 	logger::trace("found form id for form spec='{}'; name='{}'; formID={}",
		// 		a_str,
		// 		form->GetName(),
		// 		string_util::int_to_hex(form->GetFormID()));
		// }

		return form;
	}

	rust::Box<HudItem> formSpecToHudItem(const std::string& spec)
	{
		if (spec.empty())
		{
			// logger::debug("Empty string passed to formSpecToHudItem(); returning empty item.");
			return empty_huditem();
		}
		auto* form_item = formSpecToFormItem(spec);
		if (!form_item)
		{
			logger::debug("form item not found for form spec='{}'; Item could be from a removed mod.", spec);
			return empty_huditem();
		}
		return equippable::hudItemFromForm(form_item);
	}

	MenuSelection::MenuSelection(RE::FormID formid) : form_id(formid) {}

	void MenuSelection::makeFromFavoritesMenu(RE::FavoritesMenu* menu, MenuSelection*& outSelection)
	{
		if (!menu) return;

		RE::FormID form_id = 0;
		RE::GFxValue result;
		menu->uiMovie->GetVariable(&result, "_root.MenuHolder.Menu_mc.itemList.selectedEntry.formId");
		if (result.GetType() == RE::GFxValue::ValueType::kNumber)
		{
			form_id = static_cast<std::uint32_t>(result.GetNumber());
			logger::debug("favorites menu selection has formid {}"sv, util::string_util::int_to_hex(form_id));
		}

		auto favorites = menu->GetRuntimeData().favorites;
		for (auto favorite : favorites)
		{
			if (favorite.item->formID == form_id)
			{
				auto* selection = new MenuSelection(form_id);
				selection->form = favorite.item;
				if (favorite.entryData)
				{
					selection->favorite  = favorite.entryData->IsFavorited();
					selection->poisoned  = favorite.entryData->IsPoisoned();
					selection->equipped  = favorite.entryData->IsWorn();
					selection->count     = favorite.entryData->countDelta;  // probably wrong
					selection->bound_obj = favorite.entryData->object;
				}
				else
				{
					selection->favorite = true;  // this is the only thing we know for sure.
				}

				outSelection = selection;
			}
		}
		logger::debug("fell through without finding our object.");
	}

	void MenuSelection::makeFromInventoryMenu(RE::InventoryMenu* menu, MenuSelection*& outSelection)
	{
		if (!menu) return;

		auto* itemList = menu->GetRuntimeData().itemList;
		auto* selected = itemList->GetSelectedItem();

		if (selected && selected->data.objDesc && selected->data.objDesc->object)
		{
			auto* obj           = selected->data.objDesc->object;
			auto form_id        = obj->GetFormID();
			auto* selection     = new MenuSelection(obj->GetFormID());
			selection->count    = selected->data.GetCount();
			selection->poisoned = selected->data.objDesc->IsPoisoned();
			selection->favorite =
				!selected->data.objDesc->IsFavorited();  // We are handling button DOWN and it is toggled on button UP.
			selection->equipped  = selected->data.objDesc->IsWorn();
			selection->bound_obj = obj->IsBoundObject() ? obj : nullptr;
			selection->form      = RE::TESForm::LookupByID(form_id);
			outSelection         = selection;
			return;
		}

		// Fallback if we didn't get anything.
		RE::GFxValue result;
		//menu->uiMovie->SetPause(true);
		menu->uiMovie->GetVariable(&result, "_root.Menu_mc.inventoryLists.itemList.selectedEntry.formId");
		if (result.GetType() == RE::GFxValue::ValueType::kNumber)
		{
			RE::FormID form_id = static_cast<std::uint32_t>(result.GetNumber());
			logger::trace("formid {}"sv, util::string_util::int_to_hex(form_id));
			auto* item_form = RE::TESForm::LookupByID(form_id);
			if (!item_form) return;

			auto* player                  = RE::PlayerCharacter::GetSingleton();
			RE::TESBoundObject* bound_obj = nullptr;
			RE::ExtraDataList* extra      = nullptr;
			game::boundObjectForForm(item_form, player, bound_obj, extra);

			auto* selection     = new MenuSelection(form_id);
			selection->count    = 0;
			selection->poisoned = extra ? extra->HasType(RE::ExtraDataType::kPoison) : false;
			selection->favorite = !(extra ? extra->HasType(RE::ExtraDataType::kHotkey) : false);
			selection->equipped =
				extra ? extra->HasType(RE::ExtraDataType::kWorn) || extra->HasType(RE::ExtraDataType::kWornLeft) :
						false;
			selection->bound_obj = bound_obj;
			selection->form      = item_form;
			outSelection         = selection;
			return;
		}
	}

	void MenuSelection::makeFromMagicMenu(RE::MagicMenu* menu, MenuSelection*& outSelection)
	{
		if (!menu) return;

		RE::FormID form_id = 0;
		RE::GFxValue result;
		menu->uiMovie->GetVariable(&result, "_root.Menu_mc.inventoryLists.itemList.selectedEntry.formId");
		if (result.GetType() == RE::GFxValue::ValueType::kNumber)
		{
			form_id = static_cast<std::uint32_t>(result.GetNumber());
		}
		else
		{
			logger::debug(
				"magic menu selection lookup failed; got result type: {}"sv, static_cast<uint8_t>(result.GetType()));
		}

		auto* mfaves = RE::MagicFavorites::GetSingleton();

		for (auto* form : mfaves->spells)
		{
			logger::debug(
				"mfave form: id={}; name='{}'"sv, util::string_util::int_to_hex(form->GetFormID()), form->GetName());
			if (form->GetFormID() == form_id)
			{
				// match time
				auto* selection      = new MenuSelection(form_id);
				selection->count     = 0;
				selection->poisoned  = false;
				selection->favorite  = true;
				selection->equipped  = false;  // TODO
				selection->bound_obj = nullptr;
				selection->form      = form;

				outSelection = selection;
				return;
			}
		}

		if (form_id == 0) return;

		// If we got here and we have a form id, then we were un-favorited.
		auto* selection      = new MenuSelection(form_id);
		selection->count     = 0;      // irrelevant
		selection->poisoned  = false;  // irrelevant
		selection->favorite  = false;  // we know this
		selection->equipped  = false;  // we do not know this
		selection->bound_obj = nullptr;
		selection->form      = RE::TESForm::LookupByID(form_id);

		outSelection = selection;
	}

	uint32_t MenuSelection::getSelectionFromMenu(RE::UI*& ui, MenuSelection*& outSelection)
	{
		if (!ui) return 0;

		if (ui->IsMenuOpen(RE::InventoryMenu::MENU_NAME))
		{
			auto* inventory_menu = static_cast<RE::InventoryMenu*>(ui->GetMenu(RE::InventoryMenu::MENU_NAME).get());
			if (inventory_menu)
			{
				MenuSelection::makeFromInventoryMenu(inventory_menu, outSelection);
				if (outSelection) return outSelection->form_id;
			}
		}

		if (ui->IsMenuOpen(RE::MagicMenu::MENU_NAME))
		{
			auto* magic_menu = static_cast<RE::MagicMenu*>(ui->GetMenu(RE::MagicMenu::MENU_NAME).get());
			if (magic_menu)
			{
				MenuSelection::makeFromMagicMenu(magic_menu, outSelection);
				if (outSelection) return outSelection->form_id;
			}
		}

		if (ui->IsMenuOpen(RE::FavoritesMenu::MENU_NAME))
		{
			auto* favorite_menu = static_cast<RE::FavoritesMenu*>(ui->GetMenu(RE::FavoritesMenu::MENU_NAME).get());
			if (favorite_menu)
			{
				MenuSelection::makeFromFavoritesMenu(favorite_menu, outSelection);
				if (outSelection) return outSelection->form_id;
			}
		}

		return 0;
	}

	//  Two references for this implementation:
	//  https://github.com/Vermunds/SkyrimSoulsRE/blob/master/src/SlowMotionHandler.cpp
	//  RE/B/BStimer.h

	static bool isInSlowMotion             = false;
	static constexpr auto globalMultOffset = RELOCATION_ID(511883, 388443);

	static float* getGlobalTimeMultPtr()
	{
		float* globalMultPtr = reinterpret_cast<float*>(globalMultOffset.address());
		return globalMultPtr;
	}


	void enterSlowMotion()
	{
		if (isInSlowMotion) { return; }
		const auto desiredFactor = user_settings()->slow_time_factor();
		auto currentMult         = reinterpret_cast<float*>(getGlobalTimeMultPtr());
		auto newFactor           = desiredFactor * (*currentMult);
		*currentMult             = newFactor;

		isInSlowMotion = true;
		logger::info("Slow-motion: entered slow time."sv);
	}

	void exitSlowMotion()
	{
		if (!isInSlowMotion) { return; }

		auto currentMult = reinterpret_cast<float*>(getGlobalTimeMultPtr());
		// If we're already not in slow-motion, either some other mod tinkered with
		// time, or we re-entered. Pin it back down to 1.0.
		if (*currentMult >= 1.0f)
		{
			logger::info("Slow motion: game speed={} but our flag was still set."sv, *currentMult);
			*currentMult   = 1.0f;
			isInSlowMotion = false;
			return;
		}

		const auto desiredFactor = user_settings()->slow_time_factor();
		float newFactor          = (*currentMult) / desiredFactor;
		if (std::fabs(newFactor - 1.0f) < 0.01) { newFactor = 1.0f; }
		*currentMult = newFactor;

		isInSlowMotion = false;
		logger::info("Slow motion: returned to normal time."sv);
	}

	/*
	// TODO move to the right home
	void addCycleKeyword(const std::string& form_spec)
	{
		auto* item = formSpecToFormItem(form_spec);
		if (!item) { return; }
		// The keyword is going to be a fixed formid in the plugin esp.
		// AddKeyword(BGSKeyword* a_keyword)
		// const auto kwd = RE::TESForm::LookupByEditorID<RE::BGSKeyword>(a_edid))
		// or
		// const auto kwd = RE::TESForm::LookupByID(0x00106614)->As<RE::BGSKeyword>();
		// item->AddKeyword(kwd);
	}

	// TODO move to the right home
	void removeCycleKeyword(const std::string& form_spec)
	{
		auto* item = formSpecToFormItem(form_spec);
		if (!item) { return; }
		// bool RemoveKeyword(BGSKeyword* a_keyword)
	}
	*/

	bool itemIsFavorited(RE::TESForm* item_form)
	{
		auto* player = RE::PlayerCharacter::GetSingleton();

		if (item_form->Is(RE::FormType::Spell))
		{
			RE::TESBoundObject* bound_spell = item_form->As<RE::SpellItem>()->GetMenuDisplayObject();
			auto formid                     = bound_spell->GetFormID();
			uint32_t item_count             = 0;
			RE::ExtraDataList* extra        = nullptr;
			std::vector<RE::ExtraDataList*> extra_vector;

			std::map<RE::TESBoundObject*, std::pair<int, std::unique_ptr<RE::InventoryEntryData>>> candidates =
				player->GetInventory([formid](const RE::TESBoundObject& obj) { return obj.GetFormID() == formid; });

			for (const auto& [item, inv_data] : candidates)
			{
				if (const auto& [num_items, entry] = inv_data; entry->object->formID == formid)
				{
					// bound_obj                   = item;
					item_count                  = num_items;
					auto simple_extra_data_list = entry->extraLists;
					if (simple_extra_data_list)
					{
						for (auto* extra_data : *simple_extra_data_list)
						{
							extra = extra_data;
							extra_vector.push_back(extra_data);
							auto is_favorited = extra_data->HasType(RE::ExtraDataType::kHotkey);
							auto is_poisoned  = extra_data->HasType(RE::ExtraDataType::kPoison);
							auto worn_right   = extra_data->HasType(RE::ExtraDataType::kWorn);
							auto worn_left    = extra_data->HasType(RE::ExtraDataType::kWornLeft);
							logger::debug(
								"extra data count={}; is_favorite={}; is_poisoned={}; worn right={}, worn left={}"sv,
								extra_data->GetCount(),
								is_favorited,
								is_poisoned,
								worn_right,
								worn_left);
						}
					}
					break;
				}
			}


			// auto* obj_refr = bound_spell->As<RE::TESObjectREFR>();
			// auto extra     = obj_refr->extraList;
			// return extra.HasType(RE::ExtraDataType::kHotkey);
		}
		else if (item_form->Is(RE::FormType::Shout))
		{
			// get inventory entry data somehow
		}
		else if (item_form->Is(RE::FormType::AlchemyItem))
		{
			// ditto
		}
		else if (item_form->Is(RE::FormType::Ammo))
		{
			// yeah
		}
		else
		{
			RE::TESBoundObject* bound_obj = nullptr;
			RE::ExtraDataList* extra      = nullptr;
			game::boundObjectForForm(item_form, player, bound_obj, extra);
			if (extra) { return extra->HasType(RE::ExtraDataType::kHotkey); }
		}

		return false;
	}
}
