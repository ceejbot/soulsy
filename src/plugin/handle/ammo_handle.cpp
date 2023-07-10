#include "ammo_handle.h"

namespace handle {
    ammo_handle* ammo_handle::get_singleton() {
        static ammo_handle singleton;
        return std::addressof(singleton);
    }

    void ammo_handle::init_ammo(const std::vector<ammo_data*>& a_ammo) {
        if (!this->data_) {
            this->data_ = new ammo_handle_data();
        }

        ammo_handle_data* data = this->data_;
        data->ammo_list = a_ammo;
        data->current = -1;
    }

    void ammo_handle::clear_ammo() const {
        if (ammo_handle_data* data = this->data_; data && !data->ammo_list.empty()) {
            data->ammo_list.clear();
            data->current = -1;
        }
    }

    void ammo_handle::set_current(const int a_current) const {
        if (ammo_handle_data* data = this->data_; data) {
            data->current = a_current;
        }
    }

    //maybe handle if item count == 0
    RE::TESForm* ammo_handle::get_next_ammo() const {
        if (const ammo_handle_data* data = this->data_; data && !data->ammo_list.empty()) {
            if (data->current < data->ammo_list.size() - 1) {
                set_current(data->current + 1);
                return data->ammo_list.at(data->current)->form;
            }
            set_current(0);
            return data->ammo_list.at(data->current)->form;
        }
        return nullptr;
    }

    ammo_data* ammo_handle::get_current() const {
        if (const ammo_handle_data* data = this->data_; data && !data->ammo_list.empty()) {
            if (data->ammo_list.size() - 1 >= data->current) {
                return data->ammo_list.at(data->current);
            }
        }
        return nullptr;
    }

    std::vector<ammo_data*> ammo_handle::get_all() const {
        if (const ammo_handle_data* data = this->data_; data && !data->ammo_list.empty()) {
            return data->ammo_list;
        }
        return {};
    }
}
