#pragma once

class MenuHook : public RE::MenuControls
{
public:
	static void install();

private:
	RE::BSEventNotifyControl process_event(RE::InputEvent** a_event, RE::BSTEventSource<RE::InputEvent*>* a_source);

	using process_event_type =
		decltype(static_cast<RE::BSEventNotifyControl (RE::MenuControls::*)(RE::InputEvent* const*,
				RE::BSTEventSource<RE::InputEvent*>*)>(&RE::MenuControls::ProcessEvent));
	static inline REL::Relocation<process_event_type> process_event_;
	static bool buttonMatchesEvent(RE::ControlMap* controlMap, RE::BSFixedString eventStr, RE::ButtonEvent* button);
};

struct MenuSelection
{
	static uint32_t getSelectionFromMenu(RE::UI*& a_ui, MenuSelection*& outSelection);
	static uint32_t makeFromFavoritesMenu(RE::FavoritesMenu* menu, MenuSelection*& outSelection);
	static void makeFromMagicMenu(RE::MagicMenu* menu, MenuSelection*& outSelection);
	static void makeFromInventoryMenu(RE::InventoryMenu* menu, MenuSelection*& outSelection);

	MenuSelection(RE::FormID formid);
	MenuSelection(RE::TESBoundObject* boundObject);

	RE::FormID form_id;
	bool favorite;
	bool poisoned;
	bool equipped;
	RE::FormType formType;
	uint32_t count;
	RE::TESBoundObject* bound_obj;
	RE::TESForm* form;
};
