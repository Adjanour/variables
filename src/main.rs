fn main() {
    let  x = 5;
    let x = x + 1;
    // x = 6 reassignment
    // let x = 7 shadowing
    {
        // inner shadowing
        let x = x + 1;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
