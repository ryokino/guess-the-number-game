mod game;
mod input;

fn main() {
    println!("=== Guess The Number ===");

    let min = input::read_int("最小の数を入力してね: ");
    let max = input::read_int("最大の数を入力してね: ");

    if min > max {
        eprintln!("最小値 < 最大値 で入力してね〜👍");
        return;
    }

    game::play(min, max);
}
