fn main() {
    let  s = String::from("Helloword");
    let word = slicing(&s);
    println!("{word}");
}



// fn slicing(s:&String) -> usize {

//     let bytes = s.as_bytes();

//     for(i, &item) in bytes.iter().enumerate(){

//         if item == b' '{
//            return i
//         }
//     }

//     s.len()
// }


fn slicing(s: &String) -> &str{

    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
    }
}

    &s[..]

}