cmake_minimum_required(VERSION 3.1..3.14)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

include(CheckFunctionExists)
include(CheckLibraryExists)
include(CheckIncludeFile)
include(CMakeDependentOption)

check_library_exists(m lround "" HAVE_LIBM)
if(HAVE_LIBM)
  list(APPEND CMAKE_REQUIRED_LIBRARIES m)
endif()

check_function_exists(fseeko HAVE_FSEEKO)
check_function_exists(lround HAVE_LROUND)

check_include_file(inttypes.h HAVE_INTTYPES_H)

find_package(Ogg)
cmake_dependent_option(ENABLE_OGG "Use ogg library" ON "Ogg_FOUND" OFF)
if(ENABLE_OGG)
  set(FLAC__HAS_OGG 1)
endif()

configure_file(config.h.cmake config.h)

add_subdirectory(src)
