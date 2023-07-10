#pragma once
#include "include/helper.h"

namespace handle {
    class name_handle {
    public:
        static name_handle* get_singleton();
        void init_names(const std::vector<helpers::data_helper*>& data_helpers);
        void init_voice_name(const RE::TESForm* a_form);
        [[nodiscard]] std::string get_item_name_string() const;
        [[nodiscard]] std::string get_voice_name_string() const;

        name_handle(const name_handle&) = delete;
        name_handle(name_handle&&) = delete;

        name_handle& operator=(const name_handle&) const = delete;
        name_handle& operator=(name_handle&&) const = delete;

    private:
        name_handle() : data_(nullptr) {}
        ~name_handle() = default;

        struct name_handle_data {
            std::string name;
            std::string voice_name;
        };

        name_handle_data* data_;
    };
}
