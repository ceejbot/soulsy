#include "helpers.h"

#include "constant.h"
#include "equippable.h"
#include "gear.h"
#include "player.h"
#include "ui_renderer.h"
#include "utility.h"

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
			if (sound) { game::playSound(sound->soundDescriptor, the_player); }
		}
	}

	// How you know I've been replaced by a pod person: if I ever declare that
	// I love dealing with strings in systems programming languages.

	std::vector<uint8_t> chars_to_vec(const char* input)
	{
		if (!input) { return std::move(std::vector<uint8_t>()); }
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

	std::string makeFormSpecString(RE::TESForm* form)
	{
		std::string form_string;
		if (!form) { return form_string; }

		if (form->IsDynamicForm())
		{
			// rlog::trace("it is dynamic"sv);
			form_string =
				fmt::format("{}{}{}", util::dynamic_name, util::delimiter, rlog::formatAsHex(form->GetFormID()));
		}
		else
		{
			auto* source_file = form->sourceFiles.array->front()->fileName;
			auto local_form   = form->GetLocalFormID();

			const auto hexified = rlog::formatAsHex(local_form);
			// rlog::trace("source file='{}'; local id={}'; hex={};"sv, source_file, local_form, hexified);
			form_string = fmt::format("{}{}{}", source_file, util::delimiter, hexified);
		}

		return form_string;
	}

	RE::TESForm* formSpecToFormItem(const std::string& a_str)
	{
		if (a_str.empty())
		{
			// rlog::debug("formSpecToFormItem() got empty string; this can never return an item.");
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
			rlog::warn("malformed form spec? spec={};"sv, a_str);
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
		// 	rlog::trace("found form id for form spec='{}'; name='{}'; formID={}",
		// 		a_str,
		// 		form->GetName(),
		// 		rlog::formatAsHex(form->GetFormID()));
		// }

		return form;
	}

	rust::Box<HudItem> formSpecToHudItem(const std::string& spec)
	{
		if (spec.empty())
		{
			// rlog::debug("Empty string passed to formSpecToHudItem(); returning empty item.");
			return empty_huditem();
		}
		auto* form_item = formSpecToFormItem(spec);
		if (!form_item)
		{
			rlog::debug("form item not found for form spec='{}'; Item could be from a removed mod.", spec);
			return empty_huditem();
		}
		return equippable::hudItemFromForm(form_item);
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
		rlog::info("Slow-motion: entered slow time."sv);
	}

	void exitSlowMotion()
	{
		if (!isInSlowMotion) { return; }

		auto currentMult = reinterpret_cast<float*>(getGlobalTimeMultPtr());
		// If we're already not in slow-motion, either some other mod tinkered with
		// time, or we re-entered. Pin it back down to 1.0.
		if (*currentMult >= 1.0f)
		{
			rlog::info("Slow motion: game speed={} but our flag was still set."sv, *currentMult);
			*currentMult   = 1.0f;
			isInSlowMotion = false;
			return;
		}

		const auto desiredFactor = user_settings()->slow_time_factor();
		float newFactor          = (*currentMult) / desiredFactor;
		if (std::fabs(newFactor - 1.0f) < 0.01) { newFactor = 1.0f; }
		*currentMult = newFactor;

		isInSlowMotion = false;
		rlog::info("Slow motion: returned to normal time."sv);
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

	bool isPoisonedByFormSpec(const std::string& form_spec)
	{
		auto* const form = formSpecToFormItem(form_spec);
		return game::isItemPoisoned(form);
	}

	bool isFavoritedByFormSpec(const std::string& form_spec)
	{
		auto* const form = formSpecToFormItem(form_spec);
		return game::isItemFavorited(form);
	}

	float chargeLevelByFormSpec(const std::string& form_spec)
	{
		auto* const form = formSpecToFormItem(form_spec);
		return game::itemChargeLevel(form);
	}
}
