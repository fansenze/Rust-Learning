fn main() {
    let s1 = String::from("hello");
    let len1 = calculate_length(&s1);
    println!("s1 is {}, len1 is {}", s1, len1);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 is {}", s2);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world!!!");
}
