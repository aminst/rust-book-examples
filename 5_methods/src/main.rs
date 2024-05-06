struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // borrow ownership so you can still access the rectangle after calling this method
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", r.area());
}
