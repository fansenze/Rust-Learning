fn main() {
    let mut n = 5;
    if n < 5 {
        println!("n <= 5");
    } else if n == 5 {
        println!("n == 5");
    } else {
        println!("n > 5");
    }

    loop {
        println!("again!");
        break;
    }

    while n > 0 {
        println!("now n is {}", n);
        n -= 1;
    }

    let arr = [11, 22, 33, 44, 55];
    for elem in arr.iter() {
        println!("elem is {}", elem);
    }

    for number in (1..4).rev() {
        // rev用来颠倒数组
        println!("number is {}", number);
    }

    println!("摄氏度 21.5 度 转成 华氏温度 为: {}", to_f(21.5));
    println!("华氏温度 122 度 转成 摄氏度 为: {}", to_c(122.0));

    let n = 15;
    println!("{} 阶斐波那契数列: {}", n, fib(n));

    println!("end");
}
// 华氏温度和摄氏温度转换
fn to_f(n: f64) -> f64 {
    n * 1.8 + 32.0
}

fn to_c(n: f64) -> f64 {
    (n - 32.0) / 1.8
}

// 生成 n 阶斐波那契数列
fn fib(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1
    }
    fib(n - 1) + fib(n - 2)
}
