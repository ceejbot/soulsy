#include "include/user_settings.h"

namespace config {
    static const char* mcm_default_setting = R"(.\Data\MCM\Config\SoulsyHUD\settings.ini)";
    static const char* mcm_config_setting = R"(.\Data\MCM\Settings\SoulsyHUD.ini)";

    static uint32_t top_action_key;
    static uint32_t right_action_key;
    static uint32_t bottom_action_key;
    static uint32_t left_action_key;
    static uint32_t toggle_key;
    static uint32_t show_hide_key;
    static bool key_press_to_enter_edit;
    static uint32_t edit_key;
    static uint32_t left_or_overwrite_edit_key;
    static uint32_t remove_key;
    static bool bottom_execute_key_combo_only;
    static uint32_t controller_set;

    static float hud_image_scale_width;
    static float hud_image_scale_height;
    static float hud_image_position_width;
    static float hud_image_position_height;
    static float hud_slot_position_offset_x;
    static float hud_slot_position_offset_y;
    static float hud_key_position_offset;
    static float icon_scale_width;
    static float icon_scale_height;
    static float key_icon_scale_width;
    static float key_icon_scale_height;
    static float hud_arrow_image_scale_width;
    static float hud_arrow_image_scale_height;
    static float arrow_icon_scale_width;
    static float arrow_icon_scale_height;
    static float master_scale;
    static float slot_count_text_offset;
    static float toggle_key_offset_x;
    static float toggle_key_offset_y;
    static float current_items_offset_x;
    static float current_items_offset_y;
    static float slot_item_name_offset_horizontal_x;
    static float slot_item_name_offset_horizontal_y;
    static float slot_item_name_offset_vertical_x;
    static float slot_item_name_offset_vertical_y;
    static float arrow_slot_offset_x;
    static float arrow_slot_offset_y;
    static float arrow_slot_count_text_offset;
    static float current_shout_offset_x;
    static float current_shout_offset_y;

    static uint32_t background_transparency;
    static uint32_t background_icon_transparency;
    static uint32_t icon_transparency;
    static uint32_t key_transparency;
    static uint32_t current_items_transparency;
    static uint32_t current_shout_transparency;
    static uint32_t slot_count_transparency;
    static uint32_t slot_item_name_transparency;
    static uint32_t icon_transparency_blocked;
    static float slot_count_text_font_size;
    static float current_items_font_size;
    static float arrow_count_font_size;
    static uint32_t current_items_red;
    static uint32_t current_items_green;
    static uint32_t current_items_blue;
    static uint32_t slot_count_red;
    static uint32_t slot_count_green;
    static uint32_t slot_count_blue;
    static uint32_t slot_item_red;
    static uint32_t slot_item_green;
    static uint32_t slot_item_blue;
    static uint32_t slot_button_feedback;
    static bool draw_current_items_text;
    static bool draw_item_name_text;
    static bool draw_toggle_button;
    static bool draw_current_shout_text;
    static float current_shout_font_size;
    static float item_name_font_size;
    static bool draw_page_id;

    static uint32_t alpha_slot_animation;
    static float duration_slot_animation;

    static bool action_check;
    static bool empty_hand_setting;
    static bool hide_outside_combat;
    static float fade_timer_outside_combat;
    static bool disable_input_quick_loot;
    static bool elder_demon_souls;
    static uint32_t max_page_count;
    static uint32_t max_ammunition_type;
    static bool check_duplicate_items;
    static bool un_equip_ammo;
    static bool only_favorite_ammo;
    static bool prevent_consumption_of_last_dynamic_potion;
    static bool group_potions;
    static float potion_min_perfect;
    static float potion_max_perfect;
    static bool disable_re_equip_of_actives;
    static bool sort_arrow_by_quantity;
    static bool overwrite_poison_dose;
    static uint32_t apply_poison_dose;
    static bool try_dual_cast_top_spell;

