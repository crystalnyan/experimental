/// Print array of string slices of unknown at compile time size
///
/// # Examples
/// ```
///  let animals: Box<[&str]> = Box::new(["cat", "dog", "turtle"]);
///  let colors: Box<[&str]> = Box::new(["blue", "yellow", "red", "pink", "green"]);
///
///  println!("Running box example on colors array: ");
///  print_array(colors);
///  println!("Running box example on animal array: ");
///  print_array(animals);
/// ```
pub  fn  print_array (array: Box<[&str]>) {
  for item in array.iter() {
    println!("{}", *item);
  }
}