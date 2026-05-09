// 1. 定义一个可变变量 x = 10
// 2. 把它加 5
// 3. 写一个函数 add_one(n:i32) -> i32
// 4. 调用这个函数打印结果

fn add_one(mut n:i32) -> i32
{
  n += 1;
  n
}

fn main()
{
    let mut x = 10;
    x += 5;
    println!("{}", add_one(x));
}