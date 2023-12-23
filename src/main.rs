//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::tree;

fn main()
{
    print_rust();

    let mut bst = tree::BST::<i32, char>::new();
    bst.insert(8, 'e');
    bst.insert(6, 'c');
    bst.insert(7, 'd');
    bst.insert(5, 'b');
    bst.insert(10, 'g');
    bst.insert(9, 'f');
    bst.insert(11, 'h');
    bst.insert(4, 'a');

    println!("empty: {:?}, len: {:?}", bst.is_empty(), bst.len());
    println!("max: {:?}, min: {:?}", bst.max(), bst.min());
    println!("key: 5, val: {:?}", bst.get(&5));
    println!("5 in bst: {:?}", bst.search(&5));

    println!("inorder: ");
    bst.inorder();
    println!("preorder: ");
    bst.preorder();
    println!("postorder: ");
    bst.postorder();
}