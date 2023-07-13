ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase

; bool property uPowerCycleKey Auto
; bool property uUtilityCycleKey Auto
; bool property uLeftCycleKey Auto
; bool property uRightCycleKey Auto
; bool property uUtilityActivateKey Auto
; bool property uRefreshLayoutKey Auto
; bool property uShowHideKey Auto
; bool property uMaxCycleLength Auto
; bool property uEquipDelay Auto
; bool property bFade Auto
; bool property uFadeDelay Auto
; bool property uControllerKind Auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native

event onPageReset(string page)
    ; urk?
endEvent

Event OnSettingChange(String changedId)
    ; thh I'm not sure why we're doing this
EndEvent

Event OnConfigOpen()
    ; uPowerCycleKey = GetModSettingInt("uPowerCycleKey:Controls")
    ; uUtilityCycleKey = GetModSettingInt("uUtilityCycleKey:Controls")
    ; uLeftCycleKey = GetModSettingInt("uLeftCycleKey:Controls")
    ; uRightCycleKey = GetModSettingInt("uRightCycleKey:Controls")
    ; uUtilityActivateKey = GetModSettingInt("uUtilityActivateKey:Controls")
    ; uRefreshLayoutKey = GetModSettingInt("uRefreshLayoutKey:Controls")
    ; uShowHideKey = GetModSettingInt("uShowHideKey:Controls")
    
    ; uMaxCycleLength = GetModSettingInt("uMaxCycleLength:Options")
    ; uEquipDelay = GetModSettingInt("uEquipDelay:Controls")
    ; bFade = GetModSettingBool("bFade:Options")
    ; uFadeDelay = GetModSettingInt("fFadeDelay:Options")
    ; uControllerKind = GetModSettingInt("uControllerKind::Options")
EndEvent
