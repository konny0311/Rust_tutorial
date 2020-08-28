fn main() {
    let mut s = String::new();

    let data = "sample data";
    s = data.to_string();
    println!("test output to_string {}", s);
    s = String::from(s); // this function works same as "to_string()"
    println!("test output from {}", s);
    s.push_str(". this is a pushed sentence.");
    println!("test output from {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let mut addedS = s1 + "-" + &s2;
    println!("added strings: {}", addedS);
    let s3 = String::from("京都");
    addedS = format!("{}-{}", s3, s2);
    println!("another added strings: {}", addedS);
    let id0s = &s3[0..3];
    println!("slice {}", id0s);
    for c in s3.chars() {
        println!("{}", c);
    }
    for b in s3.bytes() {
        println!("{}", b);
    }
}
