

fn used_function() {}

//cargo  `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}


// std library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// root mod define
pub mod  c00_variables;

// mod that be used.
use c00_variables::uninitialized as c00;
//

fn main() {

    c00::initialized();

    used_function();
    println!("Hello, world!");

//------------------
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");


    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please type a number!");
                continue;},
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!");break;},
        }        
    }

}
