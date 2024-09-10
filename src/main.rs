//#[allow(dead_code)]
mod func;

// use self::func::*; // 采用self
// use crate::func::*; // 采用crate
// use rust_stl::sort::bubble_sort::bubble_sort1;

fn main() {
    rust_stl::func::print_rust();

    let mut vec: [i32; 10] = [1, 3, 8, 6, 4, 2, 7, 5, 9, 0];
    rust_stl::sort::bubble_sort1(&mut vec);

    for element in vec {
        print!("{element}, ");
    }
    println!();

    let mut binary_tree = rust_stl::tree::BinaryTree::new(1);
    binary_tree.insert_left_tree(0);
    binary_tree.insert_right_tree(2);
    let root_value = binary_tree.get_key();
    let left_value = binary_tree.get_left().unwrap().get_key();
    let right_value = binary_tree.get_right().unwrap().get_key();
    println!("root_value: {root_value}");
    println!("left_value: {left_value}");
    println!("right_value: {right_value}");

    binary_tree.set_key(3);
    let value_parent_new = binary_tree.get_key();
    println!("value_parent_new: {value_parent_new}");

    println!("------pretorder------");
    binary_tree.preorder();

    println!("------inorder------");
    binary_tree.inorder();

    println!("------postorder------");
    binary_tree.postorder();

    println!("------pretorder------");
    rust_stl::tree::BinaryTree::preorder(&binary_tree);

    println!("------inorder------");
    rust_stl::tree::BinaryTree::inorder(&binary_tree);

    println!("------postorder------");
    rust_stl::tree::BinaryTree::postorder(&binary_tree);
}
