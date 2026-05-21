/*
Copyright (C) 2026 Undercat037
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

 A simple OSINT tool for searching database records by phone number
*/

use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("===============");
    println!("TESTING PROGRAM");
    println!("===============");

    println!("Number Checker // OSINT ");

    // Database structure (Имя с большой буквы по конвенции Rust)
    #[derive(Debug, Deserialize)]
    struct Database {
        id: u32,
        name: String,
        birth_date: String,
        phone_num: String,
        org: String,
        weapon_licence: bool,
        email: String,
    }

    loop {
        // Enter number
        let mut number = String::new();
        println!("===============================================");
        println!("Enter the number phone (or type 'exit' to quit):");
        io::stdin().read_line(&mut number).expect("Error read");

        let trimm = number.trim();

        if trimm == "exit" {
            break;
        }

        if !trimm.chars().all(|c| c.is_ascii_digit()) {
            println!("Error: only digits allowed\n");
            continue;
        }

        let len = trimm.len();

        if len == 10 || len == 12 {
            let full_num: String = if len == 10 {
                format!("38{}", trimm)
            } else {
                trimm.to_string()
            };

            // Read
            let file = File::open("db.csv")?;
            let mut rdr = Reader::from_reader(file);

            // checker
            println!("===============================================");
            println!("Searching {}...", full_num);
            println!("===============================================");

            let mut found = false;

            for result in rdr.deserialize() {
                let user: Database = result?;

                if user.phone_num == full_num {
                    println!(
                        "Sussces! ID: {}, Name: {}, Birth Date: {}, Number Phone: {}, Org: {}, Email: {}, Weapon Licence: {}",
                        user.id,
                        user.name,
                        user.birth_date,
                        user.phone_num,
                        user.org,
                        user.email,
                        user.weapon_licence
                    );
                    found = true;
                }
            }

            if !found {
                println!("Nothing found for {}", full_num);
            }
        } else {
            println!("Error: Phone number must be 10 or 12 digits long.");
        }
    }

    Ok(())
}
