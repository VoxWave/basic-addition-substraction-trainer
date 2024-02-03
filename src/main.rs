use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() {
    println!("Welcome to addition and substraction training.");
    let mut level = 1;
    let mut rng = thread_rng();
    loop {
        println!("\nLevel {}", level);
        let sign: bool = rand::random();
        let x = rng.gen_range(1..=level);
        let y = rng.gen_range(1..=level);
        let answer;
        if sign {
            //+
            println!("{} + {} = ", x, y);
            answer = x + y;
        } else {
            //-
            println!("{} - {} = ", x, y);
            answer = x - y;
        }
        
        loop {
            let mut guess_string = String::new();
            stdin().read_line(&mut guess_string).unwrap();
            let guess = guess_string.trim().parse::<i64>().unwrap();
            if guess == answer {
                println!("Correct!");
                break;
            } else {
                println!("Try again!");
            }
        }
        level += 1;
    }
}
