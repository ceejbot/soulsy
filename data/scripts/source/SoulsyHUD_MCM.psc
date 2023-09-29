ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase

import SKI_ConfigBase

bool property pEnableActivateModifier = false auto
bool property pEnableActivateHotkey = false auto
bool property pCycleNeedsModifier = false auto
bool property pMenuNeedsModifier = false auto
bool property pEnableUnequipModifier = false auto
bool property pAutoFadeGroupControl = false auto
bool property pBuildIsPreAE = false auto
int property pCycleToShow = 0 auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native
string[] function getPowerCycleNames() native
string[] function getUtilityCycleNames() native
string[] function getLeftCycleNames() native
string[] function getRightCycleNames() native
bool function BuildIsPreAE() native
function ClearCycles() native

function ShowCycleEntries(int which)
    if (BuildIsPreAE()) 
        return
    endif
    pCycleToShow = which
    
    if (which == 0)
        SetMenuOptions("cycleDisplay", getPowerCycleNames())
    elseif (which == 1)
        SetMenuOptions("cycleDisplay", getUtilityCycleNames())
    elseif (which == 2)
        SetMenuOptions("cycleDisplay", getLeftCycleNames())
    elseif (which == 3)
        SetMenuOptions("cycleDisplay", getRightCycleNames())
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

    pBuildIsPreAE = BuildIsPreAE()

    int menuEnum = GetModSettingInt("uHowToggleInMenus:Controls")
    pMenuNeedsModifier = (menuEnum == 2)
    int cycleEnum = GetModSettingInt("uHowToCycle:Controls")
    pCycleNeedsModifier = (cycleEnum == 2)

    int activateEnum = GetModSettingInt("uHowToActivate:Controls")
    pEnableActivateModifier = (activateEnum == 2)
    pEnableActivateHotkey = (activateEnum == 0)

    int unequipEnum = GetModSettingInt("uHowToUnequip:Controls")
    pEnableUnequipModifier = (unequipEnum == 2)

    if (!pBuildIsPreAE)
        ShowCycleEntries(pCycleToShow)
    endif

    ForcePageReset()
EndEvent
