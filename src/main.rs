use chrono::Utc;
use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    clear_screen();
    let username = get_or_create_user();
    let mut session_guesses = Vec::new();

    loop {
        println!(
            "\n{}",
            format!("=== Welcome, {username} ===").bold().bright_white()
        );
        println!("{}", "1. Play Game".bright_green());
        println!("{}", "2. View Leaderboard".bright_cyan());
        println!("{}", "3. Logout".bright_red());

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                clear_screen();
                let guesses = play_game(&username);
                session_guesses.push(guesses);

                let total: u32 = session_guesses.iter().sum();
                let avg = total as f32 / session_guesses.len() as f32;
                println!("{}", format!("Session average guesses: {:.2}", avg).blue());
            }
            "2" => {
                clear_screen();
                show_leaderboard();
            }
            "3" => {
                println!("{}", format!("Logged out. Goodbye, {username}!").yellow());
                break;
            }
            _ => println!("{}", "❌ Invalid choice. Please enter 1, 2, or 3.".red()),
        }
    }
}

fn play_game(username: &str) -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    println!("{}", "\n New Game Started!".bold().cyan());
    println!("Guess the number between 1 and 100!");

    loop {
        println!("{}", "Please input your guess:".white());

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", " Invalid number. Try again.".red());
                continue;
            }
        };

        guess_count += 1;
        println!("You guessed: {}", guess.to_string().bold().bright_yellow());

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".blue()),
            Ordering::Greater => println!("{}", "Too Big".magenta()),
            Ordering::Equal => {
                let message = match guess_count {
                    1..=5 => format!(" YOU WIN in {guess_count} guesses!").green().bold(),
                    6..=10 => format!(" YOU WIN in {guess_count} guesses!")
                        .yellow()
                        .bold(),
                    _ => format!(" YOU WIN in {guess_count} guesses!").red().bold(),
                };
                println!("{}", message);
                save_score(username, guess_count);

                if let Some(best) = get_personal_best(username) {
                    if guess_count < best {
                        println!(
                            "{}",
                            format!(" New personal best! Previous best: {best} guesses.")
                                .magenta()
                                .bold()
                        );
                    }
                }

                return guess_count;
            }
        }
    }
}

fn get_or_create_user() -> String {
    let users = load_users();

    loop {
        println!("{}", "Enter your arcade initials (1–3 letters):".blue());

        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read input");
        let username = username.trim().to_uppercase();

        if username.len() < 1
            || username.len() > 3
            || !username.chars().all(|c| c.is_ascii_alphabetic())
        {
            println!("{}", "❌ Invalid initials. Use 1–3 letters only.".red());
            continue;
        }

        if let Some(stored_password) = users.get(&username) {
            println!("{}", " Enter your password:".yellow());
            let mut password = String::new();
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read input");
            let password = password.trim();

            if password == stored_password {
                println!(
                    "{}",
                    format!(" Login successful. Welcome back, {username}!").green()
                );
                update_last_login(&username);
                return username;
            } else {
                println!("{}", "❌ Wrong password.".red());
                continue;
            }
        } else {
            println!("{}", " New user. Set your password:".cyan());
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

            println!(
                "{}",
                format!(" Account created. Welcome, {username}!").green()
            );
            update_last_login(&username);
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

fn update_last_login(username: &str) {
    let timestamp = Utc::now().to_rfc3339();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("logins.txt")
        .expect("Could not open logins.txt");

    writeln!(file, "{},{}", username, timestamp).expect("Failed to write login time.");
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

fn show_leaderboard() {
    let file = match File::open("scores.txt") {
        Ok(f) => f,
        Err(_) => {
            println!("{}", "No scores recorded yet.".red());
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut scores: Vec<(String, u32, String)> = vec![];

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() == 3 {
            if let Ok(guess_count) = parts[1].parse::<u32>() {
                scores.push((parts[0].to_string(), guess_count, parts[2].to_string()));
            }
        }
    }

    scores.sort_by(|a, b| a.1.cmp(&b.1));

    println!("\n{}", " Top 10 Leaderboard:".bold().bright_white());
    println!("{:<5} {:<7} {}", "User", "Guesses", "Date".underline());
    println!("{}", "------------------------------".dimmed());

    for (user, guesses, time) in scores.iter().take(10) {
        let date = match time.parse::<chrono::DateTime<chrono::Utc>>() {
            Ok(dt) => dt.format("%Y-%m-%d").to_string(),
            Err(_) => "N/A".to_string(),
        };

        println!("{:<5} {:<7} {}", user, guesses, date);
    }
}

fn get_personal_best(username: &str) -> Option<u32> {
    if let Ok(file) = File::open("scores.txt") {
        let reader = BufReader::new(file);
        let mut best: Option<u32> = None;

        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 3 && parts[0] == username {
                if let Ok(guess_count) = parts[1].parse::<u32>() {
                    best = Some(match best {
                        Some(prev) => prev.min(guess_count),
                        None => guess_count,
                    });
                }
            }
        }

        best
    } else {
        None
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
