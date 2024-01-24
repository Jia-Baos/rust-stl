//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::demo;
use rust_stl::demo::Encoder;
use rust_stl::demo::Decoder;

fn main()
{
    print_rust();

    println!("{:#?}", "abc".encode_to_base58());
    println!("{:#?}", "ZiCa".decode_from_base58().unwrap());

    println!("{:#?}", "我爱你".encode_to_base58());
    println!("{:#?}", "3wCHf2LRNuMmh".decode_from_base58());

    println!("{:#?}", "我愛你".encode_to_base58());
    println!("{:#?}", "3wCHf1q5U5pUP".decode_from_base58());
}