cmake_minimum_required(VERSION 3.22)

if(PROJECT_SOURCE_DIR STREQUAL PROJECT_BINARY_DIR)
    message(
        FATAL_ERROR
        "Trying to run cmake inside the source directory. Generate a build dir first!"
    )
endif()

set(NAME "SoulsyHUD")
set(VERSION 0.11.2.0)

project(
    ${NAME}
    VERSION ${VERSION}
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION_DEBUG OFF)

macro(set_from_environment VARIABLE)
    if(NOT DEFINED ${VARIABLE} AND DEFINED ENV{${VARIABLE}})
        set(${VARIABLE} $ENV{${VARIABLE}})
    endif()
endmacro()

# We are managing our dependencies with vcpkg and need this env var set.
set_from_environment(VCPKG_ROOT)

if(DEFINED VCPKG_ROOT)
    set(CMAKE_TOOLCHAIN_FILE "${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake" CACHE STRING "")
    set(VCPKG_TARGET_TRIPLET "x64-windows-static" CACHE STRING "")
else()
    message(
        FATAL_ERROR
        "Variable VCPKG_ROOT is not set. Please set it in either the cmake file or your env."
    )
endif()

set(Boost_USE_STATIC_RUNTIME OFF CACHE BOOL "")
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>" CACHE STRING "")

option(ENABLE_SKYRIM_SE "Enable support for Skyrim SE in the dynamic runtime feature." ON)
option(ENABLE_SKYRIM_AE "Enable support for Skyrim AE in the dynamic runtime feature." ON)
option(ENABLE_SKYRIM_VR "Enable support for Skyrim VR in the dynamic runtime feature." OFF)
set(BUILD_TESTS OFF)

# We're building and linking with a Rust library.
set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

