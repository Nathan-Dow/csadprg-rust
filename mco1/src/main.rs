
use std::io::{self, Write};

struct Account {
    name: String,
    balance: f64,
}

fn main() {
    main_menu();
}

fn main_menu(){

    let mut accounts: Vec<Account> = Vec::new();
    loop {
        println!("Select Transaction:");
        println!("[1] Register Account Name");
        println!("[2] Deposit Amount");
        println!("[3] Withdraw Amount");
        println!("[4] Currency Exchange");
        println!("[5] Record Exchange Rates");
        println!("[6] Show Interest Computation");
        println!("[0] Exit Program");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        let choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        if choice == 0 {
            println!("Exiting program...");
            break;
        }

        match choice {
            1 => accounts.push(register_account()),
            2 => deposit_amount(),
            3 => withdraw_amount(),
            4 => currency_exchange(),
            5 => record_exchange_rates(),
            6 => show_interest_computation(),
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn register_account() -> Account {
    loop {
        println!("Register Account Name");
        let name = get_input("Account Name:");

        let action = get_input("Back to the Main Menu (Y/N): ");

        if action.eq_ignore_ascii_case("Y") {
            return Account {
                name,
                balance: 0.0,
            };
        } else {
            println!("Account not saved. Try again...");
        }
    }
}


fn deposit_amount() {
    println!("--- Deposit Amount ---");

}

fn withdraw_amount() {
    println!("--- Withdraw Amount ---");

}

fn currency_exchange() {
    println!("--- Currency Exchange ---");

}

fn record_exchange_rates() {
    println!("--- Record Exchange Rates ---");

}

fn show_interest_computation() {
    println!("--- Show Interest Computation ---");

}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}