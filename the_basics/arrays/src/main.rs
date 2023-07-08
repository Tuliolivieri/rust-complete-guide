fn main() {
    let mut arr: [i32; 3] = [1, 2, 3];
    
    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);

    arr[2] = 10;

    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);
}
