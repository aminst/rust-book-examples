fn move_ownership(s: String) {
    println!("{}", s);
}

fn borrow_ownership(s: &String) {
    println!("{}", s);
}

fn main() {
    // let mut s: &str = "Hello, world!";
    // s = "Hello, world!";

    // let mut s = String::from("Hello, world!");
    // println!("{}", s);
    // s = "Hello, world!".to_string();
    // println!("{}", s);
    let s = String::from("Hello, world!");
    move_ownership(s);
    // println!("{}", s); // error because the ownership has been moved

    let s = String::from("Hello, world!");
    borrow_ownership(&s);
    println!("{}", s); // no problem because the ownership has been borrowed
}
