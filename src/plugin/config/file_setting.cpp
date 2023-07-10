#include "include/file_setting.h"
#include "include/constant.h"

// This name is bad, and this code needs to be re-implemented on the Rust side.
// but for now...

namespace layout {
    static const char* ini_path = R"(.\Data\SKSE\Plugins\LamasTinyHUD.ini)";

    CSimpleIniA ini;

    static bool is_debug;
    static bool draw_key_background;

    static bool font_load;
    static std::string font_file_name;
    static float font_size;
    static bool font_chinese_full;
    static bool font_chinese_simplified_common;
    static bool font_cyrillic;
    static bool font_japanese;
    static bool font_korean;
    static bool font_thai;
    static bool font_vietnamese;

    static std::string default_config;
    static std::string elden_config;

    static bool show_ui;

    void load_setting() {
        logger::info("reading dll ini files");

        ini.Reset();
        ini.SetUnicode();
        ini.LoadFile(ini_path);

        is_debug = ini.GetBoolValue("General", "bIsDebug", false);

        draw_key_background = ini.GetBoolValue("Image", "bDrawKeyBackground", false);

        font_load = ini.GetBoolValue("Font", "bLoad", true);
        font_file_name = ini.GetValue("Font", "sName", "");
        font_size = static_cast<float>(ini.GetDoubleValue("Font", "fSize", 20));
        font_chinese_full = ini.GetBoolValue("Font", "bChineseFull", false);
        font_chinese_simplified_common = ini.GetBoolValue("Font", "bChineseSimplifiedCommon", false);
        font_cyrillic = ini.GetBoolValue("Font", "bCyrillic", false);
        font_japanese = ini.GetBoolValue("Font", "bJapanese", false);
        font_korean = ini.GetBoolValue("Font", "bKorean", false);
        font_thai = ini.GetBoolValue("Font", "bThai", false);
        font_vietnamese = ini.GetBoolValue("Font", "bVietnamese", false);

        default_config = ini.GetValue("Config", "sDefault", (util::ini_default_name + util::ini_ending).c_str());
        elden_config = ini.GetValue("Config", "sElden", (util::ini_elden_name + util::ini_ending).c_str());

        show_ui = ini.GetBoolValue("Interface", "bShowUI", true);

        logger::info("finished reading dll ini files. return.");
    }

    bool get_is_debug() { return is_debug; }
    bool get_draw_key_background() { return draw_key_background; }

    bool get_font_load() { return font_load; }
    std::string get_font_file_name() { return font_file_name; }
    float get_font_size() { return font_size; }
    bool get_font_chinese_full() { return font_chinese_full; }
    bool get_font_chinese_simplified_common() { return font_chinese_simplified_common; }
    bool get_font_cyrillic() { return font_cyrillic; }
    bool get_font_japanese() { return font_japanese; }
    bool get_font_korean() { return font_korean; }
    bool get_font_thai() { return font_thai; }
    bool get_font_vietnamese() { return font_vietnamese; }
    std::string get_config_default() { return default_config; }
    std::string get_config_elden() { return elden_config; }

    void save_setting() {
        (void)ini.SaveFile(ini_path);
        load_setting();
    }

    void set_config_default(const std::string& a_config) {
        ini.SetValue("Config", "sDefault", a_config.c_str());
        save_setting();
    }

    void set_config_elden(const std::string& a_config) {
        ini.SetValue("Config", "sElden", a_config.c_str());
        save_setting();
    }

    bool get_show_ui() { return show_ui; }

    void set_show_ui(bool a_show) {
        ini.SetBoolValue("Interface", "bShowUI", a_show);
        save_setting();
    }
}
