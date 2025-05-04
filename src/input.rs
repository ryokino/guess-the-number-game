use std::io::{self, Write};

pub fn read_int(prompt: &str) -> i32 {
    let stdin = io::stdin();

    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        match buf.trim().parse::<i32>() {
            Ok(n) => break n,
            Err(_) => {
                println!("整数で入力してね~~~~~😅");
                continue;
            }
        }
    }
}
