pub  fn  print_array (array: Box<[&str]>) {
  for item in array.iter() {
    println!("{}", *item);
  }
}