//use io library from standard library
use std::io;
//adding rng support from the rand library (dependency listed in Cargo.toml)
use rand::Rng;
use std::cmp::Ordering;
fn main() 
{
    println!("Guess the number!");


    let secret_number = rand::thread_rng().gen_range(1,101);

    //println!("The secret number is {}", secret_number);

    //loop through the program
    loop
    {
        println!("Please input your guess.");
    
        //let is how you declare new data types. Mut means mutable - as in it can be changed. Rust is immutable by default
        //the :: means that new() is a function associated with the string class
        let mut guess = String::new();

        //& means you are using the reference of guess. It is immutable by default so you need to use mut again
        io::stdin().read_line(&mut guess)
            //expect handles errors in the read_lines function, returned in io::Result (an enum either Err or Ok)
            .expect("failed to read line");
    
        //make a shadow variable of guess from string to a 32-bit unsigned int (u32)
        //trim() eliminates any white spaces at the beginning and the end of the string. It gets rid of the \n character at end
        //parse() turns the string into a u32
        //matching to handle if a non-int was entered
        let guess: u32 = match guess.trim().parse()
        {
            //handle non-ints that have been entered
            Ok(num) => num,
            //underscore is a catch-all value (catches all the Err values)
            //continue makes program skip to the next iteration of the loop
            Err(_) => continue,            
        };
            

        println!("You guessed {}", guess);

        //match is kinda like switch - check the ordering of guess & secret number
        match guess.cmp(&secret_number)
        {
            //ordering is enum with Less, Greater, and Equal values. cmp will compare the value guess with 
            //secret_number and return the enum ordering based on if it's less, greater or equal to the secret number
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => 
            {
                println!("You win!");
                break;
            }
        }
    }
    
}
