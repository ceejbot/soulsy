#include "logs.h"

#include "fmt/format.h"
#include "lib.rs.h"

namespace log
{
	void error(const char* fmtstr, va_list args)
	{
		std::string msg = fmt::format(fmtstr, args);
		log_error(&msg);
	}

	void warn(const char* fmtstr, va_list args)
	{
		std::string msg = fmt::format(fmtstr, args);
		log_warn(&msg);
	}

	void info(const char* fmtstr, va_list args)
	{
		std::string msg = fmt::format(fmtstr, args);
		log_info(&msg);
	}

	void debug(const char* fmtstr, va_list args)
	{
		std::string msg = fmt::format(fmtstr, args);
		log_debug(&msg);
	}

	void trace(const char* fmtstr, va_list args)
	{
		std::string msg = fmt::format(fmtstr, args);
		log_trace(&msg);
	}
}
