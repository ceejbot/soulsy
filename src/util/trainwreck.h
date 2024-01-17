#include <Windows.h>
#include <stddef.h>
#include <stdint.h>

#pragma comment(lib, "Kernel32")

struct trainwreck_string {
  const uint8_t *data;
  size_t len;
};

struct trainwreck_decoder_callback_args {
  uint32_t version;
  void *user_context;
  void *log_context;
  const void *object;
};

struct trainwreck_section_callback_args {
  uint32_t version;
  void *user_context;
  void *log_context;
  const EXCEPTION_POINTERS *exception;
};

enum trainwreck_error {
  TRAINWRECK_ERROR_OK = 0,

  TRAINWRECK_ERROR_REGISTER_SECTION_WHERE_INVALID,
  TRAINWRECK_ERROR_REGISTER_SECTION_HOW_INVALID,
  TRAINWRECK_ERROR_REGISTER_SECTION_HEADER_INVALID,
  TRAINWRECK_ERROR_REGISTER_SECTION_CALLBACK_INVALID,

  TRAINWRECK_ERROR_LOG_CONTEXT_INVALID,
  TRAINWRECK_ERROR_LOG_LINE_INVALID,

  TRAINWRECK_ERROR_REGISTER_DECODER_DECORATED_NAME_INVALID,
  TRAINWRECK_ERROR_REGISTER_DECODER_CALLBACK_INVALID,
};

enum trainwreck_section_where {
  TRAINWRECK_SECTION_SYSTEM_SPECS = 0,
  TRAINWRECK_SECTION_CALL_STACK,
  TRAINWRECK_SECTION_REGISTERS,
  TRAINWRECK_SECTION_STACK,
  TRAINWRECK_SECTION_MODULES,
  TRAINWRECK_SECTION_XSE_PLUGINS,
};

enum trainwreck_section_how {
  TRAINWRECK_SECTION_BEFORE = 0,
  TRAINWRECK_SECTION_AFTER,
};

typedef enum trainwreck_error(__cdecl *trainwreck_log_indent_t)(void *context);
typedef enum trainwreck_error(__cdecl *trainwreck_log_dedent_t)(void *context);
typedef enum trainwreck_error(__cdecl *trainwreck_log_write_line_t)(
    void *context, const struct trainwreck_string *line);

typedef void(__cdecl *trainwreck_register_section_callback_t)(
    const struct trainwreck_section_callback_args *);
typedef enum trainwreck_error(__cdecl *trainwreck_register_section_t)(
    uint32_t register_where, uint32_t register_how,
    const struct trainwreck_string *header,
    trainwreck_register_section_callback_t callback, void *user_context);

typedef void(__cdecl *trainwreck_register_decoder_callback_t)(
    const struct trainwreck_decoder_callback_args *);
typedef enum trainwreck_error(__cdecl *trainwreck_register_decoder_t)(
    const struct trainwreck_string *decorated_name,
    trainwreck_register_decoder_callback_t callback, void *user_context);

#if __cplusplus > 201606L

#include <functional>
#include <optional>
#include <string_view>

namespace trainwreck {
auto log_indent(void *context) -> std::optional<::trainwreck_error> {
  const auto handle = ::GetModuleHandleW(L"trainwreck.dll");
  if (handle != NULL) {
    const auto proc = ::GetProcAddress(handle, "trainwreck_log_indent");
    if (proc != NULL) {
      const auto func = reinterpret_cast<::trainwreck_log_indent_t>(proc);
      return func(context);
    }
  }

  return std::nullopt;
}

auto log_dedent(void *context) -> std::optional<::trainwreck_error> {
  const auto handle = ::GetModuleHandleW(L"trainwreck.dll");
  if (handle != NULL) {
    const auto proc = ::GetProcAddress(handle, "trainwreck_log_dedent");
    if (proc != NULL) {
      const auto func = reinterpret_cast<::trainwreck_log_dedent_t>(proc);
      return func(context);
    }
  }

  return std::nullopt;
}

auto log_write_line(void *context, std::string_view line)
    -> std::optional<::trainwreck_error> {
  const auto handle = ::GetModuleHandleW(L"trainwreck.dll");
  if (handle != NULL) {
    const auto proc = ::GetProcAddress(handle, "trainwreck_log_write_line");
    if (proc != NULL) {
      const auto func = reinterpret_cast<::trainwreck_log_write_line_t>(proc);
      const auto string = ::trainwreck_string{
          .data = reinterpret_cast<const uint8_t *>(line.data()),
          .len = line.length(),
      };
      return func(context, &string);
    }
  }

  return std::nullopt;
}

auto register_section(::trainwreck_section_where register_where,
                      ::trainwreck_section_how register_how,
                      std::string_view header,
                      ::trainwreck_register_section_callback_t callback,
                      void *user_context = nullptr)
    -> std::optional<::trainwreck_error> {
  const auto handle = ::GetModuleHandleW(L"trainwreck.dll");
  if (handle != NULL) {
    const auto proc = ::GetProcAddress(handle, "trainwreck_register_section");
    if (proc != NULL) {
      const auto func = reinterpret_cast<::trainwreck_register_section_t>(proc);
      const auto string = ::trainwreck_string{
          .data = reinterpret_cast<const uint8_t *>(header.data()),
          .len = header.length(),
      };
      return func(register_where, register_how, &string, callback,
                  user_context);
    }
  }

  return std::nullopt;
}

auto register_decoder(std::string_view decorated_name,
                      ::trainwreck_register_decoder_callback_t callback,
                      void *user_context = nullptr)
    -> std::optional<::trainwreck_error> {
  const auto handle = ::GetModuleHandleW(L"trainwreck.dll");
  if (handle != NULL) {
    const auto proc = ::GetProcAddress(handle, "trainwreck_register_decoder");
    if (proc != NULL) {
      const auto func = reinterpret_cast<::trainwreck_register_decoder_t>(proc);
      const auto string = ::trainwreck_string{
          .data = reinterpret_cast<const uint8_t *>(decorated_name.data()),
          .len = decorated_name.length(),
      };
      return func(&string, callback, user_context);
    }
  }

  return std::nullopt;
}

class Log {
public:
  Log(void *log_context) { this->m_log_context = log_context; }

  auto indent() -> std::optional<::trainwreck_error> {
    return trainwreck::log_indent(this->m_log_context);
  }

  auto dedent() -> std::optional<::trainwreck_error> {
    return trainwreck::log_dedent(this->m_log_context);
  }

  auto with_indent(std::function<void(trainwreck::Log &)> callback)
      -> std::optional<::trainwreck_error> {
    const auto result = this->indent();
    if (result.has_value() && *result == ::TRAINWRECK_ERROR_OK) {
      callback(*this);
      return this->dedent();
    } else {
      return result;
    }
  }

  auto write_line(std::string_view line) -> std::optional<::trainwreck_error> {
    return trainwreck::log_write_line(this->m_log_context, line);
  }

private:
  void *m_log_context = nullptr;
};
} // namespace trainwreck

#endif
