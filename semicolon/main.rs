// () -> empty tuple = (void)

fn empty_tuple() {
  // 함수가 아무런 값도 리턴하지 않을 경우 empty tuple type으로 정의됨
}

fn main() {
  let tuple = empty_tuple();
  println!({}, tuple); // 이 경우error가 발생하지만, {:?} {:#?} 등 debug print를 할 수도 있음.
  // 일반적인 print를 display print,
  // 프로그래밍 할 때 사용되는 print를 debug print

  // main 함수도 empty_tuple function에 해당.
}