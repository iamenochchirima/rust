// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;
use uuid::Uuid;



struct Bill {
    id: String,
    name: String,
    amount: f64,
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Error reading buffer, please try again")
    }
    let input = buffer.trim().to_owned();
    if input == "" {
        println!("Input cannot be empty, please try again");
        return None;
    } else {
        return Some(input);
    }
}

enum MainMenu {
    AddBill,
    ViewBills,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("== Bill Manager ==");
        println!("");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("");
        println!("Please choose an option:");

    }
}



fn main() {
    
    loop {
        MainMenu::show();
        let input = get_input().expect("No data entered");
        match MainMenu::from_str(&input.as_str()) {
            Some(MainMenu::AddBill) => {
                println!("Add Bill");
            },
            Some(MainMenu::ViewBills) => {
                println!("View Bills");
            },
            None => {
                println!("Invalid option");
                return;
            },
        }
    }
}
