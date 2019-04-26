add_library(FLAC
  bitmath.c
  bitreader.c
  bitwriter.c
  cpu.c
  crc.c
  fixed.c
  fixed_intrin_sse2.c
  fixed_intrin_ssse3.c
  float.c
  format.c
  lpc.c
  lpc_intrin_sse.c
  lpc_intrin_sse2.c
  lpc_intrin_sse41.c
  lpc_intrin_avx2.c
  lpc_intrin_vsx.c
  md5.c
  memory.c
  metadata_iterators.c
  metadata_object.c
  stream_decoder.c
  stream_encoder.c
  stream_encoder_intrin_sse2.c
  stream_encoder_intrin_ssse3.c
  stream_encoder_intrin_avx2.c
  stream_encoder_framing.c
  window.c
  $<$<BOOL:${WIN32}>:windows_unicode_filenames.c>
  ${PROJECT_BINARY_DIR}/config.h)

target_compile_definitions(FLAC
  PRIVATE
    HAVE_CONFIG_H
    $<$<NOT:$<BOOL:${BUILD_SHARED_LIBS}>>:FLAC__NO_DLL>
    )
if(CMAKE_SIZEOF_VOID_P EQUAL 4 AND NOT MSVC)
  target_compile_definitions(FLAC PRIVATE _FILE_OFFSET_BITS=64)
endif()

target_include_directories(FLAC
  PUBLIC
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>
  PRIVATE
    include
    ${PROJECT_BINARY_DIR}
    )

if(HAVE_LIBM)
  target_link_libraries(FLAC PRIVATE m)
endif()
  
if(ENABLE_OGG)
  target_link_libraries(FLAC PRIVATE Ogg::ogg)
  target_sources(FLAC
    PRIVATE
      ogg_decoder_aspect.c
      ogg_encoder_aspect.c
      ogg_helper.c
      ogg_mapping.c
    )
endif()
