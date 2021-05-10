use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    
    //println!("The nb is :{}", secret_number);
    
    loop {
        println!("input the number:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");
        
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("you guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("yess");
                break;
            }
            Ordering::Greater => println!("too big"),
        };
    }
}