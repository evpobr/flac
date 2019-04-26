set(utf8_SRCS
  ../share/utf8/charmaps.h
  ../share/utf8/charset.c
  ../share/utf8/charset.h
  ../share/utf8/iconvert.c
  ../share/utf8/iconvert.h
  ../share/utf8/utf8.c
  )

set(grabbag_SRCS
  ../share/grabbag/alloc.c
  ../share/grabbag/cuesheet.c
  ../share/grabbag/file.c
  ../share/grabbag/picture.c
  ../share/grabbag/replaygain.c
  ../share/grabbag/seektable.c
  ../share/grabbag/snprintf.c
  )

set(getopt_SRCS
  ../share/getopt/getopt.c
  ../share/getopt/getopt1.c
  )

set(replaygain_analysis_SRCS
  ../share/replaygain_analysis/replaygain_analysis.c
  )

set(replaygain_synthesis_SRCS
  ../share/replaygain_synthesis/replaygain_synthesis.c
  )

add_executable(flac
  analyze.c
  decode.c
  encode.c
  foreign_metadata.c
  main.c
  local_string_utils.c
  utils.c
  vorbiscomment.c
  analyze.h
  decode.h
  encode.h
  foreign_metadata.h
  local_string_utils.h
  utils.h
  vorbiscomment.h
  $<$<BOOL:${WIN32}>:../share/win_utf8_io/win_utf8_io.c>
  ${utf8_SRCS}
  ${grabbag_SRCS}
  ${getopt_SRCS}
  ${replaygain_analysis_SRCS}
  ${replaygain_synthesis_SRCS}
  ${PROJECT_BINARY_DIR}/config.h
  )

target_compile_definitions(flac PRIVATE HAVE_CONFIG_H)
if(CMAKE_SIZEOF_VOID_P EQUAL 4 AND NOT MSVC)
  target_compile_definitions(flac PRIVATE _FILE_OFFSET_BITS=64)
endif()

target_include_directories(flac
  PRIVATE
    ${PROJECT_BINARY_DIR}
  )

target_link_libraries(flac PRIVATE FLAC)
