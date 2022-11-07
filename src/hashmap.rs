use std::io;
use std::collections::HashMap;

// returns the element with the highest frequency
pub fn mode() -> (i32, i32) {
  println!("Enter integer values to get the mode (most frequent integer) from separated by a space: ");
  let mut input = String::new();
  io::stdin()
	  .read_line(&mut input)
	  .ok()
	  .expect("Failed to read line");

  let mut ints = Vec::new();

  let raw = input.split_whitespace();
  for r in raw {
	let  value = r.parse::<i32>().expect("Failed to parse integer. Enter numbers only, separated by a space.");
	ints.push(value);
  }

  let mut map = HashMap::new();
  for i in ints {
	let count = map.entry(i).or_insert(0);
	*count += 1;
  }

  let mut max = 0;
  let mut max_key = 0;
  for (key, value) in &map {
	if value > &max {
	  max = *value;
	  max_key = *key;
	}
  }

  (max_key, max)
}