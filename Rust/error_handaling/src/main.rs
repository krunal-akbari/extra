use std::fs;
use std::io;

fn read_filename() -> Result<String, io::Error> {
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("string can not be get");
    fs::read_to_string(file_name)
}
fn main() {
    println!("hello\n");
    let resutl: _ = read_filename();
    println!("{:?}", resutl);
}
