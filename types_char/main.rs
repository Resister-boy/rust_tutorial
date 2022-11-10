fn main() {
    /*
        rust에서 char는 ''(single quote), string은 ""(double quote) 사용
        rust에서 char는 메모리 상 4byte을 차지
    */
    let a_char = 'A'; // as u8;
    let b_char = 'B' as u8;

    let my_number : u16 = 100; 
    let my_other_numebr : u8 = 50; 
    let result = my_number + my_other_numebr as u16;
    // 위 경우 as u16을 정의하지 않으면 throw error! => type casting
    // ** casting = simple type change
    // casting은 먼저 정의된 type에 한해 가능

    println!("a_char is : {}", a_char);
    println!("b_char is : {}", b_char);
    println!("Hello, world! My number is {}", result);
}
