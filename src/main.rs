use chrono::Utc;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let username = get_or_create_user();
    println!("Hello, {username}! Let's begin...");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    println!("🎯 Guess the number between 1 and 100!");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⛔ Invalid number. Try again.");
                continue;
            }
        };

        guess_count += 1;
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("🎉 YOU WIN!");
                println!("📊 Stats: {username}, it took you {guess_count} guesses.");
                save_score(&username, guess_count);
                break;
            }
        }
    }
}

fn get_or_create_user() -> String {
    let users = load_users();

    loop {
        println!("Enter your arcade initials (1–3 letters):");

        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read input");
        let username = username.trim().to_uppercase();

        if username.len() < 1
            || username.len() > 3
            || !username.chars().all(|c| c.is_ascii_alphabetic())
        {
            println!("❌ Invalid initials. Use 1–3 letters only.");
            continue;
        }

        if let Some(stored_password) = users.get(&username) {
            println!("🔐 Welcome back! Enter your password:");
            let mut password = String::new();
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read input");
            let password = password.trim();

            if password == stored_password {
                println!("✅ Login successful. Welcome back, {username}!");
                return username;
            } else {
                println!("❌ Wrong password.");
                continue;
            }
        } else {
            println!("🆕 New user. Set your password:");
            let mut password = String::new();
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read input");
            let password = password.trim().to_string();

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("users.txt")
                .expect("Could not open users.txt");
            writeln!(file, "{},{}", username, password).expect("Could not write to users.txt");

            println!("🎉 Account created. Welcome, {username}!");
            return username;
        }
    }
}

fn load_users() -> HashMap<String, String> {
    let mut users = HashMap::new();

    if let Ok(file) = File::open("users.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                users.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    users
}

fn save_score(username: &str, guesses: u32) {
    let timestamp = Utc::now().to_rfc3339();
    let record = format!("{},{},{}\n", username, guesses, timestamp);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("scores.txt")
        .expect("Failed to open or create scores.txt");

    file.write_all(record.as_bytes())
        .expect("Failed to write score to file");
}
