// const
// static

const HIGH_SCORE: i32 = 30; // const는 uppercase!
// const [변수명]: [타입] = [변수값]
// const는 함수 안에서 사용할 수 없음! Always global
// mut 사용 불가능

static LOW_SCORE: i32 = 0; // static은 uppercase!
// static [변수명]: [타입] = [변수값]
// const와의 차이는 같은 메모리 
// mut 사용 가능, but unsafe => 사용을 지양하고 있음.

// Rust에서 모든 reference type은 lifetime을 가짐.
// lifetime은 reference type의 변수가 존재할 수 있는 scope
// lifetime은 Dangling reference를 방지하는 것.
// Dangling refernce는 프로그램이 개발자가 의도하지 않은 데이터를 참조하는 것의 원인

fn main() {
  let score = 10; // let binding : 숫자를 변수에 바인딩! type define X
  println!("My average is {score}!!");
  println!("My worst is {LOW_SCORE}!!");
  println!("My best is {HIGH_SCORE}!!");
}