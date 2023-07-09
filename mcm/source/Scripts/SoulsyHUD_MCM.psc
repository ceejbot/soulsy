ScriptName LamasTinyHUD_MCM Extends MCM_ConfigBase

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

string[] function GetSectionNames(int a_position) native
string function GetPage(int a_index, int a_position) native
string function GetPosition(int a_index, int a_position) native
int function GetSelectionType(int a_index, bool a_left, int a_position) native
string function GetFormString(int a_index, bool a_left, int a_position) native
int function GetSlotAction(int a_index, bool a_left, int a_position) native
int function GetHandSelection(int a_index, int a_position) native
string function GetFormName(int a_index, bool a_left, int a_position) native
function ResetSection(int a_index, int a_position) native
function SetActionValue(int a_index, bool a_left, int a_value, int a_position) native
string[] function GetConfigFiles(bool a_elden) native
string function GetActiveConfig(bool a_elden) native
function SetConfig(bool a_elden, string a_name) native
function SetActiveConfig(bool a_elden, int a_index) native
function AddUnarmedSetting(int a_position) native
string function GetActorValue(int a_index, int a_position) native

function FillPageSelection()
    string[] menu_list = GetSectionNames(GetModSettingInt("uPositionSelect:Page"))
    SetMenuOptions("uPageList:Page", menu_list, menu_list)
    SetModSettingInt("uPageList:Page", 0)
endfunction

function ResetSlot()
    ResetSection(GetModSettingInt("uPageList:Page"), GetModSettingInt("uPositionSelect:Page"))
    FillPageSelection()
    RefreshMenu()
endfunction

function CreateConfig()
    SetConfig(bElden, GetModSettingString("sSettingName:MiscSetting"))
    LoadReloadSettingFiles()
    RefreshMenu()
endfunction

function LoadReloadSettingFiles()
    SetModSettingString("sActiveSetting:MiscSetting", GetActiveConfig(bElden))
    string[] setting_list = GetConfigFiles(bElden)
    SetMenuOptions("uSettingList:MiscSetting", setting_list, setting_list)
endfunction

function AddUnarmed()
    AddUnarmedSetting(GetModSettingInt("uPositionSelect:Page"))
    FillPageSelection()
    RefreshMenu()
endfunction

