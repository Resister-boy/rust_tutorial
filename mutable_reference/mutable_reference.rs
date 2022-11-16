// & immutable reference / shared reference
// & mut mutable reference // unique reference

fn main() {
  let mut my_number = 10;
  let mut my_other_number = 11;
  let num_ref = &mut my_number; // 아래 데이터 수정을 위해 &(Ampersand)
  let num_other_ref = &mut &mut my_other_number;

  *num_ref = 20; // *(Asterisk)로 Dereferencing을 해줘야 메모리 안에 데이터 수정 가능
  **num_other_ref = 22;
  println!("return {my_number}\n");
  println!("return {my_other_number}\n");
}