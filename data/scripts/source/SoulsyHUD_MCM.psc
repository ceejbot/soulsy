ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase

bool property pEnableActivateModifier = 0 auto
bool property pEnableActivateHotkey = 0 auto
bool property pEnableCycleModifier = 0 auto
bool property pEnableUnequipModifier = 0 auto

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
        pEnableCycleModifier = ((menuEnum == 2) || (cycleEnum = 2))
    elseif (changedID == "uHowToggleInMenus:Controls")
        pEnableCycleModifier = ((menuEnum == 2) || (cycleEnum = 2))
    elseif (changedID == "uHowToUnequip:Controls")
        int unequipEnum = GetModSettingInt("uHowToUnequip:Controls")
        pEnableUnequipModifier = (unequipEnum == 2)
    endif

    RefreshMenu()
EndEvent

Event OnConfigOpen()
    parent.OnConfigOpen()

    int menuEnum = GetModSettingInt("uHowToggleInMenus:Controls")
    int cycleEnum = GetModSettingInt("uHowToCycle:Controls")
    pEnableCycleModifier = ((menuEnum == 2) || (cycleEnum = 2))

    int activateEnum = GetModSettingInt("uHowToActivate:Controls")
    pEnableActivateModifier = (activateEnum == 2)
    pEnableActivateHotkey = (activateEnum == 0)        

    int unequipEnum = GetModSettingInt("uHowToUnequip:Controls")
    pEnableUnequipModifier = (unequipEnum == 2)
EndEvent
