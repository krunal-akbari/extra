use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();
    score.insert(String::from("money"), 1);
    score.insert(String::from("moa"), 1);
    score.insert(String::from("money"), 2);

    for (k, v) in &score {
        println!("{k} {v}\t");
    }

    let key_name = String::from("money");
    let scor = score.get(&key_name).copied().unwrap_or(0);
    println!("{scor}");
}
