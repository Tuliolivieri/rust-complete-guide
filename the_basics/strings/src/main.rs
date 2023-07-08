fn main() {
    let name = String::from("Tyler");
    let course = "Rust".to_string();
    let new_name = name.replace("Tyler", "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    let str = "hello";
    let str2 = str.to_string();
    let str3 = &str2;

    println!("{}", str);
    println!("{}", str2);
    println!("{}", str3);

    println!("{}", "ONE".to_lowercase() == "one");
    println!("{}", "ONE".to_lowercase() == "onee");

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
