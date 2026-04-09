use rand::{ Rng, prelude::IndexedRandom };
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("data.txtを読み込むよ");
    let choices = load_choices();

    println!("選択肢:");
    for (i, c) in choices.iter().enumerate() {
        println!("  {}: {}", i + 1, c);
    }

    let first = select_choice(&choices, "始まりの単語を選んでね");
    let last = select_choice(&choices, "終わりの単語を選んでね");

    println!("始まり: {}", first);
    println!("終わり: {}", last);

    let mut current_strings = String::from(first);
    let mut count = 1;
    loop {
        let (new_strings, result) = maybe_append(&choices, current_strings);
        println!("{}回目の抽選: {}", count, result);
        current_strings = new_strings;
        if !result {
            break
        }
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }
    current_strings.push_str(last);
    println!("結果: {}", current_strings);

}

fn select_choice<'a>(choices: &'a Vec<String>, prompt: &str) -> &'a String {
    loop {
        print!("{} (番号を入力): ", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<usize>() {
            if n >= 1 && n <= choices.len() {
                return &choices[n - 1];
            }
        }
        println!("1〜{}の番号を入力してね", choices.len());
    }
}

fn load_choices() -> Vec<String> {
    std::fs::read_to_string("./data.txt")
        .expect("data.txtがひらけなかったよ")
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn maybe_append(choices: &Vec<String>, mut current_strings: String) -> (String, bool) {
    let mut rng = rand::rng();
    let result = rng.next_u32() % 2 == 0;
    if result {
        // 50%でなんかふえる
        current_strings.push_str(choices.choose(&mut rng).unwrap());
    }
    return (current_strings, result);
}
