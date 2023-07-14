#pragma once

namespace file_setting
{
	// Notes from ceej:
	// This is the old "file setting" aka a canned top-level config choice set for major
	// mod behaviors. The only thing remaining from this that we care about is the debug
	// setting, which affects log levels.

	// Should be entirely replaced with the Rust-side `layout`

	void load_setting();

	bool get_is_debug();
	bool get_draw_key_background();

	bool get_font_load();
	std::string get_font_file_name();
	float get_font_size();
	bool get_font_chinese_full();
	bool get_font_chinese_simplified_common();
	bool get_font_cyrillic();
	bool get_font_japanese();
	bool get_font_korean();
	bool get_font_thai();
	bool get_font_vietnamese();

	std::string get_config_default();
	std::string get_config_elden();

	void set_config_default(const std::string& a_config);
	void set_config_elden(const std::string& a_config);

	bool get_show_ui();
	void set_show_ui(bool a_show);

	void save_setting();
}
