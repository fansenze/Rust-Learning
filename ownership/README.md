## 认识所有权
> 代码实战在src/main.rs中  


所有语言的内存管理方式:   
1. GC
2. 亲自分配和释放内存
3. 内存被一个所有权系统管理, 它拥有一系列的规则使编译器在编译时进行检查。任何所有权系统的功能都不会导致运行时开销 (Rust)  
### 栈 (Stack) 与 堆 (Heap)  
> 栈和堆都是代码在运行时可供使用的部分内存, 不过他们以不同的结构组成  

#### 栈  

+ 栈以放入值的顺序存储并以相反顺序取出值。这也被称作 **后进先出**, 增加数据叫做 **进栈**, 而移出数据叫做 **出栈**  

```
初始    进栈   进栈    出栈  出栈
              --3--
       --2--  --2--  --2--
--1--  --1--  --1--  --1-- --1--
```

操作栈是非常快的:  
+ 因为它访问数据的方式: 永远也不需要寻找一个位置放入新数据或者取出数据 因为这个位置总是在 **栈顶**    
+ 栈中的 **所有数据** 都必须是一个 **已知的固定的大小**   

#### 堆
> 相反对于在编译时未知大小或大小可能变化的数据, 可以把他们储存在堆上  
> 堆是缺乏组织的：当向堆放入数据时, 我们请求一定大小的空间。操作系统在堆的某处找到一块足够大的空位, 把它标记为已使用, 并返回给我们一个它位置的指针。这个过程称作 在堆上分配内存（allocating on the heap）, 并且有时这个过程就简称为 “分配”（allocating）。向栈中放入数据并不被认为是分配。因为指针是已知的固定大小的, 我们可以将指针储存在栈上, 不过当需要实际数据时, 必须访问指针  
+ 访问堆上的数据要比访问栈上的数据要慢因为必须通过指针来访问。现代的处理器在内存中跳转越少就越快 (缓存) 
+ 在堆上分配大量的空间也可能消耗时间  


### 所有权规则  
+ 每一个值都被它的 **所有者 (owner)** 变量拥有  
+ 值在任意时刻只能被一个所有者拥有  
+ 当所有者离开作用域, 这个值将被丢弃  

### 变量作用域  
+ 变量从声明的点开始直到当前 **作用域** 结束时都是有效的  

### `String` 类型
+ 这个类型储存在 **堆** 上所以能够储存在编译时未知大小的文本
+ 可以用 `from` 从字符串字面值来创建 `String`  
+ 这类字符串 可以 被修改, 为什么 `String` 可变而字面值却不行呢? 区别在于两个类型对内存的处理上  

```Rust
let hello = String::from("hello");

let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!
```  

### 内存与分配  
> 对于字符串字面值的情况, 我们在编译时就知道内容所以它直接被硬编码进最终的可执行文件中, 这使得字符串字面值快速和高效。不过这些属性都只来源于其不可变性。不幸的是, 我们不能为了每一个在编译时未知大小的文本而将一块内存放入二进制文件中而它的大小还可能随着程序运行而改变  

+ Rust 采取的策略：内存在拥有它的变量离开作用域后就被自动释放  

```Rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is nolonger valid

// 当 s 离开作用域的时候。当变量离开作用域，Rust 为其调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop
```  

#### 变量与数据交互的方式（一）: 移动  

```Rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```  
以上代码会 panic, 因为这里的 s1 是分配在**堆**上的内存, 当执行到赋值给 s2 的语句时, 相当于两个相同的指针, Rust 会使 s1 的**引用无效化** (因为离开作用域时, Rust 会自动调用 drop 进行内存释放, 如果**多次释放相同的内存**, 会导致内存污染 以及 潜在的安全漏洞) 。因此, 这个操作可以被称为 **移动**

```Rust
let c1 = 5;
let c2 = c1;
println!("c1 = {}, c2 = {}", c1, c2);
```
这里不会报错, 知道为什么吗? 原因就是该数据类型在编译时是分配在 **栈** 上 还是 **堆** 上

> Rust 永远也不会自动创建数据的 “深拷贝”。因此, 任何 自动 的复制可以被认为对运行时性能影响较小  

#### 变量与数据交互的方式（二）: 克隆  
+ 为了达到深度复制栈堆上数据的目的, 可以使用 clone 的通用函数  
```Rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```  

### 所有权与函数
+ 将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会 **移动** 或者 **复制**, 就像赋值语句一样  

### 返回值与作用域

#### 变量的所有权总是遵循相同的模式: 将值赋值给另一个变量时移动它。当持有堆中数据值的变量离开作用域时, 其值将通过 `drop` 被清理掉, 除非数据被移动为另一个变量所有