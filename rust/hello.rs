use std::io;

fn main() {
  println!("Hello world!!");

  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Error when input!!");

  println!("You enter {}", inp);
}
