/*

fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let __ r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

*/

fn main() {
    let c = '中';

    let r1 = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}