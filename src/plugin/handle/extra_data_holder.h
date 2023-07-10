#pragma once

namespace handle {
    class extra_data_holder {
    public:
        static extra_data_holder* get_singleton();
        void init_extra_data(const RE::TESForm* a_form, const std::vector<RE::ExtraDataList*>& a_extra_data_list);
        void overwrite_extra_data_for_form(const RE::TESForm* a_form,
            const std::vector<RE::ExtraDataList*>& a_extra_data_list);
        void reset_data();
        bool is_form_set(const RE::TESForm* a_form);
        [[nodiscard]] std::vector<RE::ExtraDataList*> get_extra_list_for_form(const RE::TESForm* a_form);

        extra_data_holder(const extra_data_holder&) = delete;
        extra_data_holder(extra_data_holder&&) = delete;

        extra_data_holder& operator=(const extra_data_holder&) const = delete;
        extra_data_holder& operator=(extra_data_holder&&) const = delete;

    private:
        extra_data_holder() : data_(nullptr) {}
        ~extra_data_holder() = default;

        struct extra_data_holder_data {
            std::map<const RE::TESForm*, std::vector<RE::ExtraDataList*>> form_extra_data_map;
        };

        extra_data_holder_data* data_;
    };
}  // handle
