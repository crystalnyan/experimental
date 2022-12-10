use crate::thread::make_breakfast;

mod vec;
mod hashmap;
mod box_pointer;
mod thread;

fn main() {
  println!("Result of the median fn on given integers: {}", vec::median());

  let res = hashmap::mode();
  println!("Result of the mode fn on given integers (most frequent integer): {}", res.0);

  let  animals: Box<[&str]> = Box::new(["cat", "dog", "turtle"]);
  let colors: Box<[&str]> = Box::new(["blue", "yellow", "red", "pink", "green"]);

  // Rust would not let us define a fn operating on arrays of unknown sizes but
  // with Box smart pointer arrays are created on the heap and fn gets a fixed size Box pointer
  // as a parameter so all is clean!
  println!("Running box example on colors array: ");
  box_pointer::print_array(colors);
  println!("Running box example on animal array: ");
  box_pointer::print_array(animals);

  // Simple threads example with concurrency
  make_breakfast();
}


