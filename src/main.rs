//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::tree;

fn main()
{
    print_rust();

    let mut avl = tree::AvlTree::new();
    for i in 0..10 {
    let (_r1, _r2) = avl.insert(i);
    }
    println!("empty: {}", avl.is_empty());
    println!("length: {}", avl.len());
    println!("depth: {}", avl.depth());
    println!("9 in avl: {}", avl.search(&9));
}