// reference = pointer in rust

fn main() {
  let my_number = 15;
  let single_ref = &my_number;
  let double_ref = &single_ref;
  println!(" my_number is {0}\n single_ref is {1}\n double_ref is {2}", 
    my_number,
    single_ref,
    double_ref
  )
}