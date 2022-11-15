fn main() {
  // allocation || reallocation
  // 문자열 길이에 맞게 메모리 할당
  // String
    // .capacity
    // .push
    // .push_Str
    // .pop
    // with_capacity

    let mut my_name = String::from("resister_boy");
    let my_other_name = String::with_capacity(50); // capacity 50;
    // 와 같은 방식으로 미리 메모리 크기를 할당할 수 있음

    // string method
    my_name.push('!'); // push char in string.
    my_name.push_str(" and I live in Seoul!"); // push string in string.

    println!("My name is {}", my_name);
    println!("my_name capacity is {}", my_name.capacity());
    println!("my_other_name is {}", my_other_name.capacity());
}