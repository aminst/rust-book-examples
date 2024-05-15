use std::{collections::HashMap, hash::Hash};

fn own_single_element_from_vector(v_el: String) {
    println!("The value of v is: {}", v_el);
    // I want to test if I can drop a single part of the vector here
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    let str_v = vec![String::from("hello"), String::from("world")];
    // own_single_element_from_vector(str_v[0]);
    // The previous line gives an error since str_v[0] returns a reference to the value in the vector
    // So, the value is still owned by the vector
    // You cannot move a borrowed value
    // Makes sense since you don't want to just drop a value from a vector without the vector knowing
    let mut hash_map: HashMap<String, i32> = HashMap::new();
    hash_map.insert(String::from("one"), 1);
    println!("{:?}", hash_map);
}
