extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Step right up, guess the number");
  let secret_num = rand::thread_rng().gen_range(1, 101);
  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
      .ok()
      .expect("failed to get the input");
    println!("Your guess was {}", guess);

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("let's keep it a number you putz");
        continue;
      }
    };

    match guess.cmp(&secret_num) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
        println!("You win!");
        break;
      }
    }
  }
}