//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::hash;

fn main()
{
    print_rust();

    let mut hmap = hash::HashMap::new(11);
    hmap.insert(10, "cat");
    hmap.insert(2, "dog");
    hmap.insert(3, "tiger");

    println!("HashMap size {:?}", hmap.len());
    println!("HashMap contains key 2: {}", hmap.contains(2));
    println!("HashMap key 3: {:?}", hmap.get(3));
    println!("HashMap remove key 3: {:?}", hmap.remove(3));
    println!("HashMap remove key 3: {:?}", hmap.remove(3));
}