    static bool auto_cleanup;
    static bool clean_armor;
    static bool clean_weapon;
    static bool clean_spell;
    static bool clean_alchemy_item;
    static bool clean_shout;
    static bool clean_light;
    static bool clean_scroll;

    void mcm_setting::read_setting() {
        logger::info("reading mcm ini files");

        const auto read_mcm = [&](const std::filesystem::path& path) {
            CSimpleIniA mcm;
            mcm.SetUnicode();
            mcm.LoadFile(path.string().c_str());

            top_action_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uTopActionKey", 10));
            right_action_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uRightActionKey", 11));
            bottom_action_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uBottomActionKey", 12));
            left_action_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uLeftActionKey", 13));
            toggle_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uToggleKey", 27));
            show_hide_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uShowHideKey", 26));
            key_press_to_enter_edit = mcm.GetBoolValue("Controls", "bKeyPressToEnterEdit", false);
            edit_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uKeyToEnterEdit", 22));
            left_or_overwrite_edit_key =
                static_cast<uint32_t>(mcm.GetLongValue("Controls", "uLeftOrOverwriteEditKey", 38));
            remove_key = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uRemoveKey", 37));

            bottom_execute_key_combo_only = mcm.GetBoolValue("Controls", "bBottomExecuteKeyComboOnly", false);
            controller_set = static_cast<uint32_t>(mcm.GetLongValue("Controls", "uControllerSet", 0));

            hud_image_scale_width = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudImageScaleWidth", 0.16));
            hud_image_scale_height = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudImageScaleHeight", 0.16));
            hud_image_position_width =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudImagePositionWidth", 200));
            hud_image_position_height =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudImagePositionHeight", 775));
            hud_slot_position_offset_x =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudSlotPositionOffsetX", 105));
            hud_slot_position_offset_y =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudSlotPositionOffsetY", 105));
            hud_key_position_offset = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudKeyPositionOffset", 38));
            icon_scale_width = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fIconScaleWidth", 0.10));
            icon_scale_height = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fIconScaleHeight", 0.10));
            key_icon_scale_width = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fKeyIconScaleWidth", 0.28));
            key_icon_scale_height = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fKeyIconScaleHeight", 0.28));
            hud_arrow_image_scale_width =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudArrowImageScaleWidth", 0.09));
            hud_arrow_image_scale_height =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fHudArrowImageScaleHeight", 0.09));
            arrow_icon_scale_width = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fArrowIconScaleWidth", 0.05));
            arrow_icon_scale_height =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fArrowIconScaleHeight", 0.05));
            master_scale = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fMasterScale", 1));
            toggle_key_offset_x = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fToggleKeyOffsetX", 115));
            toggle_key_offset_y = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fToggleKeyOffsetY", 115));
            current_items_offset_x = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fCurrentItemsOffsetX", -15));
            current_items_offset_y = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fCurrentItemsOffsetY", 215));
            slot_count_text_offset = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fSlotCountTextOffset", 20));
            slot_item_name_offset_horizontal_x =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fSlotItemNameOffsetHorizontalX", -15));
            slot_item_name_offset_horizontal_y =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fSlotItemNameOffsetHorizontalY", 100));
            slot_item_name_offset_vertical_x =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fSlotItemNameOffsetVerticalX", 10));
            slot_item_name_offset_vertical_y =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fSlotItemNameOffsetVerticalY", 65));
            arrow_slot_offset_x = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fArrowSlotOffsetX", -125));
            arrow_slot_offset_y = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fArrowSlotOffsetY", 125));
            arrow_slot_count_text_offset =
                static_cast<float>(mcm.GetDoubleValue("HudSetting", "fArrowSlotCountTextOffset", 12));
            current_shout_offset_x = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fCurrentShoutOffsetX", -10));
            current_shout_offset_y = static_cast<float>(mcm.GetDoubleValue("HudSetting", "fCurrentShoutOffsetY", -225));

            background_transparency =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uBackgroundTransparency", 150));
            background_icon_transparency =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uBackgroundIconTransparency", 175));
            icon_transparency = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uIconTransparency", 125));
            key_transparency = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uKeyTransparency", 225));
            current_items_transparency =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uCurrentItemsTransparency", 255));
            current_shout_transparency =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uCurrentShoutTransparency", 255));
            slot_count_transparency =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotCountTransparency", 255));
            slot_item_name_transparency =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotItemNameTransparency", 255));
            icon_transparency_blocked =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uIconTransparencyBlocked", 50));
            slot_count_text_font_size =
                static_cast<float>(mcm.GetDoubleValue("GraphicSetting", "fSlotCountTextFontSize", 20));
            current_items_font_size =
                static_cast<float>(mcm.GetDoubleValue("GraphicSetting", "fCurrentItemsFontSize", 20));
            arrow_count_font_size = static_cast<float>(mcm.GetDoubleValue("GraphicSetting", "fArrowCountFontSize", 20));
            current_items_red = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uCurrentItemsRed", 255));
            current_items_green = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uCurrentItemsGreen", 255));
            current_items_blue = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uCurrentItemsBlue", 255));
            slot_count_red = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotCountRed", 255));
            slot_count_green = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotCountGreen", 255));
            slot_count_blue = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotCountBlue", 255));
            slot_item_red = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotItemRed", 255));
            slot_item_green = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotItemGreen", 255));
            slot_item_blue = static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotItemBlue", 255));

            slot_button_feedback =
                static_cast<uint32_t>(mcm.GetLongValue("GraphicSetting", "uSlotButtonFeedback", 150));
            draw_current_items_text = mcm.GetBoolValue("GraphicSetting", "bDrawCurrentItemsText", true);
            draw_item_name_text = mcm.GetBoolValue("GraphicSetting", "bDrawItemNameText", true);
            draw_toggle_button = mcm.GetBoolValue("GraphicSetting", "bDrawToggleButton", true);
            draw_current_shout_text = mcm.GetBoolValue("GraphicSetting", "bDrawCurrentShoutText", false);
            current_shout_font_size =
                static_cast<float>(mcm.GetDoubleValue("GraphicSetting", "fCurrentShoutFontSize", 20));
            item_name_font_size = static_cast<float>(mcm.GetDoubleValue("GraphicSetting", "fItemNameTextFontSize", 20));
            draw_page_id = mcm.GetBoolValue("GraphicSetting", "bDrawPageId", false);

            alpha_slot_animation =
                static_cast<uint32_t>(mcm.GetLongValue("AnimationSetting", "uAlphaSlotAnimation", 51));
            duration_slot_animation =
                static_cast<float>(mcm.GetDoubleValue("AnimationSetting", "fDurationSlotAnimation", 0.1));

            action_check = mcm.GetBoolValue("MiscSetting", "bActionCheck", false);
            empty_hand_setting = mcm.GetBoolValue("MiscSetting", "bEmptyHandSetting", false);
            hide_outside_combat = mcm.GetBoolValue("MiscSetting", "bHideOutsideCombat", false);
            fade_timer_outside_combat =
                static_cast<float>(mcm.GetDoubleValue("MiscSetting", "fFadeTimerOutsideCombat", 5));
            disable_input_quick_loot = mcm.GetBoolValue("MiscSetting", "bDisableInputQuickLoot", false);
            elder_demon_souls = mcm.GetBoolValue("MiscSetting", "bEldenDemonSouls", false);
            max_page_count = static_cast<uint32_t>(mcm.GetLongValue("MiscSetting", "uMaxPageCount", 4));
            max_ammunition_type = static_cast<uint32_t>(mcm.GetLongValue("MiscSetting", "uMaxAmmunitionType", 3));
            check_duplicate_items = mcm.GetBoolValue("MiscSetting", "bCheckDuplicateItems", true);
            un_equip_ammo = mcm.GetBoolValue("MiscSetting", "bUnEquipAmmo", false);
            only_favorite_ammo = mcm.GetBoolValue("MiscSetting", "bOnlyFavoriteAmmo", false);
            prevent_consumption_of_last_dynamic_potion =
                mcm.GetBoolValue("MiscSetting", "bPreventConsumptionOfLastDynamicPotion", true);
            group_potions = mcm.GetBoolValue("MiscSetting", "bGroupPotions", false);
            potion_min_perfect = static_cast<float>(mcm.GetDoubleValue("MiscSetting", "fPotionMinPerfect", 0.7));
            potion_max_perfect = static_cast<float>(mcm.GetDoubleValue("MiscSetting", "fPotionMaxPerfect", 1.2));
            disable_re_equip_of_actives = mcm.GetBoolValue("MiscSetting", "bDisableReEquipOfActives", false);
            sort_arrow_by_quantity = mcm.GetBoolValue("MiscSetting", "bSortArrowByQuantity", false);
            overwrite_poison_dose = mcm.GetBoolValue("MiscSetting", "bPoisonDoseOverwrite", false);
            apply_poison_dose = static_cast<uint32_t>(mcm.GetLongValue("MiscSetting", "uApplyPoisonDose", 5));
            try_dual_cast_top_spell = mcm.GetBoolValue("MiscSetting", "bTryDualCastTopSpell", false);

            auto_cleanup = mcm.GetBoolValue("CleanupSetting", "bAutoCleanup", false);
            clean_armor = mcm.GetBoolValue("CleanupSetting", "bCleanArmor", true);
            clean_weapon = mcm.GetBoolValue("CleanupSetting", "bCleanWeapon", true);
            clean_spell = mcm.GetBoolValue("CleanupSetting", "bCleanSpell", true);
            clean_alchemy_item = mcm.GetBoolValue("CleanupSetting", "bCleanAlchemyItem", false);
            clean_shout = mcm.GetBoolValue("CleanupSetting", "bCleanShout", true);
            clean_light = mcm.GetBoolValue("CleanupSetting", "bCleanLight", false);
            clean_scroll = mcm.GetBoolValue("CleanupSetting", "bCleanScroll", false);
        };

        read_mcm(mcm_default_setting);
        read_mcm(mcm_config_setting);

        logger::info("finished reading mcm ini files. return.");
    }

