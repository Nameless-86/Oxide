// Fix the error with the use of define_x
// fn main() {
//     let x = define_x();
//     println!("{}, world", x);
// }

// fn define_x() -> &'static str {
//     let x = "hello";
//     x
// }

fn main() {
    define_x();
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}

// // Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", x);
// }

// fn define_x() {
//     let x = "hello";
// }
