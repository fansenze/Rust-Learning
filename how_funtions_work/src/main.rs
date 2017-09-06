fn main() {
    println!("Hello Rust!");
    let x = 5;
    let y = {
        let v = 2;
        v + 1
    };
    func(-6, true, "你好~");
    println!("c is {}", func2(x, y));
}

fn func(x: i32, y: bool, z: &str) {
    println!("params is {}, {}, {}", x, y, z);
}

fn func2(x: i32, y: i32) -> i32 {
    println!("x is {}, y is {}", x, y);
    // x + y
    return x + y;
}