use std::io;

fn main() {
  println!("Step right up, guess the number");
  let mut guess = String::new();
  io::stdin().read_line(&mut guess)
  .ok()
  .expect("failed to get the input");
  println!("Your guess was {}", guess);
}