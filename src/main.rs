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

    let mut list_vec = rust_stl::vec::LVec::new();
    assert_eq!(list_vec.len(), 0);
    assert_eq!(list_vec.is_empty(), true);

    list_vec.push(0);
    list_vec.push(1);
    list_vec.insert(2, 2);
    list_vec.print_lvec();
    println!("----------------");

    let mut val = list_vec.remove(0);
    assert_eq!(val.unwrap(), 0);

    val = list_vec.pop();
    assert_eq!(val, Some(2));

    list_vec.print_lvec();
    println!("----------------");

    list_vec.clear();
    assert_eq!(list_vec.len(), 0);
    assert_eq!(list_vec.is_empty(), true);

    let mut list_vec_other = rust_stl::vec::LVec::new();
    list_vec_other.push(2);

    list_vec.append(&mut list_vec_other);
    list_vec.print_lvec();
    println!("----------------");
}
