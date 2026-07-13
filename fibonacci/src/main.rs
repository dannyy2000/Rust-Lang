fn main() {
    fibonacci(6);
}



fn fibonacci(mut number: i32){
    let mut previous_number = 0;
    let mut current_number = 1;
    let mut next_number = 0;

    for i in 1..=number - 2{

        if number == 1{
            break;
        }

        else if number == 2{
            break;
        }
       next_number = previous_number + current_number;
       previous_number = current_number;
       current_number = next_number;
    }

    println!("The fibonnaci number is : {next_number}")
}