    uint32_t mcm_setting::get_top_action_key() { return top_action_key; }
    uint32_t mcm_setting::get_right_action_key() { return right_action_key; }
    uint32_t mcm_setting::get_bottom_action_key() { return bottom_action_key; }
    uint32_t mcm_setting::get_left_action_key() { return left_action_key; }
    uint32_t mcm_setting::get_toggle_key() { return toggle_key; }
    uint32_t mcm_setting::get_show_hide_key() { return show_hide_key; }
    bool mcm_setting::get_key_press_to_enter_edit() { return key_press_to_enter_edit; }
    uint32_t mcm_setting::get_edit_key() { return edit_key; }
    uint32_t mcm_setting::get_left_or_overwrite_edit_key() { return left_or_overwrite_edit_key; }
    uint32_t mcm_setting::get_remove_key() { return remove_key; }
    bool mcm_setting::get_bottom_execute_key_combo_only() { return bottom_execute_key_combo_only; }
    uint32_t mcm_setting::get_controller_set() { return controller_set; }

    float mcm_setting::get_hud_image_scale_width() { return hud_image_scale_width * master_scale; }
    float mcm_setting::get_hud_image_scale_height() { return hud_image_scale_height * master_scale; }
    float mcm_setting::get_hud_image_position_width() { return hud_image_position_width; }
    float mcm_setting::get_hud_image_position_height() { return hud_image_position_height; }
    float mcm_setting::get_hud_slot_position_offset_x() { return hud_slot_position_offset_x * master_scale; }
    float mcm_setting::get_hud_slot_position_offset_y() { return hud_slot_position_offset_y * master_scale; }
    float mcm_setting::get_hud_key_position_offset() { return hud_key_position_offset * master_scale; }
    float mcm_setting::get_icon_scale_width() { return icon_scale_width * master_scale; }
    float mcm_setting::get_icon_scale_height() { return icon_scale_height * master_scale; }
    float mcm_setting::get_key_icon_scale_width() { return key_icon_scale_width * master_scale; }
    float mcm_setting::get_key_icon_scale_height() { return key_icon_scale_height * master_scale; }
    float mcm_setting::get_hud_arrow_image_scale_width() { return hud_arrow_image_scale_width * master_scale; }
    float mcm_setting::get_hud_arrow_image_scale_height() { return hud_arrow_image_scale_height * master_scale; }
    float mcm_setting::get_arrow_icon_scale_width() { return arrow_icon_scale_width * master_scale; }
    float mcm_setting::get_arrow_icon_scale_height() { return arrow_icon_scale_height * master_scale; }
    [[maybe_unused]] float mcm_setting::get_master_scale() { return master_scale; }
    float mcm_setting::get_slot_count_text_offset() { return slot_count_text_offset * master_scale; }
    float mcm_setting::get_toggle_key_offset_x() { return toggle_key_offset_x * master_scale; }
    float mcm_setting::get_toggle_key_offset_y() { return toggle_key_offset_y * master_scale; }
    float mcm_setting::get_current_items_offset_x() { return current_items_offset_x * master_scale; }
    float mcm_setting::get_current_items_offset_y() { return current_items_offset_y * master_scale; }
    float mcm_setting::get_slot_item_name_offset_horizontal_x() {
        return slot_item_name_offset_horizontal_x * master_scale;
    }
    float mcm_setting::get_slot_item_name_offset_horizontal_y() {
        return slot_item_name_offset_horizontal_y * master_scale;
    }
    float mcm_setting::get_slot_item_name_offset_vertical_x() {
        return slot_item_name_offset_vertical_x * master_scale;
    }
    float mcm_setting::get_slot_item_name_offset_vertical_y() {
        return slot_item_name_offset_vertical_y * master_scale;
    }
    float mcm_setting::get_arrow_slot_offset_x() { return arrow_slot_offset_x * master_scale; }
    float mcm_setting::get_arrow_slot_offset_y() { return arrow_slot_offset_y * master_scale; }
    float mcm_setting::get_arrow_slot_count_text_offset() { return arrow_slot_count_text_offset * master_scale; }
    float mcm_setting::get_current_shout_offset_x() { return current_shout_offset_x * master_scale; }
    float mcm_setting::get_current_shout_offset_y() { return current_shout_offset_y * master_scale; }

