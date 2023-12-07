//#[allow(dead_code)]
mod func;

use self::func::*;   // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::deque;

fn main()
{
    print_rust();

    let pal = "rustsur";
    let is_pal = deque::pal_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");
}