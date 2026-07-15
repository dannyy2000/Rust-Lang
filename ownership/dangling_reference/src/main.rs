fn main() {
    let _s = non_dangle();
    println!("{_s}");
}



// fn dangle() -> &String{
//     let s = String::from("Hello");

//      &s;
// }


 // the fix

 fn non_dangle() -> String{
    let s = String::from("Hello");

    s
 }
