/*

fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}

*/

fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}", x, y);
}

fn main() {
    let x = 50;
    let y = x;
    println!("{},{}",x,y);
}

fn main() {
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}

fn main() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}


