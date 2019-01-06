#[macro_use]
extern crate immutable;

use immutable::*;

fn main() {
    let a = l!(1, 2, 3, 4, 3, 6);
    let b = s!(1, 2, 3, 4, 3, 6);
    let c = v!(1, 2, 3, 4, 3, 6);
    let d = a.rest();
    let e = d.first();
    //let d = h!("a", 2, "b", 4, "c", 6, "d", 9);
    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c);
    println!("e={:?}", e);
}