set(SOULSY_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(SOULSY_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/soulsy/src/lib.rs.cc)
set(SOULSY_LIB ${CARGO_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}soulsy${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
    OUTPUT ${SOULSY_BRIDGE_CPP} ${SOULSY_LIB}
    COMMAND cargo build --manifest-path ${CARGO_MANIFEST} --release
    DEPENDS ${SOULSY_SOURCE_FILE}
    USES_TERMINAL
    COMMENT "Running cargo..."
)

# Set up our version header.
configure_file(
	${CMAKE_CURRENT_SOURCE_DIR}/cmake/Version.h.in
	${CMAKE_CURRENT_BINARY_DIR}/include/Version.h
	@ONLY
)

configure_file(
	${CMAKE_CURRENT_SOURCE_DIR}/cmake/version.rc.in
	${CMAKE_CURRENT_BINARY_DIR}/version.rc
	@ONLY
)

if(MSVC)
    add_compile_definitions(
        _UNICODE
    )

    if(NOT ${CMAKE_GENERATOR} STREQUAL "Ninja")
        add_compile_options(
            /MP # Build with Multiple Processes
        )
    endif()
endif()

set(Boost_USE_STATIC_LIBS ON)

add_compile_definitions(SKYRIM)
set(CommonLibPath "extern/CommonLibSSE-NG")
set(CommonLibName "CommonLibSSE")
set(GameVersion "Skyrim")

# Find our vcpkg dependencies.
# find_package(CommonLibSSE CONFIG REQUIRED)
find_package(spdlog REQUIRED CONFIG)
find_package(imgui REQUIRED)

include(cmake/sourcelist.cmake)

source_group(
    TREE
    ${CMAKE_CURRENT_SOURCE_DIR}
    FILES
    ${sources}
)

source_group(
    TREE
    ${CMAKE_CURRENT_BINARY_DIR}
    FILES
    ${CMAKE_CURRENT_BINARY_DIR}/include/Version.h
)

# We're building a DLL.
add_library(
    ${PROJECT_NAME}
    SHARED
    ${sources}
    ${SOULSY_BRIDGE_CPP}
    ${CMAKE_CURRENT_BINARY_DIR}/include/Version.h
    ${CMAKE_CURRENT_BINARY_DIR}/version.rc
)

set_target_properties(
    ${PROJECT_NAME}
    PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

target_compile_features(
    ${PROJECT_NAME}
    PRIVATE
    cxx_std_23
)

# Set up our compiler options. Windows is weird.
if(MSVC)
    add_compile_definitions(
        _UNICODE
    )

    if(NOT ${CMAKE_GENERATOR} STREQUAL "Ninja")
        add_compile_options(
            /MP # Build with Multiple Processes
        )
    endif()

    target_compile_options(
        ${PROJECT_NAME}
        PRIVATE
        /sdl # Enable Additional Security Checks
        /utf-8 # Set Source and Executable character sets to UTF-8
        /Zi # Debug Information Format

        /permissive- # Standards conformance

        /Zc:alignedNew # C++17 over-aligned allocation
        /Zc:auto # Deduce Variable Type
        /Zc:char8_t
        /Zc:__cplusplus # Enable updated __cplusplus macro
        /Zc:externC
        /Zc:externConstexpr # Enable extern constexpr variables
        /Zc:forScope # Force Conformance in for Loop Scope
        /Zc:hiddenFriend
        /Zc:implicitNoexcept # Implicit Exception Specifiers
        /Zc:lambda
        /Zc:noexceptTypes # C++17 noexcept rules
        /Zc:preprocessor # Enable preprocessor conformance mode
        /Zc:referenceBinding # Enforce reference binding rules
        /Zc:rvalueCast # Enforce type conversion rules
        /Zc:sizedDealloc # Enable Global Sized Deallocation Functions
        /Zc:strictStrings # Disable string literal type conversion
        /Zc:ternary # Enforce conditional operator rules
        /Zc:threadSafeInit # Thread-safe Local Static Initialization
        /Zc:tlsGuards
        /Zc:trigraphs # Trigraphs Substitution
        /Zc:wchar_t # wchar_t Is Native Type

        /external:anglebrackets
        /external:W0

        /W4 # Warning level
        /WX # Warning level (warnings are errors)

        "$<$<CONFIG:DEBUG>:>"
        "$<$<CONFIG:RELEASE>:/Zc:inline;/JMC-;/Ob3>"
    )

    target_link_options(
        ${PROJECT_NAME}
        PRIVATE
        /WX # Treat Linker Warnings as Errors

        "$<$<CONFIG:DEBUG>:/INCREMENTAL;/OPT:NOREF;/OPT:NOICF>"
        "$<$<CONFIG:RELEASE>:/INCREMENTAL:NO;/OPT:REF;/OPT:ICF;/DEBUG:FULL>"
    )

    SET_PROPERTY(
        TARGET ${PROJECT_NAME}
        PROPERTY
        VS_USER_PROPS ${CMAKE_CURRENT_SOURCE_DIR}/cmake/no_precomp_std.props
    )
endif()

add_subdirectory(${CommonLibPath} ${CommonLibName} EXCLUDE_FROM_ALL)

target_include_directories(
    ${PROJECT_NAME}
    PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}/src/config
    ${CMAKE_CURRENT_SOURCE_DIR}/src/game
    ${CMAKE_CURRENT_SOURCE_DIR}/src/migrate_me
    ${CMAKE_CURRENT_SOURCE_DIR}/src/plugin
    ${CMAKE_CURRENT_SOURCE_DIR}/src/renderer
    ${CMAKE_CURRENT_SOURCE_DIR}/src/util
    ${CARGO_TARGET_DIR}/cxxbridge/soulsy/src
    ${CARGO_TARGET_DIR}/cxxbridge
)

# The last few of these are surprising, but are pulled in by some of the rust crates
# included in the Rust library.
target_link_libraries(
    ${PROJECT_NAME}
    PRIVATE
    ${SOULSY_LIB}
    CommonLibSSE::CommonLibSSE
    spdlog::spdlog
    imgui::imgui
    wsock32
    ws2_32
    bcrypt
    ntdll
    userenv
)

# Windows-only configuration (from rust bridge example;)
# if(WIN32)
#     target_link_libraries(${PROJECT_NAME} userenv ws2_32 bcrypt)
#     set_target_properties(
#         ${PROJECT_NAME}
#         PROPERTIES
#         MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
#         RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
#         RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
#     )
# endif()

# Precompiled headers, because C++ compilers are slow. Everything is slow.
target_precompile_headers(
    ${PROJECT_NAME}
    PRIVATE
    src/PCH.h
)
