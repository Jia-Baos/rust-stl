//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::sort;

fn main()
{
    print_rust();

    let mut nums = [1,2,8,3,4,9,5,6,7];
    sort::cbic_sort2(&mut nums);
    println!("sorted nums: {:?}", nums);
}