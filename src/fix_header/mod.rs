use std::io::{Seek, Write, Read, SeekFrom, Result};

#[cfg(test)]
mod tests;

// compressed logo
static GBA_LOGO: &'static [u8] = &[
    0x24,0xFF,0xAE,0x51,0x69,0x9A,0xA2,0x21,0x3D,0x84,0x82,0x0A,0x84,0xE4,0x09,0xAD,
	0x11,0x24,0x8B,0x98,0xC0,0x81,0x7F,0x21,0xA3,0x52,0xBE,0x19,0x93,0x09,0xCE,0x20,
	0x10,0x46,0x4A,0x4A,0xF8,0x27,0x31,0xEC,0x58,0xC7,0xE8,0x33,0x82,0xE3,0xCE,0xBF,
	0x85,0xF4,0xDF,0x94,0xCE,0x4B,0x09,0xC1,0x94,0x56,0x8A,0xC0,0x13,0x72,0xA7,0xFC,
	0x9F,0x84,0x4D,0x73,0xA3,0xCA,0x9A,0x61,0x58,0x97,0xA3,0x27,0xFC,0x03,0x98,0x76,
	0x23,0x1D,0xC7,0x61,0x03,0x04,0xAE,0x56,0xBF,0x38,0x84,0x00,0x40,0xA7,0x0E,0xFD,
	0xFF,0x52,0xFE,0x03,0x6F,0x95,0x30,0xF1,0x97,0xFB,0xC0,0x85,0x60,0xD6,0x80,0x25,
	0xA9,0x63,0xBE,0x03,0x01,0x4E,0x38,0xE2,0xF9,0xA2,0x34,0xFF,0xBB,0x3E,0x03,0x44,
	0x78,0x00,0x90,0xCB,0x88,0x11,0x3A,0x94,0x65,0xC0,0x7C,0x63,0x87,0xF0,0x3C,0xAF,
	0xD6,0x25,0xE4,0x8B,0x38,0x0A,0xAC,0x72,0x21,0xD4,0xF8,0x07,];

static FIXED_DATA: &'static [u8] = &[
    0x96, // I'm not sure anyone knows what this is
    0x00, // Main unit code
    0x00, // Device type
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserved
];

pub fn fix_header<T: Write + Seek + Read>(
    mut cart_file: T,
    game_title: &[u8; 12],
    game_code: &[u8; 4],
    maker_code: &[u8; 2],
    version: u8) -> Result<()> {
    // first order of business: add the gba logo.
    cart_file.seek(SeekFrom::Start(0x04))?;
    cart_file.write_all(GBA_LOGO)?;
    cart_file.write_all(game_title)?;
    cart_file.write_all(game_code)?;
    cart_file.write_all(maker_code)?;
    cart_file.write_all(FIXED_DATA)?;
    cart_file.write_all(&[version])?;

    let mut checksum_area = [0u8; 28];
    cart_file.seek(SeekFrom::Start(0xA0))?;
    cart_file.read_exact(&mut checksum_area)?;

    let checksum = -(0x19i8.wrapping_add(checksum_area.iter().fold(0i8, |a, n| a.wrapping_add(*n as i8))));

    cart_file.seek(SeekFrom::Start(0xBD))?;
    cart_file.write_all(&[checksum as u8, 0u8, 0u8])?;

    Ok(())
}