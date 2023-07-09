fn concat_string(str: &str) -> String {
    let world: &str = " World";

    let mut owned: String = str.to_owned();
    owned.push_str(world);

    owned
}

fn control_flow(value: u8) -> String {
    if value == 1 {
        "The value is one".to_string()
    } else if value < 25 {
        "The value is less than 25".to_string()
    } else if value < 50 {
        "The value is greater than 25 but less than 50".to_string()
    } else {
        "The value is greater than 50".to_string()
    }
}

fn main() {
    let val1 = 5;
    let val2 = 2;

    let ans = val1 % val2;

    println!("{}", ans);

    let mut vec = vec![2, 4, 6, 8, 10];

    println!("{:?}", vec);

    vec.pop();
    vec.push(12);

    println!("{:?}", vec);

    println!("{}", concat_string("Hello"));

    println!("{}", control_flow(1));
    println!("{}", control_flow(24));
    println!("{}", control_flow(26));
    println!("{}", control_flow(49));
    println!("{}", control_flow(50));
}
