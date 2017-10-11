fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
    let x = String::from("hello");
    let y = x;
    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let c1 = 5;
    let c2 = c1;
    println!("c1 = {}, c2 = {}", c1, c2);

    let s3 = String::from("hello");
    let c3 = 99;
    takes_ownership(s3);
    makes_copy(c3);
    // println!("s3 is {}", s3);
    println!("c3 is {}", c3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}