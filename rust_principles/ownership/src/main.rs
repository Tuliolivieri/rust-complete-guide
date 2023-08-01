fn main() {
    // created on the stack
    let var: i32 = 1;

    // created on the heap
    let mut s: String = "hello".to_string(); 

    // because s is on the heap, the push_str below
    // will be appended on current s, wich is able
    // to grow
    s.push_str(", world!");

    println!("{}", var);
    println!("{}", s);
}

// var is dropped, s is dropped
