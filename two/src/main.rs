fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let name = "OKE";
    let s_name = return_name(name);
    println!("{}",s_name);

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