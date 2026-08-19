fn main() {
    let  x = 5;
    let x = x + 1;

    println!("Value of x is: {x}");

    {
        let x = x * 2;
        println!("New value of x is: {x}");
    }

    println!("Value of x is again: {x}");
}
