fn main() {
    
    // let number = 3;

    // if number < 5 {
    //    println!("The number is true");
    // }
    // else{
    //    println!("The number is false");
    // }

   execute_condition();


   
}


fn execute_condition() -> i32 {
    let condition = true;

    let number = if condition {5} else {6};

     println!("The value of number is: {number}");

     number

}



  
