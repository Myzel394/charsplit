pub mod cli {
    use std::fmt;

    use clap::{arg, command, Parser};
    use clap::builder::TypedValueParser as _;
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
    pub enum Encoding {
        UTF8,
    }

    impl Encoding {
        pub fn from_str(s: &str) -> Self {
            match s {
                "utf8" => Encoding::UTF8,
                _ => Encoding::UTF8,
            }
        }
    }

    impl fmt::Display for Encoding {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Parser, Debug)]
    #[command(name = "charsplit")]
    #[command(bin_name = "charsplit")]
    #[command(about = "Split a string into its bytes and characters")]
    #[command(long_about = "charsplit is a small utility tool that will give you information about your string input. It will split the string into its bytes and graphemes, and give you information about them.")]
    #[command(version = "0.1.0")]
    pub struct Arguments {
        /// What encoding to use
        #[arg(
            short, 
            long,
            default_value_t = Encoding::UTF8,
            value_parser = clap::builder::PossibleValuesParser::new(["UTF8"])
                .map(|value| Encoding::from_str(&value))
        )]
        pub encoding: Encoding,

        #[arg(default_value_t = String::new())]
        pub text: String,
    }
}
