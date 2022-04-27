use std::io;

fn main(){
  println!("Welcome to Rusty Typer!");
  println!("Press any key to start");

  // need to make this not block the stdin
  io::stdin().read_line(&mut String::new()).unwrap();
}