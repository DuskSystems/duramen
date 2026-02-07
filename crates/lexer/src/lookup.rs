pub const IDENTIFIER_TABLE: [bool; 256] = {
    let mut table = [false; 256];

    let mut index = 0;
    while index < 256 {
        table[index] = matches!(
            index as u8,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'
        );

        index += 1;
    }

    table
};

pub const WHITESPACE_TABLE: [bool; 128] = {
    let mut table = [false; 128];

    let mut index = 0;
    while index < 128 {
        table[index] = matches!(index as u8, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C);
        index += 1;
    }

    table
};

pub const INTEGER_TABLE: [bool; 256] = {
    let mut table = [false; 256];

    let mut index = 0;
    while index < 256 {
        table[index] = matches!(index as u8, b'0'..=b'9');
        index += 1;
    }

    table
};
