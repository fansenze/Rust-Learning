## 控制流
> 代码实战在src/main.rs中  

### if 表达式  
```Rust
fn main() {
  let n = 5;
  if n > 5 {
    println!("n > 5");
  } else if n == 5 {
    println!("n == 5");
  } else {
    println!("n < 5");
  }
}
```  
- if 后的条件 必须是 **bool** 类型, 不然直接抛出错误, 因为rust不会隐式地尝试转换条件为 bool, 所以必须显式地使用 bool  

### 循环  

> 三种类型:  
+ loop  
+ while  
+ for  

> 关键字: break, 来告诉程序何时退出循环  
> 虽然可以用 while 来遍历数组, 但是由于数组长度不正确会导致程序 panic, 因此数组循环建议用更有效率的 for 来替代


### 本章节实践  

+ 华氏温度和摄氏温度转换  
+ 生成 n 阶斐波那契数列  
+ 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分 (编写循环)   

```Rust
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

```