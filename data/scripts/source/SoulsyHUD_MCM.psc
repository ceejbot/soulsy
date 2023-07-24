ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase

; int property uPowerCycleKey = 3 Auto
; int property uUtilityCycleKey = 6 Auto
; int property uLeftCycleKey = 5 Auto
; int property uRightCycleKey = 7 Auto
; int property uUtilityActivateKey = 4 Auto
; int property uRefreshLayoutKey = 2 Auto
; int property uShowHideKey = 8 Auto
; int property uMaxCycleLength = 10 Auto
; int property uEquipDelay = 500 Auto
; bool property bAutoFade = false Auto
; int property uFadeDelay = 2000 Auto
; int property uControllerKind = 0 Auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native

Event OnSettingChange(String changedId)
    parent.OnSettingChange(changedId)
EndEvent

Event OnConfigOpen()
    parent.OnConfigOpen()
    ; uPowerCycleKey = GetModSettingInt("uPowerCycleKey:Controls")
    ; uUtilityCycleKey = GetModSettingInt("uUtilityCycleKey:Controls")
    ; uLeftCycleKey = GetModSettingInt("uLeftCycleKey:Controls")
    ; uRightCycleKey = GetModSettingInt("uRightCycleKey:Controls")
    ; uUtilityActivateKey = GetModSettingInt("uUtilityActivateKey:Controls")
    ; uRefreshLayoutKey = GetModSettingInt("uRefreshLayoutKey:Controls")
    ; uShowHideKey = GetModSettingInt("uShowHideKey:Controls")
    
    ; uMaxCycleLength = GetModSettingInt("uMaxCycleLength:Options")
    ; uEquipDelay = GetModSettingInt("uEquipDelay:Controls")
    ; bAutoFade = GetModSettingBool("bAutoFade:Options")
    ; uFadeDelay = GetModSettingInt("uFadeDelay:Options")
    ; uControllerKind = GetModSettingInt("uControllerKind::Options")
EndEvent
