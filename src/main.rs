fn main() {
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y is {}", y);
    }
    println!("y is {}", y);

    let mut some_strings = "aaa";
    println!("strings is {}", some_strings);

    let some_strings = some_strings.len();
    println!("string len is {}", some_strings);
}
