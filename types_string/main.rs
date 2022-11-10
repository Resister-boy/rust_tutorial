use std::mem::size_of;

fn main() {
    /*
        rust에서 char은 4bytes이며, 선언 시 ''(single quote)를 사용, string의 경우 선언 시 ""(double quote)를 사용
    */
    let my_name = "resister_boy";
    println!("My name is {}", my_name);
    // println!("Size of a char {}", std::mem::size_of::<char>()); // 4 byte
    println!("Size of a char {}", size_of::<char>()); // 4bytes
    println!("Size of a a {}", "resister_boy".len()); // 12bytes
}
