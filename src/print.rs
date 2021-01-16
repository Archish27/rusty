pub fn run() {
  // Print to console
  println!("Hello from print.rs file");

  //Basic Formatting
  println!("{} is from {}", "Archish", "Mumbai");

  //Positional Arguments
  println!(
    "{0} is from {1} and {0} likes to play {2}",
    "Archish", "Mumbai", "Cricket"
  );

  //Name Arguments
  println!(
    "{name} likes to play {sport}",
    name = "Archish",
    sport = "Cricket"
  );

  //Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 27, 10, 96);

  //Placeholder for debug trait
  println!("{:?}", (12, true, "Hello"));

  //Basic math
  println!("{} + {} = {}", 27, 10, 27 + 10);
}
