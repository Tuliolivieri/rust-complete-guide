fn main() {
    let mut nums = vec![1, 2, 3];

    nums.push(4);
    println!("{:?}", nums);

    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new();
    vec.push("test");
    vec.push("str");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);

    let vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);
}
