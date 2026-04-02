fn main() {
    let x = 5;
    let x = 6;


    {
        let x = x *2;
        println!("This is the value of x in the inner block: , {x}");
    }

    println!("This is the value of x: {x}");
}
