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

use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    bills: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { bills: HashMap::new() }
    }

    fn add(&mut self, bill: Bill) {
        self.bills.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.bills.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.bills.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.bills.get_mut(name) {
            Some(b) => {
                b.amount = amount;
                true
            }
            None => false,
        }
    }
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

fn get_bill_amount() -> Option<f64> {
    println!("Bill amount:");

    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            println!("Input cannot be empty, please try again");
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => {
                println!("Invalid amount, please try again");
            }
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBills,
    RemoveBill,
    EditBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::EditBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("== Bill Manager ==");
        println!("");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("");
        println!("Please choose an option:");
    }
}

mod menu {
    use crate::{get_input , get_bill_amount, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");
        let name = match get_input() {
            Some(name) => name,
            None => {
                println!("Invalid name");
                return;
            }
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => {
                println!("Invalid amount");
                return;
            }
        };
        let bill = Bill {
            name,
            amount,
        };
        bills.add(bill);
        println!("Bill added");
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("Bill name-{}: Ammount-${}", bill.name, bill.amount);
        }
        println!("Enter the name of the bill you want to remove: ");
        let name = match get_input() {
            Some(name) => name,
            None => {
                println!("Invalid name");
                return;
            }
        };
        if bills.remove(&name) {
            println!("Bill removed");
        } else {
            println!("Bill not found");
        }
    }

    pub fn edit_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("Bill name-{}: Ammount-${}", bill.name, bill.amount);
        }
        println!("Enter the name of the bill you want to edit: ");
        let name = match get_input() {
            Some(name) => name,
            None => {
                println!("Invalid name");
                return;
            }
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => {
                println!("Invalid amount");
                return;
            }
        };
        if bills.update(&name, amount) {
            println!("Bill updated");
        } else {
            println!("Bill not found");
        }
    }

    pub fn view_bills(bills: &Bills) {
        println!("Bills:");
        for bill in bills.get_all() {
            println!("Bill name-{}: Ammount-${}", bill.name, bill.amount);
        }
    }
}

fn main() {

    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input().expect("No data entered");
        match MainMenu::from_str(&input.as_str()) {
            Some(MainMenu::AddBill) => {
                println!("Add Bill");
                menu::add_bill(&mut bills);
            }
            Some(MainMenu::ViewBills) => {
                println!("View Bills");
                menu::view_bills(&bills);
            }
            Some(MainMenu::RemoveBill) => {
                println!("Remove Bill");
                menu::remove_bill(&mut bills);
            }
            Some(MainMenu::EditBill) => {
                println!("Edit Bill");

            }
            None => {
                println!("Invalid option");
                return;
            }
        }
    }
}