// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    //let x = x;
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

// // Remove a line in the code to make it compile
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // Shadowing and re-binding
//     let x = x; tenes que sacar esta linea. si haces shadowing y no le pones mut da error. todas las variables en rust son inmutables a menos que lo aclares
//     x += 3;

//     let y = 4;
//     // Shadowing
//     let y = "I can also be bound to text!";

//     println!("Success!");
// }
