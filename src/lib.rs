use std::fs;
use std::path::Path;

#[macro_export]
macro_rules! read_input {
    () => {
        $crate::read_input(file!())
    };
}

pub fn read_input<P: AsRef<Path>>(bin_path: P) -> String {
    let file_name = bin_path
        .as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".rs", ".txt");

    fs::read_to_string(Path::new("../../inputs").join(&file_name))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", file_name))
}
