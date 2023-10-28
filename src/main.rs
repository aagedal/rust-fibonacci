use std::io;
mod fib;

fn main() {

    println!("Starting program.");

    //Main loop starts here. It gets an input from the user and checks if it is an integer.
    loop {

        println!("Enter your number here:");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input);

        let input_trim: &str = input.trim();

        if input_trim == "exit" || input_trim == "c" {
            break;
        }

        let val: i32 = input.trim().parse().unwrap_or(-2);

        if val == -2 {
            println!("Not a valid number. Try again.");

            continue;
        }

        println!("Your number is: {}", val);

        fib::fib(val);

        println!(" ");

        println!("Try another number or enter 'exit' and press enter to end the program.");
    
    }

}

