fn main() {
    // parse解析时，必须声明变量类型，以及带上expect错误提示
    let z1: i32 = "-42".parse().expect("Not a number!");
    let z2: i32 = "42".parse().expect("Not a number!");
    let z3: u32 = "42".parse().expect("Not a number!");
    println!("整型: {}, {}, {}", z1, z2, z3);

    let f1 = 1.0;
    let f11: f64 = "-1.1".parse().expect("Not a number!");
    let f2: f32 = 2.0;
    let f3: f32 = -3.1;
    println!("浮点型: {}, {}, {}, {}", f1, f11, f2, f3);

    let x1 = 5 + 3;
    let x2 = 10.5 - 4.4;
    let x3 = 15 * 5;
    let x4 = 25.5 / 6.5;
    let x5 = 30 % 7;
    let x6 = 5 >> 1;
    println!("表达式计算的值: {}, {}, {}, {}, {}, {}", x1, x2, x3, x4, x5, x6);

    let b1 = true;
    let b2: bool = false;
    println!("布尔型: {}, {}", b1, b2);

    let s1 = 's';
    let s2 = '🙈';
    let s3: &str= "好";
    println!("字符型: {}, {}, {}", s1, s2, s3);

    let tup: (i32, f64, &str) = (500, 6.4, "sdda");
    let (x, y, z) = tup;
    println!("复合类型 元组: {}, {}, {}", x, y, z);
    println!("元组中使用点号+索引值访问变量: {}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 44, 24, 55];
    println!("复合类型 数组前第2和第5个元素: {}, {}", arr[1], arr[4]);
}
