extern crate phdrs;

use phdrs::objects;

fn main() {
    for o in objects() {
        println!("{:?}", o);
        for p in o.iter_phdrs() {
            println!("   {:?}", p);
        }
    }
}
