ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase

bool property pEnableActivateModifier = false auto
bool property pEnableActivateHotkey = false auto
bool property pCycleNeedsModifier = false auto
bool property pMenuNeedsModifier = false auto
bool property pEnableUnequipModifier = false auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native

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
    ForcePageReset()
EndEvent
