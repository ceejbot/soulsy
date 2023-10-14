ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase
import SKI_ConfigBase

bool property pEnableActivateModifier = false auto
bool property pEnableActivateHotkey = false auto
bool property pCycleNeedsModifier = false auto
bool property pMenuNeedsModifier = false auto
bool property pEnableUnequipModifier = false auto
bool property pEnableLongPressMatchOption = true auto

int property pCycleToShow = 0 auto
int property pCycleItemShown  = 0 auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native
string[] function GetCycleFormIDs(int which) native
string[] function GetCycleNames(int which) native
function ClearCycles() native

string property pEquipSetMenuSelection = "" auto
int property pSelectedEquipSet = 0 auto
int property pSelectedEquipSetId = 0 auto
string property pIconSourceSelection = "" auto
int property pIconSourceInt = 0 auto

bool function HandleCreateEquipSet(string name) native
bool function HandleRenameEquipSet(int id, string name) native
bool function HandleUpdateEquipSet(int id) native
bool function HandleRemoveEquipSet(int id) native
string[] function GetEquipSetNames() native
string[] function GetEquipSetIDs() native
int function StringToInt(string number) native
int function FindSelectedSetID(string name) native
string[] function GetEquipSetItemNames(int id) native
bool function SetItemAsEquipSetIcon(int id, string name) native

; equip sets
function CreateEquipSet()
    string newname = GetModSettingString("sLastUsedSetName:Equipsets")
    if HandleCreateEquipSet(newname)
        ShowMessage("$SoulsyHUD_SetCreated_Msg")
        UpdateEquipSetMenu()
    else
        ShowMessage("$SoulsyHUD_CreateFail_Msg")
    endif
endFunction

function UpdateEquipSetMenu()
    string[] names = GetEquipSetNames()
    ;string[] ids = GetEquipSetIDs()
    if (names.Length == 0)
        ;ids = new string[2]
        ;ids[0] = "$SoulsyHUD_NoEquipSets"
        ;ids[1] = " "
        names = new string[2]
        names[0] = "$SoulsyHUD_NoEquipSets"
        names[1] = ""
    endif
    ; MCMHelper crashes every time if you pass two arrays here. Alas.
    SetMenuOptions("pEquipSetMenuSelection", names);

    pSelectedEquipSetId = FindSelectedSetID(pEquipSetMenuSelection)
    SetModSettingString("sLastEditedSetName:Equipsets", pEquipSetMenuSelection)
endFunction

function UpdateEquipSetItemMenu()
	string[] items = GetEquipSetItemNames(pSelectedEquipSetId)
    if (items.Length == 0)
        items = new string[2]
        items[0] = "$SoulsyHUD_NoEquipSets"
        items[1] = " "
    endif

    SetMenuOptions("pIconSourceSelection", items)
    pIconSourceSelection = items[0]
endFunction

function UseSelectionAsIcon()
	if SetItemAsEquipSetIcon(pSelectedEquipSetId, pIconSourceSelection)
		ShowMessage("$SoulsyHUD_IconSet_Msg")
	else
		ShowMessage("$SoulsyHUD_IconSetFailed_Msg")
	endif
endFunction

function RenameEquipSet()
	if pSelectedEquipSetId == -1
		; TODO honk() or otherwise signal error
		return
	endif
    string newname = GetModSettingString("sLastEditedSetName:Equipsets")
    if HandleRenameEquipSet(pSelectedEquipSetId, newname)
        ShowMessage("$SoulsyHUD_SetRenamed_Msg")
        UpdateEquipSetMenu()
    else
        ShowMessage("$SoulsyHUD_RenameFail_Msg")
    endif
endFunction

function UpdateEquipSet()
	if pSelectedEquipSetId == -1
		; TODO honk() or otherwise signal error
		return
	endif
    if HandleUpdateEquipSet(pSelectedEquipSetId)
        ShowMessage("$SoulsyHUD_SetUpdated_Msg")
    else
        ShowMessage("$SoulsyHUD_UpdateFail_Msg")
    endif
