pub mod charset {
    use std::ops::RangeBounds;

    pub trait Charset {
        fn get_description(value: &u32) -> String;
    }

    pub struct Utf8Charset();

    impl Charset for Utf8Charset {
        fn get_description(value: &u32) -> String {
            return (match value {
                0..=31 => "Control Codes (C0)",
                32..=47 => "ASCII Punctuation & Symbols",
                48..=57 => "ASCII Digits",
                58..=64 => "ASCII Punctuation & Symbols",
                65..=90 => "Latin Alphabet: Uppercase",
                91..=96 => "ASCII Punctuation & Symbols",
                97..=122 => "Latin Alphabet: Lowercase",
                123..=126 => "ASCII Punctuation & Symbols",
                127 => "Control Codes (Delete)",
                128..=159 => "Control Codes (C1)",
                160..=191 => "Latin-1 Punctuation & Symbols",
                192..=214 => "Letters: Uppercase",
                215 => "Math",


                0x04f9..=0x04ff => "Cyrillic",

                _ => ""
            }).to_string()
        }
    }
}
