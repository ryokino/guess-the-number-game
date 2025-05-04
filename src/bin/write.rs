use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"Sukiyaki\n").unwrap();
    handle.flush().unwrap();
}
