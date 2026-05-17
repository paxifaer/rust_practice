// 1. 创建 String s1 = "hello"
// 2. 把它赋值给 s2
// 3. 打印 s2
// 4. （故意尝试打印 s1）

// fn str_clone()
// {
//   let s1 = String::from("hello");
//   let s2 = s1.clone();
//   println!("s1 is {}, s2 is{}", s1,s2); 
// }

// fn str_withoutclone()
// {
//   let s1 = String::from("hello");
//   let s2 = s1;
//   println!(" s2 is{}", s2); 
// //   println!(" s1 is{}", s1); //error
// }

// fn main()
// {
//    str_clone();
//    str_withoutclone();
// }

fn take_ownership(some_string: String)
{
  println!("some_string is {}", some_string);
}

fn make_copy(some_integrater: i32)
{
  println!("some var is {}", some_integrater);
}

fn ownership1()
{
  let s = String::from("hello, world!");
  take_ownership(s);
  let x : &str = "test";
  take_ownership(x.to_string());//创建新的，ele然后move ownership
  let y = 5;
  make_copy(y);
}

fn give_ownership() -> String
{
  let x = String::from("hello_world");
  x
}

fn take_and_giveback(some_string : String) -> String
{
  some_string
}

fn ownership2()
{
  let str = give_ownership();
  println!("get ownership is {}", str);
  let s2 = take_and_giveback(str);
}

fn borrow_value1(value:&String) 
{
  print!(" value is {}", value); 
}

fn borrow_value2(value: &mut String)
{
  *value = String::from("value2");
   print!(" value is {}", value);
}

// fn borrow_value3()
// {

// }

// fn borrow_value4()
// {

// }

fn borrow_nomut()
{
  let value = String::from("value1");

  borrow_value1(&value);
  print!("after borrow is {}", value);

}

fn borrow_mut()
{
  let mut value = String::from("value1");

  borrow_value2(&mut value);
  print!("after borrow is {}", value);
}


fn main()
{
  ownership1();
  ownership2();
  borrow_nomut();
  borrow_mut();
}