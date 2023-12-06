//#[allow(dead_code)]

use rust_stl::stack::stack;
use rust_stl::stack::par_checker1;

fn main()
{
    let mut s = stack::Stack::new();
    assert_eq!(s.is_empty(), true);
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.size(), 3);

    println!("top {:?}, size {}", s.peek().unwrap(), s.size());
    println!("pop {:?}, size {}", s.pop().unwrap(), s.size());
    println!("is_empty:{}, stack:{:?}", s.is_empty(), s);

    println!("---------------");

    let sa = "()(())";
    let sb = "()((()";
    let res1 = par_checker1::par_checker(sa);
    let res2 = par_checker1::par_checker(sb);
    println!("sa balanced: {}",res1);
    println!("sb balanced: {}",res2);

}