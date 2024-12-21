use std::io; //Prelude...
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game");
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);// i32 type
    //println!("The secret number is: {secret_number}");
    loop{
        println!("Please input a number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // String data
            .expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("That's less!"),
            Ordering::Greater => println!("That's too much!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
    let tup : (u8, i32, f64) = (500, 1, 5.4);
    let arr : [i32; 5] = [1,2,3,4,5];
    let arr =[3;5]; //5 elements with 3s
}
