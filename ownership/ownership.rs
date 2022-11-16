// OWNERSHIP = 소유권
// 어떤 변수가 어떤 데이터를 얼마만큼 가지고 있을 수 있는가?
// & = reference

fn return_country -> &String {
  let other_country = String::from("North Korea");
  &other_country;
}

fn main() {
  let country = String::from("South Korea");
  let other_count = return_country(); // 이렇게 할당하려고 하면 error 발생함.
  let _ref_one = &country; // 데이터를 읽을 수 있지만, 원형을 수정할 수 없음
  let _ref_two = &other_count; // 데이터를 읽을 수 있지만,  원형을 수정할 수 없음

  println!("Country is : {}\n", _ref_one);
  // println!("Other Country is : {}\n", _ref_two);
}