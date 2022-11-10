// mutability
// shadowing

// Rust는 immutable by defualt라는 특징이 있음 => 기본적으로 변수를 바꿀 수 없음

fn print_fn() {
  let my_variable = 10;
  println!("{}\n", my_variable);
  {
    let my_variable = "It calls shadowing"; 
    // 위와 같은 방식으로 shadowing을 사용할 수 있음.
    println!("{}\n", my_variable);
  }
  println!("{}\n", my_variable);
  // shadowing은 원래 있던 변수를 없애는 것이 아니라 덮어씌우는 것.
  // 위와 같이 코드블럭, 함수 등의 안에서만 적용.
}

fn main() {
  let my_number = 10; 
  // 기본적으로 let을 사용하여 변수를 선언하면 변수를 바꿀 수 없음.
  let mut your_number = 20;
  // 변수가 수정되어야 할 경우에는 let mut를 사용하여 compiler에게 이후에 변수가 바뀔 것임을 정의.
  // let mut를 사용한 후 변수를 바꾸지 않으면 warning!
  your_number = 25; 

  // let my_number = "It's me!"; 위와 같은 방식으로 변수를 덮어쓰는 것도 가능.
  // 이를 shadowing이라고 함

  println!("immutable var : {}\n", my_number);
  println!("mutable var : {}\n", your_number);
  println!("---------------------------------\n");
  print_fn();
}