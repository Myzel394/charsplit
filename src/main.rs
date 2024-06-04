use core::fmt;
use std::{char, io::{self, Read}, iter::Iterator, slice::Iter};
use colored::Colorize;

mod config;

use config::config::Config;

macro_rules! format_num_as_byte {
    ($a:expr) => {
        format!("{:08b}", $a)
    };
}

macro_rules! _format_grapheme {
    ($byte:expr, $type:expr, $truecolor:expr) => {
        _format_grapheme!($byte, "", $type, $truecolor)
    };
    ($byte:expr, $character:expr, $type: expr, $truecolor:expr) => {
        format!(
            "{}\t{} ({})\t=\t{}\n",
            $character.to_string().yellow(),
            (|| {
                let text = format_num_as_byte!($byte);

                if ($truecolor) {
                    // Use terminal color   
                    return text.truecolor(128, 128, 128); 
                } else {
                    return text.bright_green();
                }
            })(),
            format!("{:03}", $byte).bright_blue(),
            $type,
        )
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ByteType {
    Ascii,
    Utf8Base,
    Utf8Continuation,
    NLFeed,
    Unknown
}

impl fmt::Display for ByteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_byte_type(byte: &u8) -> ByteType {
    if byte == &0x0A {
        return ByteType::NLFeed;
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

fn format_grapheme(grapheme: &char, use_truecolors: &bool) -> String {
    let grapheme_string = grapheme.to_string();
    let bytes = grapheme_string.as_bytes();

    let mut string = "".to_string();
    let first_byte = &bytes.get(0).unwrap();
    string.push_str(
        &_format_grapheme!(first_byte, grapheme, get_byte_type(&first_byte), *use_truecolors)
    );

    for byte in &bytes[1..] {
        string.push_str(
            &_format_grapheme!(byte, get_byte_type(&byte), *use_truecolors)
        );
    }

    string
}

fn main() {
    let config = Config::build();

    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let graphemes = input.chars();

    for grapheme in graphemes {
        let output = format_grapheme(&grapheme, &config.use_truecolor);

        println!("{}", output);
    }

    // let bytes = input.as_bytes();
    // let formatted_bytes: Vec<String> = bytes.iter().map(|byte| format_num_as_byte(&byte)).collect();
    //
    // println!("{:?}", &formatted_bytes);
    // Split per 8 bits
    // let byte_indexes: Vec<u8> = utf8_indexes.flat_map(|index| split_into_bytes(&index)).collect();
    // dbg!(&byte_indexes);
    //
    // // show in binary notation
    // println!("{:?}", byte_indexes.iter().map(|index| format_num_as_byte(&index)).collect::<Vec<String>>());
}
