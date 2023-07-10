#pragma once
#include "./data/ammo_data.h"

namespace handle {
    class ammo_handle {
    public:
        static ammo_handle* get_singleton();
        void init_ammo(const std::vector<ammo_data*>& a_ammo);
        void clear_ammo() const;
        void set_current(int a_current) const;
        [[nodiscard]] RE::TESForm* get_next_ammo() const;
        [[nodiscard]] ammo_data* get_current() const;
        [[nodiscard]] std::vector<ammo_data*> get_all() const;

        ammo_handle(const ammo_handle&) = delete;
        ammo_handle(ammo_handle&&) = delete;

        ammo_handle& operator=(const ammo_handle&) const = delete;
        ammo_handle& operator=(ammo_handle&&) const = delete;

    private:
        ammo_handle() : data_(nullptr) {}

        ~ammo_handle() = default;

        struct ammo_handle_data {
            std::vector<ammo_data*> ammo_list;
            int current = -1;
        };

        ammo_handle_data* data_;
    };
}