    uint32_t mcm_setting::get_background_transparency() { return background_transparency; }
    uint32_t mcm_setting::get_background_icon_transparency() { return background_icon_transparency; }
    uint32_t mcm_setting::get_icon_transparency() { return icon_transparency; }
    uint32_t mcm_setting::get_key_transparency() { return key_transparency; }
    uint32_t mcm_setting::get_current_items_transparency() { return current_items_transparency; }
    uint32_t mcm_setting::get_current_shout_transparency() { return current_shout_transparency; }
    uint32_t mcm_setting::get_slot_count_transparency() { return slot_count_transparency; }
    uint32_t mcm_setting::get_slot_item_name_transparency() { return slot_item_name_transparency; }
    uint32_t mcm_setting::get_icon_transparency_blocked() { return icon_transparency_blocked; }
    float mcm_setting::get_slot_count_text_font_size() { return slot_count_text_font_size * master_scale; }
    float mcm_setting::get_current_items_font_size() { return current_items_font_size * master_scale; }
    float mcm_setting::get_arrow_count_font_size() { return arrow_count_font_size * master_scale; }
    uint32_t mcm_setting::get_current_items_red() { return current_items_red; }
    uint32_t mcm_setting::get_current_items_green() { return current_items_green; }
    uint32_t mcm_setting::get_current_items_blue() { return current_items_blue; }
    uint32_t mcm_setting::get_slot_count_red() { return slot_count_red; }
    uint32_t mcm_setting::get_slot_count_green() { return slot_count_green; }
    uint32_t mcm_setting::get_slot_count_blue() { return slot_count_blue; }
    uint32_t mcm_setting::get_slot_item_red() { return slot_item_red; }
    uint32_t mcm_setting::get_slot_item_green() { return slot_item_green; }
    uint32_t mcm_setting::get_slot_item_blue() { return slot_item_blue; }
    bool mcm_setting::get_draw_current_items_text() { return draw_current_items_text; }
    uint32_t mcm_setting::get_slot_button_feedback() { return slot_button_feedback; }
    bool mcm_setting::get_draw_item_name_text() { return draw_item_name_text; }
    bool mcm_setting::get_draw_toggle_button() { return draw_toggle_button; }
    bool mcm_setting::get_draw_current_shout_text() { return draw_current_shout_text; }
    float mcm_setting::get_current_shout_font_size() { return current_shout_font_size; }
    float mcm_setting::get_item_name_font_size() { return item_name_font_size; }
    bool mcm_setting::get_draw_page_id() { return draw_page_id; }

