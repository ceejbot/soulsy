#include "name_handle.h"
#include "include/constant.h"
#include "include/helper.h"

namespace handle
{
	using data_helper = helpers::data_helper;

	name_handle* name_handle::get_singleton()
	{
		static name_handle singleton;
		return std::addressof(singleton);
	}

	void name_handle::init_names(const std::vector<data_helper*>& data_helpers)
	{
		if (!this->data_)
		{
			this->data_ = new name_handle_data();
		}

		name_handle_data* data = this->data_;
		std::string help_string;

		data->name = "<Empty>";

		auto check_ammo        = false;
		std::string name_right = "<Empty>";
		if (!data_helpers.empty())
		{
			if (data_helpers[0]->form)
			{
				name_right = data_helpers[0]->form->GetName();
				if (data_helpers[0]->form->IsWeapon())
				{
					//check_ammo
					if (auto* weapon = data_helpers[0]->form->As<RE::TESObjectWEAP>();
						(weapon->IsBow() || weapon->IsCrossbow()) && !weapon->IsBound())
					{
						check_ammo = true;
					}
				}
			}
			data->name = name_right;
		}

		if (check_ammo && data_helpers.size() != 2)
		{
			auto* player = RE::PlayerCharacter::GetSingleton();
			if (auto* ammo = player->GetCurrentAmmo(); ammo && (ammo->IsBolt() || ammo->IsAmmo()))
			{
				data->name = fmt::format("{} {} {}", ammo->GetName(), util::delimiter, name_right);
			}
		}

		if (data_helpers.size() == 2)
		{
			std::string name_left = "<Empty>";
			if (data_helpers[1]->form)
			{
				name_left = data_helpers[1]->form->GetName();
			}
			data->name = fmt::format("{} {} {}", name_left, util::delimiter, name_right);
		}
		logger::trace("name set to {}"sv, data->name);
	}

	void name_handle::init_voice_name(const RE::TESForm* a_form)
	{
		if (!this->data_)
		{
			this->data_ = new name_handle_data();
		}

		name_handle_data* data = this->data_;

		data->voice_name = a_form ? a_form->GetName() : "";
		logger::trace("voice name set to {}"sv, data->voice_name);
	}

	std::string name_handle::get_item_name_string() const
	{
		if (const name_handle_data* data = this->data_; data)
		{
			return data->name;
		}
		return {};
	}

	std::string name_handle::get_voice_name_string() const
	{
		if (const name_handle_data* data = this->data_; data)
		{
			return data->voice_name;
		}
		return {};
	}
}
