const WHITESPACE: u8 = 1 << 0;
const DIGIT: u8 = 1 << 1;
const IDENTIFIER_START: u8 = 1 << 2;
const IDENTIFIER_CONTINUE: u8 = 1 << 3;

const FLAGS: [u8; 256] = {
    let mut table = [0_u8; 256];
    let mut index = 0_u8;

    loop {
        table[index as usize] = match index {
            b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C => WHITESPACE,
            b'0'..=b'9' => DIGIT | IDENTIFIER_CONTINUE,
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => IDENTIFIER_START | IDENTIFIER_CONTINUE,
            _ => 0,
        };

        if index == 255 {
            break;
        }

        index += 1;
    }

    table
};

/// Byte classification using a lookup table.
pub struct ByteLookup;

impl ByteLookup {
    /// Checks if a byte is whitespace.
    #[must_use]
    #[inline(always)]
    pub const fn is_ascii_whitespace(byte: u8) -> bool {
        FLAGS[byte as usize] & WHITESPACE != 0
    }

    /// Checks if a byte is a digit.
    #[must_use]
    #[inline(always)]
    pub const fn is_digit(byte: u8) -> bool {
        FLAGS[byte as usize] & DIGIT != 0
    }

    /// Checks if a byte can start an identifier.
    #[must_use]
    #[inline(always)]
    pub const fn is_identifier_start(byte: u8) -> bool {
        FLAGS[byte as usize] & IDENTIFIER_START != 0
    }

    /// Checks if a byte can continue an identifier.
    #[must_use]
    #[inline(always)]
    pub const fn is_identifier_continue(byte: u8) -> bool {
        FLAGS[byte as usize] & IDENTIFIER_CONTINUE != 0
    }
}
