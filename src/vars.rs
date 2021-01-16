// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Archish";
  //if this is intentional, prefix it with an underscore: `_age`
  let _age = 24;

  // age = 25; -> This will throw error cannot assign twice to immutable variable
  //`#[warn(unused_assignments)]` on by default
  let mut new_age = 24;
  println!("My name is {} & I am {}", name, new_age);
  new_age = 25;
  println!("My name is {} & I am {}", name, new_age);

  //Defining constant
  const ID: i32 = 001;
  println!("ID : {}", ID);

  //Assign multiple variables
  let (my_name, my_age) = ("Archish", 24);
  println!("My name is {} & I am {}", my_name, my_age);
}
