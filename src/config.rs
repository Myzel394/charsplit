pub mod config {
    use std::env;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Config {
        pub use_truecolor: bool,
    }

    impl Config {
        pub fn build() -> Config {
            Config {
                use_truecolor: (|| {
                    return false;
                    let value = env::var("COLORTERM").unwrap_or("".to_string());

                    return value == "truecolor" || value == "24bit";
                })(),
            }
        }
    }
}
