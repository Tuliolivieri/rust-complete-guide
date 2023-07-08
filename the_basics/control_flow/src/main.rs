fn main() {
    let one = 1;

    if one > 1 {
        println!("True");
    } else if one == 1 {
        println!("Equal");
    } else {
        println!("False");
    }

    let mut num = 0;

    'counter: loop {
        println!("Count {}", num);
        let mut decrease = 5;

        loop {
            println!("Decreasing: {}", decrease);

            if decrease == 3 {
                break;
            }
            if num == 5 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

    let mut num = 0;

    while num < 5 {
        println!("Num: {}", num);
        num += 1;
    }

    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }

    println!("\nRev...\n");

    for number in (0..10).rev() {
        println!("{}", number);
    }
}
