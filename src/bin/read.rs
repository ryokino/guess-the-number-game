use std::io::{self, BufRead}; // inputを読み込むためのモジュール。BufReadを使うと1行ずつ読みやすい

fn main() {
    let stdin = io::stdin(); // selfを使っているからioを使っていい。こうすることで、Rustの標準入力用の入り口(ハンドル)を取得できる
                             // ハンドルはグローバルな共有リソースに向いている。プロセスに一つしか標準出力はないけど複数のスレッドがあるからね。
                             // mutex: 一人ずつしか使えないようにする。 stdinはmutexの制限があるのでちゃんと .lock() を使ってね
                             // ロックを取得すると、スレッドがブロックされて、競合が起きない

    let mut buffer = String::new();

    // 標準入力から1行読み込む
    stdin.lock().read_line(&mut buffer).unwrap();
    // read_lineは改行までの文字列を読む

    let food_str = buffer.trim(); // 末尾の改行を除去
                                  // trim は String(可変)の静的メソッドで、先頭と末尾の空白文字を除去する
                                  // 返り値は &str 型=Stringのスライス = 一時的に表示しているだけ
                                  // &strにすることで安全に読み取り専用にすることができる。

    println!("I like {}!", food_str);
}
