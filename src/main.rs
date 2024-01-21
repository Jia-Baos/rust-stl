//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::demo;

fn main()
{
    print_rust();

    let mut bf = demo::BloomFilter::new(100, 0.08);
    (0..20).for_each(|i| bf.insert(&i));
    let res1 = bf.contains(&2);
    let res2 = bf.contains(&50);
    println!("2 in bf: {res1}, 50 in bf: {res2}");
}