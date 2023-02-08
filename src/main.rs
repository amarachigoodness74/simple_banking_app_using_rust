use std::io::stdin;

// A simple banking app that allows a user to withdraw, deposit and check balance

fn main() {
    let mut balance: f32 = 5_000_000.00;

    let mut action = String::new();
    println!("Choose option based on action you want to perform! \n 1. Withdrawal \n 2. Deposit \n 3. Check Balance");
    stdin()
        .read_line(&mut action)
        .expect("Failed to read action");
    let action: char = action.trim().chars().next().unwrap();
    match action {
        '1' => {
            let mut amt_to_withdraw = String::new();
            println!("Please enter the amount you want to withdraw");
            stdin()
                .read_line(&mut amt_to_withdraw)
                .expect("Failed to read amount");
            let amt_to_withdraw: f32 = amt_to_withdraw
                .trim()
                .parse()
                .expect("Amount entered was not a number");
            if amt_to_withdraw >= balance {
                println!("Insufficient Funds");
            } else {
                balance -= amt_to_withdraw;
                println!(
                    "Please take your cash of {}, balance is {}",
                    amt_to_withdraw, balance
                );
            }
        }
        '2' => {
            let mut amt_to_deposit = String::new();
            println!("Please enter the amount you want to deposit");
            stdin()
                .read_line(&mut amt_to_deposit)
                .expect("Failed to read amount");
            let amt_to_deposit: f32 = amt_to_deposit
                .trim()
                .parse()
                .expect("Amount entered was not a number");
            balance += amt_to_deposit;
            println!(
                "You successfully deposited {} into your account, balance is {}",
                amt_to_deposit, balance
            );
        }
        '3' => println!("You have {} left in your account", balance),
        _ => println!("Sorry! you choose a wrong option, {}", action),
    };
}
