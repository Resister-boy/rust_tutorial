// macro = function that writes code
/*
Rust의 macro는 다른 코드를 작성하는 코드이며, Metaprogramming으로 알려져있기도 함.
macro는 개발자가 작성하고 관리해야 하는 코드의 양을 줄여준다는 점에서 함수와 비슷한 역할.
그러나 함수는 함수가 갖는 파라미터의 개수와 타입을 정의해야 하는 반면, 메크로는 파라미터의 개수를 가변적으로 처리
ex) println!("hello {}", name)

또한 macro는 compiler가 코드를 compile하기 이전에 동작.
때문에 런타임에 호출되는 함수가 할 수 없는 일을 처리할 수 있음. 

macro의 단점은 추상화 계층을 하나 더 만들어낸다는 것. 
때문에 일반적으로 함수에 비해 읽고, 이해하고, 관리하기 어려움.
*/

fn main() {
  let city = "Seoul";
  let year = 2022;
  let population = 9999;
  println!("The city of {my_city} in {this_year} had a population of {the_population}",
    my_city = city,
    this_year = year,
    the_population = population,
    // 위와 같은 방식으로 {} 안에 어떤 값을 넣을 것인지 정의.
);

  println!("The city of {0} in {1} had a population of {2}",
    city,
    year,
    population,
    // 위와 같은 방식으로 {} 안에 숫자 형태로 값을 지정하여 넣을 수 있음.
  );

  println!("{}", 1 + 1); // 최신 rust에서는 println!에서 연산도 가능.

  println!("return : {}", give_integer());
}

// 값을 리턴하는 함수일 경우 return 하는 값의 type을 명시.
// return 값이 함수가 종료되는 중괄호 바로 앞에 있을 경우 값만 적어도 동작.
fn give_integer() -> i32 {
  42
}