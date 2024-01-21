//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::demo;

fn main()
{
    print_rust();

    let mut trie = demo::Trie::new();
    trie.insert("box");
    trie.insert("insert");
    trie.insert("apple");
    trie.insert("appeal");

    let res1 = trie.search("apple");
    let res2 = trie.search("apples");
    let res3 = trie.start_with("ins");
    let res4 = trie.start_with("ina");
    println!("word 'apple' in Trie: {res1}");
    println!("word 'apples' in Trie: {res2}");
    println!("prefix 'ins' in Trie: {res3}");
    println!("prefix 'ina' in Trie: {res4}");
}