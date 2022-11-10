fn main() {
    /*
        int: i8, i16, i32, i64, i128, isize
        unsigned int: u8, u16, u32, u64, u128, usize 
        이때, size는 Computer Architecture의 default bit를 의미.
    */
    let my_number : u8 = 100; // 255;
    let my_other_numebr = 50; // i32: type을 지정하지 않을 경우 default type
    let result = my_number + my_other_numebr;
    println!("result : {}\n", result);
    // 이 경우 rust compiler는 my_other_number를 u8로 정의
    // 만일 연산할 때 type이 다르면 throw error!

    let num_1 = 0___u8;
    let num_2 = 1___000___000___i32;
    println!("num_1 : {} \nnum_2 : {}", num_1, num_2);
}