endFunction

function RemoveEquipSet()
	if pSelectedEquipSetId == -1
		; TODO honk() or otherwise signal error
		return
	endif
    if HandleRemoveEquipSet(pSelectedEquipSetId)
        ShowMessage("$SoulsyHUD_SetRemoved_Msg")
        UpdateEquipSetMenu()
    else
        ShowMessage("$SoulsyHUD_RemoveFail_Msg")
    endif
endFunction

; Regular cycle entries, for the preview page
function ShowCycleEntries(int which)
    pCycleToShow = which
    string[] options = GetCycleNames(which)
    if (options.Length == 0)
        options = new string[2]
        options[0] = "$SoulsyHUD_NoCycleItems"
        options[1] = ""
    endif
    SetMenuOptions("cycleDisplay", a_options = options)
endFunction

function ClearCyclesPapyrus()
    bool doit = ShowMessage("$SoulsyHUD_AreYouSure_Message", a_withCancel = true)
    if (doit)
        ClearCycles()
        ShowMessage("$SoulsyHUD_CyclesCleared_Message")
        pCycleItemShown = 0
        ShowCycleEntries(pCycleToShow)
    endif
endFunction

Event OnSettingChange(String changedID)
    parent.OnSettingChange(changedID)

    int menuEnum = GetModSettingInt("uHowToggleInMenus:Controls")
    int cycleEnum = GetModSettingInt("uHowToCycle:Controls")

    if (changedID == "uHowToActivate:Controls")
        int activateEnum = GetModSettingInt("uHowToActivate:Controls")
        pEnableActivateModifier = (activateEnum == 2)
        pEnableActivateHotkey = (activateEnum == 0)
    elseif (changedID == "uHowToCycle:Controls")
        pCycleNeedsModifier = (cycleEnum == 2)
    elseif (changedID == "uHowToggleInMenus:Controls")
        pMenuNeedsModifier = (menuEnum == 2)
    elseif (changedID == "uHowToUnequip:Controls")
        int unequipEnum = GetModSettingInt("uHowToUnequip:Controls")
        pEnableUnequipModifier = (unequipEnum == 2)
        if unequipEnum == 1
            SetModSettingBool("bLongPressMatches:Controls", false)
        endif
    elseif (changedId == "pEquipSetMenuSelection")
        pSelectedEquipSetId = FindSelectedSetID(pEquipSetMenuSelection)
    	SetModSettingString("sLastEditedSetName:Equipsets", pEquipSetMenuSelection)
        UpdateEquipSetItemMenu()
    endif

    int equipDelay = GetModSettingInt("uEquipDelay:Options")
    int longPress = GetModSettingInt("uLongPressMillis:Options")
    if longPress <= equipDelay
        SetModSettingInt("uLongPressMillis:Options", equipDelay + 250)
    endif

    ForcePageReset()
EndEvent

Event OnConfigOpen()
    parent.OnConfigOpen()

    int menuEnum = GetModSettingInt("uHowToggleInMenus:Controls")
    pMenuNeedsModifier = (menuEnum == 2)
    int cycleEnum = GetModSettingInt("uHowToCycle:Controls")
    pCycleNeedsModifier = (cycleEnum == 2)

    int activateEnum = GetModSettingInt("uHowToActivate:Controls")
    pEnableActivateModifier = (activateEnum == 2)
    pEnableActivateHotkey = (activateEnum == 0)

    int unequipEnum = GetModSettingInt("uHowToUnequip:Controls")
    pEnableUnequipModifier = (unequipEnum == 2)
    pEnableLongPressMatchOption = (unequipEnum != 1)

    ForcePageReset()

    ShowCycleEntries(pCycleToShow)

    UpdateEquipSetMenu()
    UpdateEquipSetItemMenu()

    RefreshMenu()
EndEvent
