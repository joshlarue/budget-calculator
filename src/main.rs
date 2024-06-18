use std::num::ParseFloatError;
fn main() {
    // gets input from get_monetary_amt(). Panics if input is invalid
    // TODO: loop on invalid input
    let total_monetary_amt = match get_monetary_amt() {
        Ok(num) => num,
        Err(err) => panic!("Invalid input! {}", err),
    };

    calculate_to_save_long_term(total_monetary_amt, 0.5);
    calculate_to_save_medium_term(total_monetary_amt, 0.3);
    calculate_to_spend_short_term(total_monetary_amt, 0.2);
}

// gets CLI input from user
fn get_monetary_amt() -> Result<f32, ParseFloatError> {
    let total_monetary_amt = std::env::args()
        .nth(1)
        .expect("No monetary amount given")
        .parse::<f32>()?;
    println!("Monetary amt: ${}", total_monetary_amt.to_string());
    Ok(total_monetary_amt)
}

// functions to calculate different saving types
fn calculate_to_save_long_term(total_monetary_amt: f32, percent_to_save: f32) {
    let total_to_save = total_monetary_amt * percent_to_save;
    println!("${}: savings account", total_to_save);
}

fn calculate_to_save_medium_term(total_monetary_amt: f32, percent_to_save: f32) {
    let total_to_save = total_monetary_amt * percent_to_save;
    println!("${}: medium-term savings account", total_to_save);
}

fn calculate_to_spend_short_term(total_monetary_amt: f32, percent_to_save: f32) {
    let total_to_spend = total_monetary_amt * percent_to_save;
    println!("${}: short-term spending account", total_to_spend);
}
