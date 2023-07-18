#pragma once

namespace util
{
	class string_util
	{
	public:
		template <typename T>
		static std::string int_to_hex(T xs)
		{
			std::stringstream stream;
			stream << "0x" << std::hex << std::setw(8) << std::setfill('0') << xs;
			return ltrim_copy(stream.str());
		}

	private:
		static void ltrim(std::string& s)
		{
			s.erase(s.begin(), std::ranges::find_if(s, [](const unsigned char ch) { return !std::isspace(ch); }));
		}

		static std::string ltrim_copy(std::string s)
		{
			ltrim(s);
			return s;
		}
	};
}
