#![allow(dead_code)]

extern "C" {
    pub(crate) fn FLAC__crc8(data: *const u8, len: u32) -> u8;
    pub(crate) fn FLAC__crc16(data: *const u16, len: u32) -> u16;
    pub(crate) fn FLAC__crc16_update_words32(words: *const u32, len: u32, crc: u16) -> u16;
    pub(crate) fn FLAC__crc16_update_words64(words: *const u64, len: u32, crc: u16) -> u16;
}
