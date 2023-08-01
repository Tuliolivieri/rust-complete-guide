fn main() {
    let x = vec!["Tyler".to_string()];
    let y = x;
    // next line print throws an error
    // because x no more had his value
    // after move him to y
    // println!("{:?}", x);
    println!("{:?}", y);
}
