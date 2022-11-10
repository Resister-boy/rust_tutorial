fn give_number(one: i16, two: i16) -> i16 { // return i16 type
  let multiplied_by_ten = { // code block
    let first_number : i16 = 10;
    first_number * one  * two // return first_number * one * two
  };
  multiplied_by_ten // return 
}

fn main() {
  let my_number = give_number(9, 8);
  println!("{}", my_number);
}

