#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny matched");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:?}", state);
            25
        },
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(coin);
    println!("value of the coin is {}", value);

    let some_value = Some(5);
    if let Some(3) = some_value {
        println!("some value matched!");
    } else {
        println!("some value not matched");
    }
}
