#include "hooks.h"
#include "equippable.h"
#include "gear.h"
#include "keycodes.h"
#include "string_util.h"

#include "lib.rs.h"

namespace hooks
{
	void install_hooks()
	{
		PlayerHook::install();
		MenuHook::install();
	}

	inline const std::set<RE::FormType> RELEVANT_FORMTYPES_INVENTORY{
		RE::FormType::AlchemyItem,
		RE::FormType::Ammo,
		RE::FormType::Armor,
		RE::FormType::Light,
		RE::FormType::Scroll,
		RE::FormType::Weapon,
	};

	inline const std::set<RE::FormType> RELEVANT_FORMTYPES_ALL{
		RE::FormType::AlchemyItem,
		RE::FormType::Ammo,
		RE::FormType::Armor,
		RE::FormType::Light,
		RE::FormType::Scroll,
		RE::FormType::Shout,
		RE::FormType::Spell,
		RE::FormType::Weapon,
	};

	// ---------- MenuHook

	void MenuHook::install()
	{
		logger::info("Hooking menus to get keystrokes..."sv);

		REL::Relocation<std::uintptr_t> menu_controls_vtbl{ RE::VTABLE_MenuControls[0] };
		process_event_ = menu_controls_vtbl.write_vfunc(0x1, &MenuHook::process_event);

		logger::info("Menus hooked."sv);
	}

	RE::BSEventNotifyControl MenuHook::process_event(RE::InputEvent** eventPtr,
		RE::BSTEventSource<RE::InputEvent*>* eventSource)
	{
		auto* ui = RE::UI::GetSingleton();
		if (!ui) return process_event_(this, eventPtr, eventSource);

		auto inInventoryMenu = ui->IsMenuOpen(RE::InventoryMenu::MENU_NAME);
		auto inMagicMenu     = ui->IsMenuOpen(RE::MagicMenu::MENU_NAME);
		auto inFavoritesMenu = ui->IsMenuOpen(RE::FavoritesMenu::MENU_NAME);
		if (ui->IsMenuOpen("LootMenu") || !(inInventoryMenu || inMagicMenu || inFavoritesMenu))
		{
			return process_event_(this, eventPtr, eventSource);
		}

		auto* controlMap = RE::ControlMap::GetSingleton();
		auto* userEvents = RE::UserEvents::GetSingleton();
		if (!controlMap || !userEvents) return process_event_(this, eventPtr, eventSource);

		rust::Box<UserSettings> settings = user_settings();
		bool link_favorites              = settings->link_to_favorites();
		auto keyboardShortcut            = userEvents->togglePOV;  // m&k shortcut
		auto gamepadShortcut             = userEvents->jump;       // controller shortcut

		// TODO consider treating the favorites menu completely differently.
		if (eventPtr && *eventPtr)
		{
			for (auto* event = *eventPtr; event; event = event->next)
			{
				if (event->eventType != RE::INPUT_EVENT_TYPE::kButton || !event->HasIDCode()) { continue; }

				auto* button = static_cast<RE::ButtonEvent*>(event);
				if (button->idCode == keycodes::k_invalid) { continue; }
				auto key = keycodes::get_key_id(button);
				// I'm not getting button UP events for the inventory menu. Why? IDK.
				// I used to get those events.
				auto check_favorites =
					link_favorites && !inFavoritesMenu && (inMagicMenu ? button->IsUp() : button->IsDown());

				if (check_favorites)
				{
					if (button->GetDevice() == RE::INPUT_DEVICES::kGamepad ?
							buttonMatchesEvent(controlMap, gamepadShortcut, button) :
							buttonMatchesEvent(controlMap, keyboardShortcut, button))
					{
						helpers::MenuSelection* selection = nullptr;
						auto menu_form                    = helpers::MenuSelection::getSelectionFromMenu(ui, selection);
						if (!menu_form) continue;

						logger::debug("Got toggled favorite: form_id={}; form_type={}; is-favorited={};"sv,
							util::string_util::int_to_hex(selection->form_id),
							selection->formType,
							selection->favorite);

						if (!RELEVANT_FORMTYPES_ALL.contains(selection->formType)) { continue; }
						if (selection->form)
						{
							auto entry = equippable::hudItemFromForm(selection->form);
							handle_favorite_event(*button, selection->favorite, std::move(entry));
						}
						continue;
					}
				}

				// we send all key events to this handler because it needs to track modifiers
				auto do_toggle = handle_menu_event(key, *button);
				if (do_toggle)
				{
					helpers::MenuSelection* selection = nullptr;
					auto menu_form                    = helpers::MenuSelection::getSelectionFromMenu(ui, selection);
					if (!menu_form) continue;

					auto* item_form = selection->form;
					if (!item_form) continue;

					auto entry = equippable::hudItemFromForm(item_form);
					toggle_item(key, std::move(entry));
				}
			}
		}

		return process_event_(this, eventPtr, eventSource);
	}

