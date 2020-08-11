fn main() {
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("value of result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("fin");

    let array = [10, 20, 30, 40, 50];
    for ele in array.iter() {
        println!("value is {}", ele);
    }

    for num in (1..4).rev() {
        println!("value of num is {}", num);
    }
}
