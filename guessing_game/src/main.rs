//use io library from standard library
use std::io;

fn main() 
{
    println!("Guess the number!");
    println!("Please input your guess.");
    
    //let is how you declare new data types. Mut means mutable - as in it can be changed. Rust is immutable by default
    //the :: means that new() is a function associated with the string class
    let mut guess = String::new();

    //& means you are using the reference of guess. It is immutable by default so you need to use mut again
    io::stdin().read_line(&mut guess)
        //expect handles errors in the read_lines function, returned in io::Result (an enum either Err or Ok)
        .expect("failed to read line");
    
    println!("You guessed {}", guess);
}
