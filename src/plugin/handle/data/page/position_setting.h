#pragma once
#include "position_draw_setting.h"
#include "include/custom_setting.h"
#include "include/enums.h"
#include "include/image_path.h"
#include "handle/data/page/slot_setting.h"

namespace handle {
    using position_type = enums::position_type;

    class position_setting {
    public:
        uint32_t page = 0;
        position_type position = position_type::total;
        std::vector<slot_setting*> slot_settings;
        ui::icon_image_type icon_type = ui::icon_image_type::icon_default;
        uint32_t button_press_modify = ui::draw_full;
        uint32_t key = 0;
        position_draw_setting* draw_setting = nullptr;
        float item_name_font_size = 0.f;
        float count_font_size = 0.f;
        bool item_name = false;
        bool highlight_slot = false;
    };
}
