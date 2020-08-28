use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let mut another_scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", another_scores);

    let blue_team = String::from("Blue");
    let blue_score = scores.get(&blue_team);
    match blue_score {
        Some(v) => println!("Score of {} is {:?}", blue_team, v),
        None => println!("Score of {} does not exist", blue_team),
    }

    for (k, v) in &scores {
        println!("{}:{}", k, v);
    }
    scores.insert(String::from("Blue"), 30); // overwrite
    println!("{:?}", scores);

    // insert key-value if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world hello";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        // The or_insert method actually returns a mutable reference (&mut V) to the value for this key
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", text_map);
}
