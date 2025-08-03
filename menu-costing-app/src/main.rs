use std::io;
use std::process;

fn main() {
    _menu();
}

fn _menu() {
    println!("Welcome to costing menu!");
    println!("1. Create a menu");
    println!("2. Update menu");
    println!("3. Delete menu");
    println!("4. Calculate menu cost");

    loop {
        println!("Enter your choice: ");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading line");

        match choice.trim().parse::<i32>() {
            Ok(0) =>{
                println!("Exiting the program...");
                process::exit(0);
            }
            Ok(1) => {
                _create_menu();
                break;
            }
            Ok(2) => {
                _update_menu();
                break;
            }
            Ok(3) => {
                _delete_menu();
                break;
            }
            Ok(4) => {
                _calculate_menu_cost();
                break;
            }
            Ok(_) => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

fn _create_menu() {
    println!("Create Menu!");
}

fn _delete_menu() {
    println!("Delete Menu!");
}

fn _update_menu() {
    println!("Update Menu!");
}

fn _calculate_menu_cost() {
    println!("Calculating Menu Cost...");
}