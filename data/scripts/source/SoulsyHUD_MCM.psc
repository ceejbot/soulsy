ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase

import SKI_ConfigBase

bool property pEnableActivateModifier = false auto
bool property pEnableActivateHotkey = false auto
bool property pCycleNeedsModifier = false auto
bool property pMenuNeedsModifier = false auto
bool property pEnableUnequipModifier = false auto
bool property pAutoFadeGroupControl = false auto
int property pCycleToShow = 0 auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native
string[] function getPowerCycleNames() native
string[] function getUtilityCycleNames() native
string[] function getLeftCycleNames() native
string[] function getRightCycleNames() native
function ClearCycles() native

function ShowCycleEntries(int which)
    pCycleToShow = which
    string[] empty = new string[1]
    if (which == 0)
        SetMenuOptions("cycleDisplay", getPowerCycleNames(), empty)
    elseif (which == 1)
        SetMenuOptions("cycleDisplay", getUtilityCycleNames(), empty)
    elseif (which == 2)
        SetMenuOptions("cycleDisplay", getLeftCycleNames(), empty)
    elseif (which == 3)
        SetMenuOptions("cycleDisplay", getRightCycleNames(), empty)
    endif
endFunction

function ClearCyclesPapyrus()
    ClearCycles()
    ShowMessage("$SoulsyHUD_CyclesCleared_Message")
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
    elseif (changedId == "bAutoFade:Options")
        pAutoFadeGroupControl =  GetModSettingBool("bAutoFade:Options")
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

    ShowCycleEntries(pCycleToShow)

    ForcePageReset()
EndEvent
