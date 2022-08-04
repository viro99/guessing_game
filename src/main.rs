use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    //let quit = string::from("q");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    //println!("The secret number is: {}", secret_number);

    let mut count = 0; 
    

    loop {
        count += 1;
        if count == 15 {
            break println!("GAME OVER- YOU LOSE! OUT OF GUESSES.");
        }
        println!("Please input your guess of 1-100. You can only have 15 guesses!");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            //.expect("Please type a number!");

        println!("You guessed: {}, this was guess #{}", guess,count);

            

        match guess.cmp(&secret_number) { 
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"), 
            Ordering::Equal => {
                println!("You win! It took {} guesses to find it!", count);
                break;
            }
        }
    }
}
