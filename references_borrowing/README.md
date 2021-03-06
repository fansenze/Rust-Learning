## 引用与借用
> 代码实战在src/main.rs中  

### 引用  
+ 符号: &  (即来表明参数的类型是一个引用)  
+ 允许使用值, 但是不获取他的所有权  

### 可变引用: 在**特定作用域中**的特定数据**有且只有一个**可变引用, 好处是 Rust 可以在编译时就**避免数据竞争**  

#### 数据竞争 是一种特定类型的竞争状态，它可由这三个行为造成:  
+ 两个或更多指针同时访问同一数据  
+ 至少有一个这样的指针被用来写入数据  
+ 不存在同步数据访问的机制  

> 数据竞争会导致未定义行为, 难以在运行时追踪, 并且难以诊断和修复。Rust 避免了这种情况的发生, 因为它直接拒绝编译存在数据竞争的代码  

#### 可以使用大括号来创建一个新的作用域来允许拥有多个可变引用，只是不能 同时 拥有:
```Rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

#### 我们 **也** 不能在拥有不可变引用的同时拥有可变引用  
```Rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // Error!!! BIG PROBLEM!!!❌
```  

### 悬垂引用  
+ 悬垂指针 是其指向的内存可能已经被分配给其它持有者。  
+ 在 Rust 中编译器确保引用永远也不会变成悬垂状态: 当我们拥有一些数据的引用, 编译器确保数据不会在其引用之前离开作用域。  


## 引用的规则  
1. 在任意给定时间，**只能** 拥有如下中的 **一个**:  
    + 一个可变引用  
    + 任意数量的不可变引用  
2. 引用必须总是有效的