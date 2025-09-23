
use std::io::{self, Write};

struct Account {
    name: String,
    balance: f64,
}

impl Account {
    fn new(name: String) -> Self {
        Account { name, balance: 0.0 }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if amount <= self.balance {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}


fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    main_menu(&mut accounts);
}

fn main_menu(accounts: &mut Vec<Account>){

    loop {
        println!("Select Transaction:");
        println!("[1] Register Account Name");
        println!("[2] Deposit Amount");
        println!("[3] Withdraw Amount");
        println!("[4] Currency Exchange");
        println!("[5] Record Exchange Rates");
        println!("[6] Show Interest Computation");
        println!("[0] Exit Program");

        let choice: u8 = get_number("Choice: ");

        if choice == 0 {
            println!("Exiting program...");
            break;
        }

        match choice {
            1 => register_account(accounts),
            2 => deposit_amount(accounts),
            3 => withdraw_amount(accounts),
            4 => currency_exchange(),
            5 => record_exchange_rates(),
            6 => show_interest_computation(),
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn register_account(accounts: &mut Vec<Account>) {

    loop {
        println!("Register Account Name");
        let name = get_input("Account Name:");
        accounts.push(Account::new(name));

        let action = get_input("\nBack to the Main Menu (Y/N): ");

        if action.eq_ignore_ascii_case("Y") {
            break;
        }
    }
}


fn deposit_amount(accounts: &mut Vec<Account>) {
    if accounts.is_empty() {
        println!("No accounts available!");
        return;
    }

    loop {
        println!("Deposit Amount");
        let name = get_input("Account Name:");

        // Find account
        if let Some(acc) = accounts.iter_mut().find(|a| a.name == name) {
            
            println!("Current Balance: {:.2}", acc.get_balance());
            println!("Currency: PHP\n");
            let amount_str = get_input("Deposit Amount:");
            
            match amount_str.trim().parse::<f64>() {

                Ok(amount) => {
                    
                    acc.deposit(amount);
                    println!("Updated Balance: {:.2}\n", acc.get_balance());                    
                }
                _ => {
                    println!("Invalid amount, try again.");
                    continue;
                }
            }

            let action = get_input("Back to the Main Menu (Y/N): ");
            if action.eq_ignore_ascii_case("Y") {
                break;
            } else {
                println!("Deposit Again...");
            }
        } else {
            println!("Account not found!");
        }
    }
}


fn withdraw_amount(accounts: &mut Vec<Account>) {

    if accounts.is_empty() {
        println!("No accounts available!");
        return;
    }

    loop {
        println!("Withdraw Amount");
        let name = get_input("Account Name:");

        // Find account
        if let Some(acc) = accounts.iter_mut().find(|a| a.name == name) {
            println!("Current Balance: {:.2}", acc.get_balance());
            println!("Currency: PHP\n");

            let amount_str = get_input("Withdraw Amount:");
            
            match amount_str.trim().parse::<f64>() {
                Ok(amount) if (acc.withdraw(amount)) => {

                    println!("Updated Balance: {:.2}\n", acc.get_balance());
                }
                _ => {
                    println!("Insufficient funds, try again.");
                    continue;
                }
            }

            let action = get_input("Back to the Main Menu (Y/N): ");
            if action.eq_ignore_ascii_case("Y") {
                break;
            } else {
                println!("Withdraw Again...");
            }
        } else {
            println!("Account not found!");
        }
    }

    

}

fn print_currency_menu(){
    println!("Source Currency Option:");
    println!("[1] Philippine Peso (PHP)");
    println!("[2] United States Dollar (USD)");
    println!("[3] Japanese Yen (JPY)");
    println!("[4] British Pound Sterling (GBP)");
    println!("[5] Euro (EUR)");
    println!("[6] Chinese Yuan Renminni (CNY)");
}

fn currency_exchange() {
    loop{
        println!("Foreign Currency Exchange");
        print_currency_menu();
        
        let sourceChoice: u8 = get_number("Source Currency: ");
        let amount: i128  = get_number("Source Amount: ");

        println!("Exchanged Currency Options:");
        print_currency_menu();

        let exchangeChoice: u8 = get_number("Exchange Currency: ");

        /*
        Yet to implement the actual currency exchange rates
         */

        let action = get_input("Convert another currency (Y/N)? . . .");
        
        if action.eq_ignore_ascii_case("Y") {
            break;
        } 
    }
    

}

fn record_exchange_rates() {
    println!("--- Record Exchange Rates ---");

}

fn show_interest_computation() {
    println!("--- Show Interest Computation ---");

}

fn get_number<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = get_input(prompt);
        match input.trim().parse::<T>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}