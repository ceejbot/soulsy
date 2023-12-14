#pragma once

namespace log
{
	void error(const char* fmtstr, va_list args);
	void warn(const char* fmtstr, va_list args);
	void info(const char* fmtstr, va_list args);
	void debug(const char* fmtstr, va_list args);
	void trace(const char* fmtstr, va_list args);
}
