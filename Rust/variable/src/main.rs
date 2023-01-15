fn main() {
    let s = String::from("this is kishan and i want to see this is working or not");
    words(&s);
}
fn words(s: &String) {
    let mut start = 0;
    for (i, &index) in s.as_bytes().iter().enumerate() {
        if index == b' ' {
            let word = &s[start..i];
            println!("{word}");
            start = i + 1;
        }
    }
    let word = &s[start..];
    println!("{word}");
}
