//#[allow(dead_code)]


mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::dp;

fn main()
{
    print_rust();

    // cashes 保存各种面额的纸币
    let cashes = [1, 5, 10, 20, 50];
    let amount = 31u32;
    let cashes_num = dp::num_coins1(&cashes, amount);
    println!("need refund {cashes_num} cashes");
}