use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("{random_number}");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("cant convert into int");
        print!("this is your valued gurss {guess}");

        let guess: u32 = guess.trim().parse().expect("this is wrign");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Number is less"),
            Ordering::Greater => println!("Number is grater then"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}
