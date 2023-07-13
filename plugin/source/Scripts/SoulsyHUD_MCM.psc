ScriptName SoulsyHUD_MCM Extends MCM_ConfigBase

bool property bChangeable Auto
bool property bSpell Auto
bool property bSpellLeft Auto
bool property bElden Auto
bool property bCombat Auto
bool property bUnarmed Auto
bool property bGroupPotions Auto
bool property bEditKey Auto
bool property bCleanup Auto
bool property bPoisonDose Auto

Event OnConfigClose() native
string function GetResolutionWidth() native
string function GetResolutionHeight() native

Event OnSettingChange(String changedId)
    ; thh I'm not sure why we're doing this
    if (changedId == "bFade:Options")
        bFade = GetModSettingBool(changedId)
        RefreshMenu()
    endif
EndEvent

Event OnConfigOpen()
    uPowerCycleKey = GetModSettingInt("uPowerCycleKey:Controls")
    uUtilityCycleKey = GetModSettingInt("uUtilityCycleKey:Controls")
    uLeftCycleKey = GetModSettingInt("uLeftCycleKey:Controls")
    uRightCycleKey = GetModSettingInt("uRightCycleKey:Controls")
    uUtilityActivateKey = GetModSettingInt("uUtilityActivateKey:Controls")
    uShowHideKey = GetModSettingInt("uShowHideKey:Controls")
    uMaxCycleLength = GetModSettingInt("uMaxCycleLength:Options")
    uEquipDelay = GetModSettingInt("uEquipDelay:Controls")
    bFade = GetModSettingBool("bFade:Options")
    uFadeDelay = GetModSettingInt("fFadeDelay:Options")
EndEvent
