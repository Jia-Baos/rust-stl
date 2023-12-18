//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::search;

fn main()
{
    print_rust();

    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

    let num = 44;
    let found = search::sequential_search_ordered(&nums, num);
    println!("{num} is in nums: {found}");

    let num = 49;
    let found = search::sequential_search_ordered(&nums, num);
    println!("{num} is in nums: {found}");
}