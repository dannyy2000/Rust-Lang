use std::io;

fn main() {

    loop{

        println!("Enter the number!");

        let mut input = String::new();

        // let valid_count;
        // let invalid_count;

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        // let input : u32 = match input.trim().parse(){
        //        Ok(num) => num,
        //        Err(_) => continue
        // };

        let parse_result = input.trim().parse::<u32>();


        // if parse_result {is} Ok(num):
        //   println!("It is a valid input {parse_input} ");
        // else:
        //    println!("Error ");


        //  println!("This is the count for the numbers {valid_count}, {invalid_count} ");


    }
   



}
