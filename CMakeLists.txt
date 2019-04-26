cmake_minimum_required(VERSION 3.1..3.14)

add_subdirectory(src)

include(CheckFunctionExists)
include(CheckLibraryExists)
include(CheckIncludeFile)

check_library_exists(m lround "" HAVE_LIBM)
if(HAVE_LIBM)
  list(APPEND CMAKE_REQUIRED_LIBRARIES m)
endif()

check_function_exists(fseeko HAVE_FSEEKO)
check_function_exists(lround HAVE_LROUND)

check_include_file(inttypes.h HAVE_INTTYPES_H)

configure_file(config.h.cmake config.h)