Event OnSettingChange(String a_ID)
    if (a_ID == "uPageList:Page")
        int idx = GetModSettingInt(a_ID)
        int position = GetModSettingInt("uPositionSelect:Page")

        SetModSettingString("sPage:Page", GetPage(idx, position))
        SetModSettingString("sPosition:Page", GetPosition(idx, position))
        int type = GetSelectionType(idx, false, position)
        if (type < 0)
            SetModSettingInt("uType:Page", 0)
        else
            SetModSettingInt("uType:Page", type)
        endif
        ;magic, power, scroll, empty (to allow if something should be unequiped)
        if ( bElden ) 
            bSpell = false
        else
            bSpell = (type == 1) || (type == 4) || (type == 7) || (type == 8)
        endif
        
        SetModSettingInt("uHandSelection:Page", GetHandSelection(idx, position))
        SetModSettingInt("uSlotAction:Page", GetSlotAction(idx, false, position))
        SetModSettingString("sFormName:Page", GetFormName(idx, false, position))
        SetModSettingString("sSelectedItemForm:Page", GetFormString(idx, false, position))
        
        type = GetSelectionType(idx, true, position)
        if (type < 0)
            SetModSettingInt("uTypeLeft:Page", 0)
        else
            SetModSettingInt("uTypeLeft:Page", type)
        endif
        if ( bElden ) 
            bSpell = false
        else
            bSpellLeft = (type == 1) || (type == 8)
        endif
        
        SetModSettingInt("uSlotActionLeft:Page", GetSlotAction(idx, true, position))
        SetModSettingString("sFormNameLeft:Page", GetFormName(idx, true, position))
        SetModSettingString("sSelectedItemFormLeft:Page", GetFormString(idx, true, position))
        
        string actorValue = GetActorValue(idx, position)
        if (actorValue != "-1" && actorValue != "0" )
            SetModSettingString("sActorValue:Page", actorValue)
        else
            SetModSettingString("sActorValue:Page", "")
        endif
        
        RefreshMenu()
    elseif (a_ID == "uSlotAction:Page")
        int value = GetModSettingInt(a_ID)
        SetActionValue(GetModSettingInt("uPageList:Page"), False, value, GetModSettingInt("uPositionSelect:Page"))
        RefreshMenu()
    elseif (a_ID == "uSlotActionLeft:Page")
        int value = GetModSettingInt(a_ID)
        SetActionValue(GetModSettingInt("uPageList:Page"), True, value, GetModSettingInt("uPositionSelect:Page"))
        RefreshMenu()
    elseif (a_ID == "bEldenDemonSouls:MiscSetting")
        bElden = GetModSettingBool(a_ID)
        LoadReloadSettingFiles()
        RefreshMenu()
    elseif (a_ID == "uPositionSelect:Page")
        FillPageSelection()
        bUnarmed = (bElden && (GetModSettingInt(a_ID) == 1 || GetModSettingInt(a_ID) == 3)) || !bElden
        RefreshMenu()
    elseif (a_ID == "bHideOutsideCombat:MiscSetting")
        bCombat = GetModSettingBool(a_ID)
        RefreshMenu()
    elseif (a_ID == "uSettingList:MiscSetting")
        SetActiveConfig(bElden, GetModSettingInt(a_ID))
        LoadReloadSettingFiles()
        RefreshMenu()
    elseif (a_ID == "bGroupPotions:MiscSetting")
        bGroupPotions = GetModSettingBool(a_ID)
        RefreshMenu()
    elseif (a_ID == "bKeyPressToEnterEdit:Controls")
        bEditKey = GetModSettingBool(a_ID)
        RefreshMenu()
    elseif (a_ID == "bAutoCleanup:CleanupSetting")
        bCleanup = GetModSettingBool(a_ID)
        RefreshMenu()
    elseif (a_ID == "bPoisonDoseOverwrite:MiscSetting")
        bPoisonDose = GetModSettingBool(a_ID)
        RefreshMenu()
    endif
EndEvent

Event OnPageSelect(string a_page)
    if ( a_page == "$LamasTinyHUD_Pages")
        string[] menu_list = GetSectionNames(GetModSettingInt("uPositionSelect:Page"))
        SetMenuOptions("uPageList:Page", menu_list, menu_list)
        RefreshMenu()
    elseif ( a_page == "$LamasTinyHUD_HudSetting" )
        SetModSettingString("sDisplayResolutionWidth:HudSetting",GetResolutionWidth())
        SetModSettingString("sDisplayResolutionHeight:HudSetting",GetResolutionHeight())
        RefreshMenu()
    elseif ( a_page == "$LamasTinyHUD_MiscSetting" )
        LoadReloadSettingFiles()
        RefreshMenu()
    endIf
EndEvent

Event OnConfigOpen()
    bElden = GetModSettingBool("bEldenDemonSouls:MiscSetting")
    bCombat = GetModSettingBool("bHideOutsideCombat:MiscSetting")
    bUnarmed = bElden && (GetModSettingInt("uPositionSelect:Page") == 1 || GetModSettingInt("uPositionSelect:Page") == 3)
    bGroupPotions = GetModSettingBool("bGroupPotions:MiscSetting")
    bSpell = false
    bSpellLeft = false
    bEditKey = GetModSettingBool("bKeyPressToEnterEdit:Controls")
    bCleanup = GetModSettingBool("bAutoCleanup:CleanupSetting")
    bPoisonDose = GetModSettingBool("bPoisonDoseOverwrite:MiscSetting")
EndEvent