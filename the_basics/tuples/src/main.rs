fn main() {
    let tup: (i32, &str, bool) = (500, "str", true);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let (x, y, z): (i32, &str, bool) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}
