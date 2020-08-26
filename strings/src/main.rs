fn main() {
    let mut s = String::new();

    let data = "sample data";
    s = data.to_string();
    println!("test output to_string {}", s);
    s = String::from(s); // this function works same as "to_string()"
    println!("test output from {}", s);
    s.push_str(". this is a pushed sentence.");
    println!("test output from {}", s);    
}
