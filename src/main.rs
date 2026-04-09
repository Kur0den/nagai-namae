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

    let first = select_choice(&choices, "始まりの単語を選んでね", "first");
    let last = select_choice(&choices, "終わりの単語を選んでね", "last");

    println!("始まり: {}", first);
    println!("終わり: {}", last);

    let mut current_strings = first;
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
    current_strings.push_str(&last);
    println!("結果: {}", current_strings);

}

fn select_choice(choices: &Vec<String>, prompt: &str, key: &str) -> String {
    loop {
        print!("{} (番号を入力、0で任意入力): ", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<usize>() {
            if n == 0 {
                let last = load_last_custom(key);
                if let Some(ref prev) = last {
                    print!("任意の文字列を入力してね (Enterで前回の「{}」を使用): ", prev);
                } else {
                    print!("任意の文字列を入力してね: ");
                }
                io::stdout().flush().unwrap();

                let mut custom = String::new();
                io::stdin().read_line(&mut custom).unwrap();
                let trimmed = custom.trim().to_string();

                let value = if trimmed.is_empty() {
                    last.unwrap_or_default()
                } else {
                    save_last_custom(key, &trimmed);
                    trimmed
                };
                return value;
            }
            if n >= 1 && n <= choices.len() {
                let value = choices[n - 1].clone();
                save_last_custom(key, &value);
                return value;
            }
        }
        println!("0〜{}の番号を入力してね", choices.len());
    }
}

fn load_last_custom(key: &str) -> Option<String> {
    let content = std::fs::read_to_string("./.last_custom").ok()?;
    content.lines()
        .find_map(|line| {
            let (k, v) = line.split_once('=')?;
            if k == key { Some(v.to_string()) } else { None }
        })
        .filter(|s| !s.is_empty())
}

fn save_last_custom(key: &str, value: &str) {
    let content = std::fs::read_to_string("./.last_custom").unwrap_or_default();
    let mut lines: Vec<String> = content.lines()
        .filter(|line| !line.starts_with(&format!("{}=", key)))
        .map(|s| s.to_string())
        .collect();
    lines.push(format!("{}={}", key, value));
    let _ = std::fs::write("./.last_custom", lines.join("\n"));
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
