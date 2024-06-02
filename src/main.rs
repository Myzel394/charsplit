use std::{char, iter::Iterator, slice::Iter};

macro_rules! format_num_as_byte {
    ($a:expr) => {
        format!("{:08b}", $a)
    };
}

macro_rules! _format_grapheme {
    ($byte:expr) => {
        _format_grapheme!($byte, "")
    };
    ($byte:expr, $character:expr) => {
        format!(
            "{}\t\t{}\n",
            $character,
            format_num_as_byte!($byte),
        )
    };
}

fn format_grapheme(grapheme: &char) -> String {
    let grapheme_string = grapheme.to_string();
    let bytes = grapheme_string.as_bytes();

    let mut string = "".to_string();
    let first_byte = &bytes.get(0).unwrap();
    string.push_str(
        &_format_grapheme!(first_byte, grapheme)
    );

    for byte in &bytes[1..] {
        string.push_str(
            &_format_grapheme!(byte)
        );
    }

    string
}

fn main() {
    let input = String::from("谁会");
    let graphemes = input.chars();

    for grapheme in graphemes {
        let output = format_grapheme(&grapheme);

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
