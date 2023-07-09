#pragma once

namespace util
{
	class config_writer_helper
	{
	public:
		std::string section{};
		uint32_t page{};
		uint32_t position{};
		uint32_t type{};
		std::string form{};
		uint32_t action{};
		uint32_t hand{};
		uint32_t type_left{};
		std::string form_left{};
		uint32_t action_left{};
		int actor_value{};
	};
}
