fn main() {
    let num: i32 = "-42".parse().expect("Not a number!");
    let num2: i32 = "42".parse().expect("Not a number!");
    println!("{}, {}", num, num2);
}
