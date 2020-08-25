fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("The number is {}", &v[3]);

    match v.get(3) {
        Some(third) => println!("The number is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("iterating values in a Vector {}", i);
    }
}
