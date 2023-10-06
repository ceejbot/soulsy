ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase
import SKI_ConfigBase

import SKI_ConfigBase

bool property pEnableActivateModifier = false auto
bool property pEnableActivateHotkey = false auto
bool property pCycleNeedsModifier = false auto
bool property pMenuNeedsModifier = false auto
bool property pEnableUnequipModifier = false auto
bool property pAutoFadeGroupControl = false auto
bool property pEnableLongPressMatchOption = true auto

int property pCycleToShow = 0 auto
int property pCycleItemShown  = 0 auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native
string[] function GetCycleFormIDs(int which) native
string[] function GetCycleNames(int which) native
function ClearCycles() native

string property pNewEquipSetName = "New Equipset" auto
string property pSelectedSetName = "" auto
int property pSelectedEquipSet = 0 auto

bool function HandleCreateEquipSet(string name) native
bool function HandleRenameEquipSet(int id, string name) native
bool function HandleUpdateEquipSet(int id) native
bool function HandleRemoveEquipSet(int id) native
string[] function GetEquipSetNames() native
int[] function GetEquipSetIDs() native

; equip sets
function CreateEquipSet(int which)
    if HandleCreateEquipSet(pNewEquipSetName)
        ShowMessage("Equip set created")
        ShowEquipsetMenu()
    else
        ShowMessage("equip set not created; look at the logs")
    endif
endFunction

function selectedSetID
	int[] ids = GetEquipSetIDs()
	if ids.Length > pSelectedEquipSet
		return ids[pSelectedEquipSet]
	else
		return -1
	endif
endFunction

function RenameEquipSet(int which)
	int idToChange = selectedSetID()
	if idToChange == -1
		; TODO honk() or otherwise signal error
		return
	endif
    if HandleRenameEquipSet(idToChange, pSelectedSetName)
        ShowMessage("Equip set renamed")
        ShowEquipsetMenu()
    else
        ShowMessage("equip set not renamed; look at the logs")
    endif
endFunction

function UpdateEquipSet()
	int idToChange = selectedSetID()
	if idToChange == -1
		; TODO honk() or otherwise signal error
		return
	endif
    if HandleUpdateEquipSet(idToChange)
        ShowMessage("Equip set updated with what you're wearing")
    else
        ShowMessage("equip set not updated; look at the logs")
    endif
endFunction

function RemoveEquipSet()
	int idToChange = selectedSetID()
	if idToChange == -1
		; TODO honk() or otherwise signal error
		return
	endif
    if HandleRemoveEquipSet(idToChange)
        ShowMessage("Equip set removed")
        ShowEquipsetMenu()
    else
        ShowMessage("equip set not removed; look at the logs")
    endif
endFunction

function ShowEquipsetMenu()
    string[] names = GetEquipSetNames()
    if (names.Length == 0)
        names = new string[2]
        names[0] = "$SoulsyHUD_empty_list"
        names[1] = ""
    endif

    SetMenuOptions("equipSetList", a_options = names)
endFunction

; Regular cycle entries, for the preview page
function ShowCycleEntries(int which)
    pCycleToShow = which
    string[] options = GetCycleNames(which)
    if (options.Length == 0)
        options = new string[2]
        options[0] = "$SoulsyHUD_empty_list"
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
    elseif (changedId == "bAutoFade:Options")
        pAutoFadeGroupControl =  GetModSettingBool("bAutoFade:Options")
    endif

    int equipDelay = GetModSettingInt("uEquipDelay:Options")
    int longPress = GetModSettingInt("uLongPressMillis:Options")
    if longPress <= equipDelay
        SetModSettingInt("uLongPressMillis:Options", equipDelay + 250)
    endif

    ForcePageReset()
    ShowEquipsetMenu()
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
    ShowEquipsetMenu()
EndEvent
