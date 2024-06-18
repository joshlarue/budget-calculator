use std::num::ParseIntError;
fn main() {
    let monetary_amt = match get_monetary_amt() {
        Ok(num) => num,
        Err(err) => panic!("Not a number! {}", err),
    };
    println!("{}", monetary_amt.to_string());
}

fn get_monetary_amt() -> Result<i32, ParseIntError> {
    let total_monetary_amt = std::env::args()
        .nth(1)
        .expect("No monetary amount given")
        .parse::<i32>()?;
    println!("Monetary amt: ${}", total_monetary_amt.to_string());
    Ok(total_monetary_amt)
}

//fn calculate_to_save() {}
