#![allow(dead_code, non_camel_case_types, non_snake_case)]

use libc::{c_int, c_void, FILE};

use crate::FLAC__bool;

pub(crate) type BitReaderReadCallback = unsafe extern "C" fn(
    buffer: *mut u8,
    bytes: *mut usize,
    client_data: *mut c_void,
) -> FLAC__bool;

#[repr(C)]
pub(crate) struct BitReader {
    dummy: [u8; 0],
}

extern "C" {
    pub(crate) fn FLAC__bitreader_new() -> *mut BitReader;
    pub(crate) fn FLAC__bitreader_delete(br: *mut BitReader);
    pub(crate) fn FLAC__bitreader_init(
        br: *mut BitReader,
        rcb: *mut BitReaderReadCallback,
        cd: *mut c_void,
    );
    pub(crate) fn FLAC__bitreader_free(br: *mut BitReader);
    pub(crate) fn FLAC__bitreader_clear(br: *mut BitReader) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_reset_read_crc16(br: *mut BitReader, seed: u16);

    pub(crate) fn FLAC__bitreader_dump(br: *const BitReader, out: *mut FILE);

    pub(crate) fn FLAC__bitreader_get_read_crc16(br: *mut BitReader) -> u16;

    pub(crate) fn FLAC__bitreader_is_consumed_byte_aligned(br: *const BitReader) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_bits_left_for_byte_alignment(br: *const BitReader) -> u32;
    pub(crate) fn FLAC__bitreader_get_input_bits_unconsumed(br: *const BitReader) -> u32;

    pub(crate) fn FLAC__bitreader_read_raw_uint32(
        br: *mut BitReader,
        val: *mut u32,
        bits: u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_raw_int32(
        br: *mut BitReader,
        val: *mut i32,
        bits: u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_raw_uint64(
        br: *mut BitReader,
        val: *mut u64,
        bits: u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_uint32_little_endian(
        br: *mut BitReader,
        val: *mut u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_skip_bits_no_crc(br: *mut BitReader, bits: u32) -> FLAC__bool;

    pub(crate) fn FLAC__bitreader_skip_byte_block_aligned_no_crc(
        br: *mut BitReader,
        nvals: u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_byte_block_aligned_no_crc(
        br: *mut BitReader,
        val: *mut u8,
        nvals: u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_unary_unsigned(
        br: *mut BitReader,
        val: *mut u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_rice_signed(
        br: *mut BitReader,
        val: *mut c_int,
        parameter: u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_rice_signed_block(
        br: *mut BitReader,
        vals: *mut c_int,
        nvals: u32,
        parameter: u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_utf8_uint32(
        br: *mut BitReader,
        val: *mut u32,
        raw: *mut u8,
        rawlen: *mut u32,
    ) -> FLAC__bool;
    pub(crate) fn FLAC__bitreader_read_utf8_uint64(
        br: *mut BitReader,
        val: *mut u64,
        raw: *mut u8,
        rawlen: *mut u32,
    ) -> FLAC__bool;
}
