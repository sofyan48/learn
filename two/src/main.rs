fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let name = "OKE";
    let s_name = return_name(name);
    println!("{}",s_name);

    let gives = gives_ownership();
    println!("{}", gives);

    let take_gives = takes_gives_ownership(gives);
    println!("{}", take_gives);

    let clen = String::from("hello");
    let (result, len) = calculate_length(clen);
    println!("The length of '{}' is {}.", result, len);

} 

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} 

fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
}

fn return_name(some_nname: &str) -> &str {
    return some_nname;
}


fn gives_ownership() -> String { 
    let some_string = String::from("iank");
    some_string 
}

fn takes_gives_ownership(some_string: String) -> String { 
    some_string 
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}