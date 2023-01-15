enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_contain(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let kishan_money = Coin::Quarter;
    let money_value = value_contain(kishan_money);
    println!("{money_value}");
}
