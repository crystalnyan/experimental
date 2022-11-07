use std::io;

// in a sorted collection returns the element at the middle
pub fn median() -> i32 {
  println!("Enter integer values to get the median from separated by a space: ");
  let mut input = String::new();
  io::stdin()
	  .read_line(&mut input)
	  .ok()
	  .expect("Failed to read line");

  let mut ints = Vec::new();

  let mut raw = input.split_whitespace();
  for r in raw {
	let  value = r.parse::<i32>().expect("Failed to parse integer. Enter numbers only, separated by a space.");
	ints.push(value);
  }

  ints.sort();
  let mid = ints.len() / 2;
  ints[mid]
}