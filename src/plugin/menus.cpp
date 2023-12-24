#include "menus.h"

#include "equippable.h"
#include "gear.h"
#include "keycodes.h"
#include "log.h"
#include "util/string_util.h"

#include "lib.rs.h"

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
	rlog::info("Hooking menus to get keystrokes...");

	REL::Relocation<std::uintptr_t> menu_controls_vtbl{ RE::VTABLE_MenuControls[0] };
	process_event_ = menu_controls_vtbl.write_vfunc(0x1, &MenuHook::process_event);

	rlog::info("Menus hooked."sv);
}

bool MenuHook::buttonMatchesEvent(RE::ControlMap* controlMap, RE::BSFixedString eventID, RE::ButtonEvent* button)
{
	auto the_device = button->GetDevice();
	auto key        = controlMap->GetMappedKey(eventID, the_device, static_cast<RE::ControlMap::InputContextID>(0));
	//  rlog::debug("favorites detection: looking for {} ? = {}"sv, key, button->GetIDCode());
	return key == button->GetIDCode();
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
			if (button->idCode == keycodes::kInvalid) { continue; }
			auto key = keycodes::keyID(button);
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
					MenuSelection* selection = nullptr;
					auto menu_form           = MenuSelection::getSelectionFromMenu(ui, selection);
					if (!menu_form) continue;

					rlog::debug("Got toggled favorite: form_id={}; form_type={}; is-favorited={};"sv,
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

			// We send all key events to this handler because it needs to track modifiers.
			// It returns true if we should act on this event.
			if (!handle_menu_event(key, *button)) { continue; }

			MenuSelection* selection = nullptr;
			auto menu_form           = MenuSelection::getSelectionFromMenu(ui, selection);
			if (!menu_form) { continue; }

			auto* item_form = selection->form;
			if (!item_form) { continue; }

			auto entry = equippable::hudItemFromForm(item_form);
			toggle_item(key, std::move(entry));

			// We know this event was something we acted on. We suppress it so downstream
			// handlers do not also act upon it by doing something annoying like changing what's
			// shown in the menu.
			button->idCode    = keycodes::kInvalid;
			button->userEvent = "";
		}
	}

	return process_event_(this, eventPtr, eventSource);
}

// ---------- MenuSelection

MenuSelection::MenuSelection(RE::FormID formid) : form_id(formid)
{
	if (formid == 0) { return; }
	auto* item_form = RE::TESForm::LookupByID(formid);
	if (!item_form) { return; }

	this->form = item_form;

	auto* player                    = RE::PlayerCharacter::GetSingleton();
	RE::TESBoundObject* boundObject = nullptr;
	game::EquippableItemData* data  = nullptr;
	game::boundObjectForForm(item_form, player, boundObject, data);

	if (boundObject)
	{
		this->bound_obj = boundObject;
		this->formType  = boundObject->GetFormType();
	}
	else { this->formType = item_form->GetFormType(); }
}

MenuSelection::MenuSelection(RE::TESBoundObject* boundObject) : bound_obj(boundObject)
{
	if (!boundObject) { return; }
	this->formType = boundObject->GetFormType();
	this->form_id  = boundObject->GetFormID();
	this->form     = boundObject->As<RE::TESForm>();
}


uint32_t MenuSelection::makeFromFavoritesMenu(RE::FavoritesMenu* menu, MenuSelection*& outSelection)
{
	if (!menu) return 0;

	RE::FormID form_id = 0;
	RE::GFxValue result;
	menu->uiMovie->GetVariable(&result, "_root.MenuHolder.Menu_mc.itemList.selectedEntry.formId");
	if (result.GetType() == RE::GFxValue::ValueType::kNumber)
	{
		form_id = static_cast<std::uint32_t>(result.GetNumber());
		// rlog::debug("favorites menu selection has formid {}"sv, util::string_util::int_to_hex(form_id));
	}
	if (form_id == 0) { return 0; }

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
			return selection->form_id;
		}
	}
	rlog::debug("fell through without finding our object.");
	return 0;
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
		rlog::trace("formid {}"sv, util::string_util::int_to_hex(form_id));
		auto* item_form = RE::TESForm::LookupByID(form_id);
		if (!item_form) return;

		auto* player                   = RE::PlayerCharacter::GetSingleton();
		RE::TESBoundObject* bound_obj  = nullptr;
		game::EquippableItemData* data = nullptr;
		game::boundObjectForForm(item_form, player, bound_obj, data);

		auto* selection     = new MenuSelection(form_id);
		selection->count    = 0;
		selection->poisoned = data ? data->isPoisoned : false;
		selection->favorite = data ? data->isFavorite : false;
		selection->equipped = data ? data->isWorn || data->isWornLeft : false;
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
		rlog::debug(
			"magic menu selection lookup failed; got result type: {}"sv, static_cast<uint8_t>(result.GetType()));
	}

	auto* mfaves = RE::MagicFavorites::GetSingleton();

	for (auto* form : mfaves->spells)
	{
		rlog::debug(
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

	outSelection = std::move(selection);
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
		if (favorite_menu) { return MenuSelection::makeFromFavoritesMenu(favorite_menu, outSelection); }
	}

	return 0;
}
