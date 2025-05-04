pub const MAX_TRIES: u8 = 5;

pub fn play(min: i32, max: i32) {
    let secret_number = rand::random_range(min..=max);

    println!("{} ~ {}の数を当ててね！", min, max);

    for turn in 1..=MAX_TRIES {
        let guess = crate::input::read_int(&format!("[{turn}/{MAX_TRIES}] ▶︎ "));

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("正解！{}回で当たりました！", turn);
                return;
            }

            std::cmp::Ordering::Less => {
                println!("↑ もっと大きいよ！");
            }

            std::cmp::Ordering::Greater => {
                println!("↓ もっと小さいよ！");
            }
        }
    }

    println!("💔 残念！正解は {secret_number} だったよ〜");
}
