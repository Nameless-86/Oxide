fn main()
{
    let x: i32 = 5;
    let y: i32; //uninitialized and unused, compile warning

    assert_eq!(x, 5);
    println!("Correct!!");
}
