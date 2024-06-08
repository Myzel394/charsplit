pub mod charset {
    use crate::utf8_groups::utf8_groups;

    pub trait Charset {
        fn get_description(value: &u32) -> String;
    }

    pub struct Utf8Charset();

    impl Charset for Utf8Charset {
        fn get_description(value: &u32) -> String {
            return utf8_groups::get_group(value);
        }
    }
}
