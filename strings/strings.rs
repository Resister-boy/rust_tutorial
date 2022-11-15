fn main() {
  // String : growable string, owned type
  // => String = Sized type
  let mut my_name = String::from("resister_boy");
  let mut my_other_name = "resister_boy".to_string();

  // &str = ref str "string slice"
  // => &str = dynamic type
  let my_other_other_name = "resister_boy";

  my_name.push('!');
  my_other_name.push('!');
  // my_other_other_name.push('!'); // Error because is &str

  println!("name is {my_name}\nname is {my_other_name}\nname is {my_other_other_name}");
}

// String은 살짝 primitive type의 string => mutable
// &str은 reference type의 string => immutable