pub mod utils {
    use std::fmt;

    use prettytable::{row, Table};

    use crate::charset::charset::{Charset, Utf8Charset};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub enum ByteType {
        Ascii,
        Utf8Base,
        Utf8Continuation,
        AsciiNewLine,
        Unknown
    }

    impl fmt::Display for ByteType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub fn get_byte_type(byte: &u8) -> ByteType {
        if byte == &0x0A {
            return ByteType::AsciiNewLine;
        }

        if byte >> 7 == 0 {
            return ByteType::Ascii;
        }

        if byte >> 5 == 0b110 || byte >> 4 == 0b1110 || byte >> 3 == 0b11110 {
            return ByteType::Utf8Base;
        }

        if byte >> 6 == 0b10 {
            return ByteType::Utf8Continuation;
        }

        ByteType::Unknown
    }

    pub fn format_grapheme(table: &mut Table, grapheme: &char, use_truecolors: &bool) {
        let grapheme_string = grapheme.to_string();
        let bytes = grapheme_string.as_bytes();

        let first_byte = bytes.get(0).unwrap();

        table.add_row(row![
            grapheme,
            format!("{:0b}", first_byte),
            format!("{}", (*first_byte) as u32),
            (*grapheme) as u32,
            format!("0x{:X}", (*grapheme) as u32),
            get_byte_type(&first_byte),
            Utf8Charset::get_description(&((*grapheme) as u32)),
        ]);


        for byte in &bytes[1..] {
            table.add_row(row![
                " ",
                format!("{:08b}", byte),
                format!("{}", (*byte) as u32),
                " ",
                " ",
                get_byte_type(&byte),
                " ",
            ]);
        }

    }
}
