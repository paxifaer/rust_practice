// 1. 创建 String s1 = "hello"
// 2. 把它赋值给 s2
// 3. 打印 s2
// 4. （故意尝试打印 s1）

fn str_clone()
{
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 is {}, s2 is{}", s1,s2); 
}

fn str_withoutclone()
{
  let s1 = String::from("hello");
  let s2 = s1;
  println!(" s2 is{}", s2); 
//   println!(" s1 is{}", s1); //error
}

fn main()
{
   str_clone();
   str_withoutclone();
}