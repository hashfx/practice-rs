/*

/* Adding trait bounds to make it work */
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) {
    y = x;
    let r: &'b &'a i32 = &&0;
}

fn main() {
    println!("Success!")
}

*/

fn f<'a, 'b>(x: &'a i32, mut y: &'b i32)
where
    'a: 'b,
{
    y = x; // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0; // &'b &'a i32 is well formed because 'a: 'b
}
fn main() {
    println!("Success!")
}
