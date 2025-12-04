mod models;
mod manager;

use std::io;
use manager::Manager;

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{prompt}");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Use decimals like 5.0."),
        }
    }
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid integer. Try again."),
        }
    }
}

fn main() {
    let mut manager = Manager::new();

    loop {
        println!("\n=== SHOPPING LIST MANAGER ===");
        println!("1 - Add Product");
        println!("2 - Register Purchase");
        println!("3 - List Products");
        println!("4 - List Purchases");
        println!("5 - Calculate Total of Month");
        println!("6 - Lowest Price of a Product");
        println!("7 - Highest Price of a Product");
        println!("8 - Cheapest Supermarket of Month");
        println!("9 - Compare Planned Price vs Real");
        println!("10 - Update Planned Price");
        println!("11 - Most Expensive Supermarket of Month");
        println!("0 - Exit");

        let option = read_u32("Choose an option:");

        match option {
            0 => {
                println!("Exiting program...");
                break;
            }

            1 => {
                let count = read_u32("How many products do you want to add?");
                for i in 1..=count {
                    println!("\nProduct {i}:");
                    let name = read_input("Product name:");
                    let unit = read_input("Unit (kg, L, etc):");
                    let planned_qty = read_u32("Planned quantity:");  // FIXED
                    let planned_price = read_f64("Planned unit price:");
                    manager.add_product(name, unit, planned_qty, planned_price);
                }
                println!("Products added successfully!");
            }

            2 => {
                let count = read_u32("How many purchases do you want to register?");
                for i in 1..=count {
                    println!("\nPurchase {i}:");
                    let month = read_input("Month (YYYY-MM):");
                    let product = read_input("Product name:");
                    let qty = read_u32("Quantity bought:"); // FIXED
                    let price = read_f64("Unit price:");
                    let market = read_input("Supermarket:");

                    match manager.add_purchase(month, product, qty, price, market) {
                        Ok(_) => println!("Purchase registered successfully!"),
                        Err(err) => println!("Error: {err}"),
                    }
                }
            }

            3 => {
                println!("\n=== Products ===");
                manager.list_products();
            }

            4 => {
                println!("\n=== Purchases ===");
                manager.list_purchases();
            }

            5 => {
                let month = read_input("Enter month (YYYY-MM):");
                let total = manager.total_for_month(&month);
                println!("Total spent in {month}: R$ {total}");
            }

            6 => {
                let prod = read_input("Product name:");
                println!("{}", manager.lowest_price_details(&prod));
            }

            7 => {
                let prod = read_input("Product name:");
                println!("{}", manager.highest_price_details(&prod));
            }

            8 => {
                let month = read_input("Month (YYYY-MM):");
                let r = manager.cheapest_supermarket_in_month(&month);
                println!("Cheapest supermarket in {month}: {r}");
            }

            9 => {
                let name = read_input("Product name:");
                let month = read_input("Month (YYYY-MM):");
                manager.compare_planned_price(&name, &month);
            }

            10 => {
                let name = read_input("Product name to update:");
                let price = read_f64("New planned price:");
                if manager.update_planned_price(&name, price) {
                    println!("Price updated successfully!");
                } else {
                    println!("Product not found.");
                }
            }

            11 => {
                let month = read_input("Month (YYYY-MM):");
                let r = manager.most_expensive_supermarket_in_month(&month);
                println!("Most expensive supermarket in {month}: {r}");
            }

            _ => println!("Invalid option! Try again."),
        }
    }
}

