#![warn(dead_code)]
fn sum_of_difit(number: u32) -> u32 {
    if number <= 9 {
        return number;
    }
    return sum_of_difit(number / 10) + number % 10;
}

fn sum_of_n(number: u32) -> u32 {
    if number <= 1 {
        return 1;
    }
    return number + sum_of_n(number - 1);
}

fn main() {
    let ans = sum_of_n(10);
    println!("{ans}");
}