	using INPUT_CONTEXT_ID = RE::UserEvents::INPUT_CONTEXT_IDS::INPUT_CONTEXT_ID;

	bool MenuHook::buttonMatchesEvent(RE::ControlMap* controlMap, RE::BSFixedString eventID, RE::ButtonEvent* button)
	{
		auto the_device = button->GetDevice();
		auto key        = controlMap->GetMappedKey(eventID, the_device, static_cast<RE::ControlMap::InputContextID>(0));
		//  logger::debug("favorites detection: looking for {} ? = {}"sv, key, button->GetIDCode());
		return key == button->GetIDCode();
	}

	// ---------- PlayerHook

	void PlayerHook::install()
	{
		logger::info("Hooking player so we get equip events plus inventory changes..."sv);

		REL::Relocation<std::uintptr_t> player_character_vtbl{ RE::VTABLE_PlayerCharacter[0] };
		add_object_to_container_ = player_character_vtbl.write_vfunc(0x5A, add_object_to_container);
		pick_up_object_          = player_character_vtbl.write_vfunc(0xCC, pick_up_object);
		remove_item_             = player_character_vtbl.write_vfunc(0x56, remove_item);

		auto& trampoline = SKSE::GetTrampoline();
		REL::Relocation<std::uintptr_t> add_item_functor_hook{ RELOCATION_ID(55946, 56490) };
		add_item_functor_ = trampoline.write_call<5>(add_item_functor_hook.address() + 0x15D, add_item_functor);
	}

	void PlayerHook::add_object_to_container(RE::Actor* a_this,
		RE::TESBoundObject* object,
		RE::ExtraDataList* extraDataList,
		int32_t count,
		RE::TESObjectREFR* a_from_refr)
	{
		add_object_to_container_(a_this, object, extraDataList, count, a_from_refr);

		if (object->IsInventoryObject())
		{
			auto item_form = RE::TESForm::LookupByID(object->formID);
			if (item_form)
			{
				// We do not pass along all inventory changes to the HUD, only changes
				// for the kinds of items the HUD is used to show.
				const auto formtype = item_form->GetFormType();
				if (!RELEVANT_FORMTYPES_INVENTORY.contains(formtype)) { return; }
				std::string form_string = helpers::makeFormSpecString(item_form);
				handle_inventory_changed(form_string, count);
			}
		}
	}

	void PlayerHook::pick_up_object(RE::Actor* actor,
		RE::TESObjectREFR* object,
		uint32_t count,
		bool a_arg3,
		bool a_play_sound)
	{
		pick_up_object_(actor, object, count, a_arg3, a_play_sound);
		if (object->GetBaseObject()->IsInventoryObject())
		{
			auto lookup = object->formID;
			if (lookup == 0) { lookup = object->GetBaseObject()->formID; }
			auto item_form = RE::TESForm::LookupByID(lookup);
			if (!item_form) { return; }

			const auto formtype = item_form->GetFormType();
			if (!RELEVANT_FORMTYPES_INVENTORY.contains(formtype)) { return; }

			std::string form_string = helpers::makeFormSpecString(item_form);
			handle_inventory_changed(form_string, count);
		}
	}

	RE::ObjectRefHandle PlayerHook::remove_item(RE::Actor* actor,
		RE::TESBoundObject* object,
		std::int32_t count,
		RE::ITEM_REMOVE_REASON a_reason,
		RE::ExtraDataList* extraDataList,
		RE::TESObjectREFR* a_move_to_ref,
		const RE::NiPoint3* a_drop_loc,
		const RE::NiPoint3* a_rotate)
	{
		if (object->IsInventoryObject())
		{
			auto* item_form = RE::TESForm::LookupByID(object->formID);
			if (item_form)
			{
				const auto formtype = item_form->GetFormType();
				if (RELEVANT_FORMTYPES_INVENTORY.contains(formtype))
				{
					std::string form_string = helpers::makeFormSpecString(item_form);
					handle_inventory_changed(form_string, -count);
				}
			}
		}

		return remove_item_(actor, object, count, a_reason, extraDataList, a_move_to_ref, a_drop_loc, a_rotate);
	}

	void PlayerHook::add_item_functor(RE::TESObjectREFR* a_this,
		RE::TESObjectREFR* object,
		int32_t count,
		bool a4,
		bool a5)
	{
		add_item_functor_(a_this, object, count, a4, a5);

		if (object->GetBaseObject()->IsInventoryObject())
		{
			auto item_form = RE::TESForm::LookupByID(object->GetBaseObject()->formID);
			if (item_form)
			{
				std::string form_string = helpers::makeFormSpecString(item_form);
				handle_inventory_changed(form_string, count);
			}
		}
	}
}
