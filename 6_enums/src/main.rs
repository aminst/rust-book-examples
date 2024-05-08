// use core::fmt;

// enum Color {
//     Red,
//     Green,
//     Blue,
// }

// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Color::Red => write!(f, "Red"),
//             Color::Green => write!(f, "Green"),
//             Color::Blue => write!(f, "Blue"),
//         }
//     }
// }

// fn main() {
//     let red = Color::Red;
//     println!("{}", red);
// }

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = match y {
        Some(y) => x + y,
        None => panic!("No value present"),
    };
    println!("Sum: {}", sum);
}
