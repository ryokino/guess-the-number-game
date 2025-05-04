use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    // 標準入力から1行読み込む
    stdin.lock().read_line(&mut buffer).unwrap();

    let food_str = buffer.trim(); // 改行を除去
    println!("I like {}!", food_str);
}
