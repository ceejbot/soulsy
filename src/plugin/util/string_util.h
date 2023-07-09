#pragma once

namespace util
{
	class string_util
	{
	public:
		template <typename T>
		static std::string int_to_hex(T a_i)
		{
			std::stringstream stream;
			stream << std::hex << a_i;

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
