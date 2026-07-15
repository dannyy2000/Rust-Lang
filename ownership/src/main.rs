fn main(){
    // let s = String::from("hello");
    // takes_ownership(&s);
    // println!("{s}");


    // let x = 5;
    // makes_copy(x);

    let mut s  = String::from("Hello");

    let r1 = &mut s;
    println!("{r1}");
    let r2 = &mut s;
     println!("{r2}");

    
}




// fn takes_ownership(some_string: &String){
//     println!("{some_string}");
// }


// fn makes_copy(some_integer:i32){
//     println!("{some_integer}");
// }



// fn main(){

//     let _s1 = gives_ownership();
//     println!("{_s1}");

//     let s2 = String::from("Hello");

//     println!("{s2}");

//     let _s3 = gives_and_take(s2);
//      println!("{_s3}");

// }


// fn gives_ownership() -> String{

//     let some_string = String::from("Myguy");

//     some_string
// }


// fn gives_and_take(give_string: String) -> String{

//     give_string
// }



