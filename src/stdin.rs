use std::io::{stdin, BufReader, Read};

pub fn read_from_stdin() -> String {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer: Vec<u8> = Vec::new();

    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read from stdin.");

    String::from(std::str::from_utf8(&buffer).expect("Failed to convert stdin to string."))
}
