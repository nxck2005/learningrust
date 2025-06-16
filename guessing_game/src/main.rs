use std::io;

fn main() {
   println!("Guessing Game");
   println!("Enter your number: ");

   let mut guess = String::new();

   io::stdin()
        .read_line(&mut guess)
        .expect("Error");

    println!("You guessed {}.", guess);
}
