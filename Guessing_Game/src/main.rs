use std::io; //lib of input and output user
use rand::Rng; //lib of random number
use std::cmp::Ordering; //variables to compare nums

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();  //create var for store input of user
    
        io::stdin()
            .read_line(&mut guess) //metod identificator pattern for get input of user, & is reference 
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //convert to string for number, u32 is a number int 
    
        println!("You guessed: {guess}");
    
        //match check ordering for ordering with base in guess and secret number, exemple (for)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

