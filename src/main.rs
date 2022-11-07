pub mod vec;
mod hashmap;

fn main() {
  println!("Result of the median fn on given integers: {}", vec::median());

  let (integer, count) = hashmap::mode();
  println!("Result of the mode fn on given integers (most frequent integer): {}", integer);
}


