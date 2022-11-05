use std::io;

pub fn median() -> i32 {
  println!("Enter integer values to get the median from separated by a space: ");
  let mut input = String::new();
  io::stdin()
	  .read_line(&mut input)
	  .ok()
	  .expect("Failed to read line");

  let mut ints = Vec::new();
  let raw = input.split_whitespace();
  for r in raw {
	ints.push(r.parse::<i32>().unwrap());
  }

  ints.sort();
  let mid = ints.len() / 2;
  ints[mid]
}