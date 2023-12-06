//#[allow(dead_code)]
mod func;

use self::func::*;   // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::stack;


fn main()
{
    print_rust();

    let infix = "( A + B ) * ( C + D )";
    let postfix = stack::infix_to_postfix(infix);
    match postfix {
        Some(val) => {
            println!("infix: {infix} -> postfix: {val}");
        }
        None => {
            println!("{infix} is not a correct infix string");
        }
    }
}