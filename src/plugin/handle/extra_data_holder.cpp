#include "extra_data_holder.h"

namespace handle
{
	extra_data_holder* extra_data_holder::get_singleton()
	{
		static extra_data_holder singleton;
		return std::addressof(singleton);
	}

	void extra_data_holder::init_extra_data(const RE::TESForm* a_form,
		const std::vector<RE::ExtraDataList*>& a_extra_data_list)
	{
		if (!this->data_)
		{
			this->data_ = new extra_data_holder_data();
		}

		extra_data_holder_data* data = this->data_;

		if (!a_form || a_extra_data_list.empty())
		{
			return;
		}

		data->form_extra_data_map[a_form] = a_extra_data_list;
		logger::trace("set extra data list, form {}, count {}"sv, a_form->GetName(), data->form_extra_data_map.size());
	}

	void extra_data_holder::overwrite_extra_data_for_form(const RE::TESForm* a_form,
		const std::vector<RE::ExtraDataList*>& a_extra_data_list)
	{
		if (!this->data_)
		{
			return;
		}
		extra_data_holder_data* data = this->data_;

		if (data->form_extra_data_map.contains(a_form))
		{
			data->form_extra_data_map[a_form] = a_extra_data_list;
		}
	}

	void extra_data_holder::reset_data()
	{
		if (!this->data_)
		{
			return;
		}
		extra_data_holder_data* data = this->data_;

		if (data->form_extra_data_map.empty())
		{
			return;
		}

		logger::trace("before reset, extra data list {}"sv, data->form_extra_data_map.size());
		data->form_extra_data_map.clear();
		logger::trace("did reset, extra data list {}"sv, data->form_extra_data_map.size());
	}

	bool extra_data_holder::is_form_set(const RE::TESForm* a_form)
	{
		if (!this->data_)
		{
			return false;
		}
		extra_data_holder_data* data = this->data_;

		return data->form_extra_data_map.contains(a_form);
	}

	std::vector<RE::ExtraDataList*> extra_data_holder::get_extra_list_for_form(const RE::TESForm* a_form)
	{
		if (!this->data_)
		{
			return {};
		}
		extra_data_holder_data* data = this->data_;

		if (data->form_extra_data_map.contains(a_form))
		{
			return data->form_extra_data_map.at(a_form);
		}

		return {};
	}
}  // handle
