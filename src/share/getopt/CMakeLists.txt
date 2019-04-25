cmake_minimum_required(VERSION 3.12)

find_package(Intl)

check_include_file("libintl.h" HAVE_LIBINTL_H)
check_include_file("string.h" HAVE_STRING_H)

add_compile_definitions(
    $<$<BOOL:${HAVE_STRING_H}>:HAVE_STRING_H>)

add_library(getopt STATIC
    getopt.c
    getopt1.c)

if(Intl_FOUND)
    target_include_directories(getopt PRIVATE ${Intl_INCLUDE_DIRS})
    target_link_libraries(getopt PRIVATE ${Intl_LIBRARIES})
    target_compile_definitions(getopt PRIVATE HAVE_LIBINTL_H)
endif()
