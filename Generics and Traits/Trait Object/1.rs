/*

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// fix the errors in main
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
    check_size([(); 31].map(|_| "hello你好".to_string()));  // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
    check_size(['中'; 191]); // A char takes 4 bytes in Rust
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

*/

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main() {
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // this bird has forgotten how to swim, so below line will cause an error
    // bird.swim();
    // but it can quak
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // this bird has forgotten how to fly, so below line will cause an error
    // bird.fly();
    // but it can quak too
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!")
}

fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    if species == 1 {
        Box::new(Swan {})
    } else {
        Box::new(Duck {})
    }
}
