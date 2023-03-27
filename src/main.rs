use std::io;

fn main() {

    println!("Odds to probability converter!");

    let odds : f32;
    loop {
        println!("Enter odd value (decimal)");
        let mut odds_input = String::new();
        io::stdin()
            .read_line(&mut odds_input)
            .expect("Failed to read input");

        odds = match odds_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue
            }
        };

        println!("Odds are {}", odds);
        break;
    }

    let probability: f32;
    let odds_compared_to_one: f32 = 1. / odds;
    probability = odds_compared_to_one / (1. + odds_compared_to_one);

    println!("The probability of the event happening is : {}", probability);
}
