use std::io;

fn main() {

    println!("Odds to probability converter!");

    let mut odds : f32 = 0.;
    let mut odds_vec: Vec<f32> = Vec::new();
    loop {
        println!("Enter odd value (decimal) or 'no' to stop adding odds");
        let mut odds_input = String::new();
        io::stdin()
            .read_line(&mut odds_input)
            .expect("Failed to read input");

        if odds_vec.len() >= 1 && odds_input.trim().eq("no") {
            break
        }

        odds = match odds_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue
            }
        };


        println!("Entered odds {}", odds);
        odds_vec.push(odds);
    }

    let mut probability_sum : f32 = 0.;
    for (index, odds) in odds_vec.iter().enumerate() {
        let probability: f32;
        // let odds_compared_to_one: f32 = 1. / odds;
        // probability = odds_compared_to_one / (1. + odds_compared_to_one);
        probability = 1. / odds;
        probability_sum += probability;

        println!("The probability of the event {}. happening is : {}", index, probability);
    }

    if odds_vec.len() > 1 {
        println!("The profit margin is : {}, probability of any of the events happening is {}", probability_sum - 1., probability_sum);
    }
}
