//#[allow(dead_code)]
mod func;

use self::func::*;   // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::stack::base_converter;

fn main()
{
    print_rust();

    let str = base_converter::base_converter(7, 2);
    println!("str: {}", str);
}