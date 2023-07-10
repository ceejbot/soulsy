#include "key_position_handle.h"
#include "include/user_settings.h"

namespace handle {
    using mcm = config::mcm_setting;
    using position_type = position_setting::position_type;

    key_position_handle* key_position_handle::get_singleton() {
        static key_position_handle singleton;
        return std::addressof(singleton);
    }

    void key_position_handle::init_key_position_map() {
        logger::trace("init key position map ..."sv);
        if (!this->data_) {
            this->data_ = new key_position_handle_data();
        }

        key_position_handle_data* data = this->data_;

        data->key_position_map[mcm::get_top_action_key()] = position_type::top;
        data->key_position_map[mcm::get_right_action_key()] = position_type::right;
        data->key_position_map[mcm::get_bottom_action_key()] = position_type::bottom;
        data->key_position_map[mcm::get_left_action_key()] = position_type::left;

        data->position_key_map[position_type::top] = mcm::get_top_action_key();
        data->position_key_map[position_type::right] = mcm::get_right_action_key();
        data->position_key_map[position_type::bottom] = mcm::get_bottom_action_key();
        data->position_key_map[position_type::left] = mcm::get_left_action_key();


        logger::trace("done with init of position key map."sv);
    }

    void key_position_handle::set_position_lock(const position_type a_position, const uint32_t a_locked) {
        if (!this->data_) {
            this->data_ = new key_position_handle_data();
        }
        key_position_handle_data* data = this->data_;
        logger::trace("init lock for position {}, lock {}"sv, static_cast<uint32_t>(a_position), a_locked);
        data->position_lock_map[a_position] = a_locked;
    }

    position_type key_position_handle::get_position_for_key(const uint32_t a_key) const {
        if (const key_position_handle_data* data = this->data_;
            data && !data->key_position_map.empty() && data->key_position_map.contains(a_key)) {
            const auto pos = data->key_position_map.at(a_key);
            logger::trace("got position {} for key {}"sv, static_cast<uint32_t>(pos), a_key);
            return pos;
        }
        return position_type::total;
    }

    uint32_t key_position_handle::get_key_for_position(const position_type a_position) const {
        if (const key_position_handle_data* data = this->data_;
            data && !data->position_key_map.empty() && data->position_key_map.contains(a_position)) {
            const auto key = data->position_key_map.at(a_position);
            logger::trace("got key {} for position {}"sv, key, static_cast<uint32_t>(a_position));
            return key;
        }
        return 0;
    }

    bool key_position_handle::is_position_locked(const position_type a_position) const {
        if (const key_position_handle_data* data = this->data_;
            data && !data->position_lock_map.empty() && data->position_lock_map.contains(a_position)) {
            return data->position_lock_map.at(a_position) == 1;
        }
        return false;
    }
}
