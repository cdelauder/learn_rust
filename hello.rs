use std::io;

fn main() {
  println!("Step right up, guess the number");
  let mut guess = String::new();
  io::stdin().readline(&mut guess)
  .ok()
  .expect("failed to get the input");
  println!("Your guess was {}", guess);
}