    uint32_t mcm_setting::get_alpha_slot_animation() { return alpha_slot_animation; }
    float mcm_setting::get_duration_slot_animation() { return duration_slot_animation; }

    bool mcm_setting::get_action_check() { return action_check; }
    bool mcm_setting::get_empty_hand_setting() { return empty_hand_setting; }
    bool mcm_setting::get_hide_outside_combat() { return hide_outside_combat; }
    float mcm_setting::get_fade_timer_outside_combat() { return fade_timer_outside_combat; }
    bool mcm_setting::get_disable_input_quick_loot() { return disable_input_quick_loot; }
    bool mcm_setting::get_elden_demon_souls() { return elder_demon_souls; }
    uint32_t mcm_setting::get_max_page_count() { return max_page_count; }
    uint32_t mcm_setting::get_max_ammunition_type() { return max_ammunition_type; }
    bool mcm_setting::get_check_duplicate_items() { return check_duplicate_items; }
    bool mcm_setting::get_un_equip_ammo() { return un_equip_ammo; }
    bool mcm_setting::get_only_favorite_ammo() { return only_favorite_ammo; }
    bool mcm_setting::get_prevent_consumption_of_last_dynamic_potion() {
        return prevent_consumption_of_last_dynamic_potion;
    }
    bool mcm_setting::get_group_potions() { return group_potions; }
    float mcm_setting::get_potion_min_perfect() { return potion_min_perfect; }
    float mcm_setting::get_potion_max_perfect() { return potion_max_perfect; }
    bool mcm_setting::get_disable_re_equip_of_actives() { return disable_re_equip_of_actives; }
    bool mcm_setting::get_sort_arrow_by_quantity() { return sort_arrow_by_quantity; }
    bool mcm_setting::get_overwrite_poison_dose() { return overwrite_poison_dose; }
    uint32_t mcm_setting::get_apply_poison_dose() { return apply_poison_dose; }
    bool mcm_setting::get_try_dual_cast_top_spell() { return try_dual_cast_top_spell; }

    bool mcm_setting::get_auto_cleanup() { return auto_cleanup; }
    bool mcm_setting::get_clean_armor() { return clean_armor; }
    bool mcm_setting::get_clean_weapon() { return clean_weapon; }
    bool mcm_setting::get_clean_spell() { return clean_spell; }
    bool mcm_setting::get_clean_alchemy_item() { return clean_alchemy_item; }
    bool mcm_setting::get_clean_shout() { return clean_shout; }
    bool mcm_setting::get_clean_light() { return clean_light; }
    bool mcm_setting::get_clean_scroll() { return clean_scroll; }
}
