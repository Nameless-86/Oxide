fn main() {
    let x = 1; //warning unused, solucion 1 hace un print
    println!("{}", x);
    main_2(); 
}





fn main_2() -> i32 { //main tiene que devolver algo que sea "termination"
    let x = 1; //warning unused, solucion 2 que la funcion devuelva x
    x
}



// fn main() {
//     let _x = 1; //warning unused, solucion 3 haces el prefix _x y no da warnings
//}




// Warning: unused variable: `x`

// fn main() {
//     let x = 1;
// }

// // Warning: unused variable: `x`
