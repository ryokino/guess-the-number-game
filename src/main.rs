use std::io;

fn main() {
    // 標準入力を促して最小値と最大値を入力させる。
    let stdin = io::stdin();

    let mut min_str = String::new();
    let mut max_str = String::new();

    println!("Enter the minimum number: ");

    stdin.read_line(&mut min_str).unwrap();
    let min: i32 = match min_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };

    println!("Enter the maximum number: ");

    stdin.read_line(&mut max_str).unwrap();

    let max: i32 = match max_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };

    if min > max {
        println!("The minimum number must be less than the maximum number.");
        return;
    }

    let secret_number = rand::random_range(min..=max);

    let mut guess = String::new();

    println!("Please guess the number between {} and {}", min, max);
    stdin.read_line(&mut guess).unwrap();

    let guess: i32 = match guess.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };

    if guess == secret_number {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
