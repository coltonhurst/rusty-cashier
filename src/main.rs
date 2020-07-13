/*
    - get cost of items
    - get cash given
    - return how much change should be given back
*/

use std::io;

#[derive(Debug)]
struct ChangeAmounts {
    pennies: i32,
    nickels: i32,
    dimes: i32,
    quarters: i32,
    bills: i32
}

impl Default for ChangeAmounts {
    fn default () -> ChangeAmounts {
        ChangeAmounts {
            bills: 0,
            quarters: 0,
            dimes: 0,
            nickels: 0,
            pennies: 0
        }
    }
}

fn main() {
    greeting();

    println!("How much does the item cost?");
    let item_cost: f64 = get_float_input();

    println!("How much cash did they give?");
    let cash_given: f64 = get_float_input();

    if item_cost > cash_given {
        println!("Not enough cash was given!");
    } else if item_cost < cash_given {
        let change: ChangeAmounts = calculate_change(item_cost, cash_given);
        println!("{:?}", change);
    } else {
        println!("No change needed!");
    }
}

fn greeting() {
    println!("----- Welcome to Rusty Cashier! -----");
}

fn get_float_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error- failed reading input!");

    return input.trim().parse::<f64>().unwrap();
}

/*
    Assumption: cash_given > item_cost
*/
fn calculate_change(item_cost: f64, cash_given: f64) -> ChangeAmounts {
    let mut processing_change: f64 = cash_given - item_cost;
    let mut change: ChangeAmounts = ChangeAmounts::default();

    // bills
    change.bills = processing_change as i32;
    processing_change = processing_change - (change.bills as f64);

    // quarters
    change.quarters = (processing_change / 0.25) as i32;
    processing_change = processing_change - (change.quarters as f64 * 0.25);

    // dimes
    change.dimes = (processing_change / 0.1) as i32;
    processing_change = processing_change - (change.dimes as f64 * 0.1);

    // nickels
    change.nickels = (processing_change / 0.05) as i32;
    processing_change = processing_change - (change.nickels as f64 * 0.05);
    
    // pennies
    processing_change = processing_change * 100.0;
    change.pennies = processing_change.round() as i32;

    return change;
}
