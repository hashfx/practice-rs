/*

fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!(__);
    assert_eq!(s, "hello, world!");
}

*/

fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}
