use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_rank(nb_try: usize) -> String {
    if nb_try == 1 {
        return String::from("🙏 Nostradamus 🙏");
    } else if nb_try < 5 {
        return String::from("a ✨ Geopardy All Star ✨");
    } else if nb_try < 10 {
        return String::from("a dichotomic bot 🤖");
    } else {
        return String::from("a big noob 😛");
    }
}

fn main() {
    println!("=======================");
    println!("=                     =");
    println!("=  Guess the number!  =");
    println!("=                     =");
    println!("=======================");

    let secret_number = rand::thread_rng().gen_range::<i32>(1, 100);
    let mut previous_guesses: Vec<i32> = Vec::new();

    loop {
        println!("\nPlease input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("An error occured while reading your input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let has_matching_previous_guess: Vec<&i32> = previous_guesses
            .iter()
            .filter(|previous_guess| **previous_guess == guess)
            .collect();

        if has_matching_previous_guess.len() != 0 {
            println!("You've already tried {}!", guess);
        } else {
            previous_guesses.push(guess);
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small! 📉"),
            Ordering::Greater => print!("🍔 Too big! 🍔"),
            Ordering::Equal => {
                println!("👏 You won! ^^ 👏");
                println!("🔩 Total number of try: {} ...", previous_guesses.len());
                println!("... you are ...");
                println!("{}", get_rank(previous_guesses.len()));
                break;
            }
        }
    }
}
