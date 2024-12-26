use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;

struct Level {
    id: usize,
    name: String,
    chances: usize,
}

fn try_guess(secret_number: usize) -> isize {
    let mut guess = String::new();
    
    loop {
        print!("\nEnter your guess: ");
        io::stdout().flush().expect("error 0x03: flush failed");

        io::stdin()
            .read_line(&mut guess)
            .expect("error 0x00: failed to read line");
        
       let guess_num: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue
            },
        };
        
    
        return match guess_num.cmp(&secret_number) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        };
    }
}
    
fn main() {
    let levels = [
        Level { id: 1, name: "Easy".to_string(), chances: 10},
        Level { id: 2, name: "Medium".to_string(), chances: 5},
        Level { id: 3, name: "Hard".to_string(), chances: 3},
    ];

    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have only a few chances to guess the correct number.");

    println!("\nPlease select the difficulty level:");
    println!("\t1. Easy (10 chances)");
    println!("\t2. Medium (5 chances)");
    println!("\t3. Hard (3 chances)");
    
    let level = loop {
        print!("\nEnter your choice (1, 2, 3): ");
        io::stdout().flush().expect("error 0x01: flush failed");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("error 0x02: failed to read input");

        let choice_id: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            },
        };

        if let Some(level) = levels.iter().find(|&lvl| lvl.id == choice_id) {
            break level;
        } else {
            println!("Invalid choice.");
        }
    };

    println!("\nGreat! You have selected the {} difficulty level.", level.name);
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let n = level.chances;

    println!("Let's start the game!");
    //println!("(debug) secret: {}", secret_number);

    let mut i = 0;
    
    while i < n {
         let res = try_guess(secret_number);

         match res {
             0 => {
                  println!("Congratulations! You guessed the number in {} attempts", i + 1);
                  return;
             },
             -1 => println!("Inccorect! The number is less than your guess."),
             1 => println!("Inccorect! The number is greater than your guess."),
             _ => println!("error x04: unreachable error."),
         }
         i += 1;
    }

    println!("\nToo bad! You ran out of chances. The correct number was {}.", secret_number);
}
