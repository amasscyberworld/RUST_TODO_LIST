// Import necessary libraries
use std::io::{self, Write};  // For input/output operations
use std::fs;                 // For file operations
use colored::*;              // For colored terminal output
use serde::{Serialize, Deserialize};  // For JSON serialization
use chrono::{NaiveDateTime, Local};   // For date/time handling

// Define user profile structure
#[derive(Serialize, Deserialize, Clone)]
struct UserProfile {
    name: String,
    age: u32,
    join_date: String,
}

// Define priority levels for tasks
#[derive(Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

// Define task structure
#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,                    // Unique identifier for each task
    description: String,          // Task description
    completed: bool,              // Task completion status
    category: String,             // Task category
    priority: Priority,           // Task priority level
    due_date: Option<String>,     // Optional due date
    created_at: String,           // Creation timestamp
    completed_at: Option<String>, // Completion timestamp
}

// Function to save tasks to a JSON file
fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to save tasks");
    fs::write("tasks.json", json).expect("Failed to write tasks");
}

// Function to load tasks from JSON file
fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

// Function to save user profile to JSON file
fn save_profile(profile: &UserProfile) {
    let json = serde_json::to_string_pretty(profile).expect("Failed to serialize profile");
    fs::write("profile.json", json).expect("Failed to write profile");
}

// Function to load user profile from JSON file
fn load_profile() -> Option<UserProfile> {
    match fs::read_to_string("profile.json") {
        Ok(contents) => serde_json::from_str(&contents).ok(),
        Err(_) => None,
    }
}

// Function to create new user profile
fn setup_profile() -> UserProfile {
    println!("{}", "\nLet's set up your profile!".cyan().bold());
    
    // Get user's name
    print!("{}", "What's your name? ".cyan());
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    // Get user's age
    print!("{}", "What's your age? ".cyan());
    io::stdout().flush().unwrap();
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read age");
    
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number!");

    // Create and save new profile
    let profile = UserProfile {
        name: name.trim().to_string(),
        age,
        join_date: Local::now().format("%Y-%m-%d %H:%M").to_string(),
    };

    save_profile(&profile);
    println!("{}", "Profile created successfully!".green());
    profile
}

// Function to get task priority from user
fn get_priority() -> Priority {
    loop {
        println!("\n{}", "Select Priority:".cyan());
        println!("1. {}", "High".red());
        println!("2. {}", "Medium".yellow());
        println!("3. {}", "Low".green());
        
        print!("Enter choice (1-3): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => return Priority::High,
            "2" => return Priority::Medium,
            "3" => return Priority::Low,
            _ => println!("{}", "Invalid choice! Try again.".red()),
        }
    }
}

// Function to get due date from user
fn get_due_date() -> Option<String> {
    println!("\n{}", "Enter due date (YYYY-MM-DD HH:MM or press Enter to skip):".cyan());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    
    if input.is_empty() {
        return None;
    }

    // Validate date format
    match NaiveDateTime::parse_from_str(&format!("{}", input), "%Y-%m-%d %H:%M") {
        Ok(_) => Some(input.to_string()),
        Err(_) => {
            println!("{}", "Invalid date format! No due date set.".red());
            None
        }
    }
}

// Function to display task and user statistics
fn show_statistics(tasks: &Vec<Task>, profile: &UserProfile) {
    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.completed).count();
    let pending = total - completed;
    
    // Display user profile information
    println!("\n{}", "=== User Profile ===".cyan().bold());
    println!("Name: {}", profile.name.yellow());
    println!("Age: {}", profile.age.to_string().yellow());
    println!("Member since: {}", profile.join_date.cyan());
    
    // Display task statistics
    println!("\n{}", "=== Task Statistics ===".cyan().bold());
    println!("Total tasks: {}", total.to_string().yellow());
    println!("Completed: {}", completed.to_string().green());
    println!("Pending: {}", pending.to_string().red());
    
    let high_priority = tasks.iter().filter(|t| t.priority == Priority::High && !t.completed).count();
    println!("High priority pending: {}", high_priority.to_string().red());
}

fn main() {
    // Load or create user profile
    let profile = load_profile().unwrap_or_else(setup_profile);
    let mut tasks = load_tasks();
    let mut next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    // Display welcome message
    println!("{}", format!("\nWelcome back, {}!", profile.name).green().bold());

    // Main program loop
    loop {
        // Display menu options
        println!("\n{}", "=== TODO LIST MENU ===".cyan().bold());
        println!("1. {}", "Add task".green());
        println!("2. {}", "List all tasks".green());
        println!("3. {}", "Mark task as complete".green());
        println!("4. {}", "Delete task".green());
        println!("5. {}", "Edit task".yellow());
        println!("6. {}", "Filter tasks by category".blue());
        println!("7. {}", "Show statistics".cyan());
        println!("8. {}", "Update profile".yellow());
        println!("9. {}", "Exit".red());

        // Get user choice
        print!("\n{}", "Enter your choice (1-9): ".cyan());
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        // Process user choice
        match choice.trim() {
            "1" => {
                // Add new task
                print!("{}", "Enter task description: ".cyan());
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read description");

                print!("{}", "Enter task category: ".cyan());
                io::stdout().flush().unwrap();
                let mut category = String::new();
                io::stdin()
                    .read_line(&mut category)
                    .expect("Failed to read category");

                // Get task priority and due date
                let priority = get_priority();
                let due_date = get_due_date();

                // Create and save new task
                tasks.push(Task {
                    id: next_id,
                    description: description.trim().to_string(),
                    completed: false,
                    category: category.trim().to_string(),
                    priority,
                    due_date,
                    created_at: Local::now().format("%Y-%m-%d %H:%M").to_string(),
                    completed_at: None,
                });
                
                save_tasks(&tasks);
                next_id += 1;
                println!("{}", "✓ Task added successfully!".green());
            },
            "2" => {
                // List all tasks
                if tasks.is_empty() {
                    println!("{}", "No tasks yet!".red());
                } else {
                    println!("\n{}", "Your Tasks:".cyan().bold());
                    for task in &tasks {
                        // Display task status
                        let status = if task.completed { 
                            "✓".green() 
                        } else { 
                            "○".red() 
                        };
                        // Display priority indicator
                        let priority_color = match task.priority {
                            Priority::High => "!".red(),
                            Priority::Medium => "!".yellow(),
                            Priority::Low => "!".green(),
                        };
                        // Print task details
                        print!("{}. [{}] {} {} ({})", 
                            task.id.to_string().blue(),
                            status,
                            task.description.white(),
                            priority_color,
                            task.category.yellow());
                        
                        // Show due date if set
                        if let Some(due_date) = &task.due_date {
                            print!(" Due: {}", due_date.cyan());
                        }
                        println!();
                    }
                }
            },
            "3" => {
                // Mark task as complete
                print!("{}", "Enter task ID to mark as complete: ".cyan());
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read ID");

                if let Ok(id) = id_input.trim().parse::<usize>() {
                    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                        task.completed = true;
                        task.completed_at = Some(Local::now().format("%Y-%m-%d %H:%M").to_string());
                        save_tasks(&tasks);
                        println!("{}", "✓ Task marked as complete!".green());
                    } else {
                        println!("{}", "Task not found!".red());
                    }
                } else {
                    println!("{}", "Invalid ID!".red());
                }
            },
            "4" => {
                // Delete task
                print!("{}", "Enter task ID to delete: ".cyan());
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read ID");

                if let Ok(id) = id_input.trim().parse::<usize>() {
                    if let Some(index) = tasks.iter().position(|t| t.id == id) {
                        tasks.remove(index);
                        save_tasks(&tasks);
                        println!("{}", "✓ Task deleted!".green());
                    } else {
                        println!("{}", "Task not found!".red());
                    }
                } else {
                    println!("{}", "Invalid ID!".red());
                }
            },
            "5" => {
                // Edit task
                print!("{}", "Enter task ID to edit: ".cyan());
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read ID");

                if let Ok(id) = id_input.trim().parse::<usize>() {
                    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                        // Edit description
                        println!("Current description: {}", task.description);
                        print!("Enter new description (or press Enter to skip): ");
                        io::stdout().flush().unwrap();
                        let mut new_desc = String::new();
                        io::stdin()
                            .read_line(&mut new_desc)
                            .expect("Failed to read description");
                        
                        if !new_desc.trim().is_empty() {
                            task.description = new_desc.trim().to_string();
                        }

                        // Edit category
                        println!("Current category: {}", task.category);
                        print!("Enter new category (or press Enter to skip): ");
                        io::stdout().flush().unwrap();
                        let mut new_cat = String::new();
                        io::stdin()
                            .read_line(&mut new_cat)
                            .expect("Failed to read category");
                        
                        if !new_cat.trim().is_empty() {
                            task.category = new_cat.trim().to_string();
                        }

                        // Edit priority
                        println!("Update priority? (y/n): ");
                        let mut update_priority = String::new();
                        io::stdin()
                            .read_line(&mut update_priority)
                            .expect("Failed to read input");
                        
                        if update_priority.trim().to_lowercase() == "y" {
                            task.priority = get_priority();
                        }

                        // Edit due date
                        println!("Update due date? (y/n): ");
                        let mut update_due_date = String::new();
                        io::stdin()
                            .read_line(&mut update_due_date)
                            .expect("Failed to read input");
                        
                        if update_due_date.trim().to_lowercase() == "y" {
                            task.due_date = get_due_date();
                        }

                        save_tasks(&tasks);
                        println!("{}", "✓ Task updated!".green());
                    } else {
                        println!("{}", "Task not found!".red());
                    }
                } else {
                    println!("{}", "Invalid ID!".red());
                }
            },
            "6" => {
                // Filter tasks by category
                print!("{}", "Enter category to filter by: ".cyan());
                io::stdout().flush().unwrap();
                let mut category = String::new();
                io::stdin()
                    .read_line(&mut category)
                    .expect("Failed to read category");
                let category = category.trim();

                let filtered_tasks: Vec<&Task> = tasks.iter()
                    .filter(|t| t.category.to_lowercase() == category.to_lowercase())
                    .collect();

                if filtered_tasks.is_empty() {
                    println!("{}", format!("No tasks found in category: {}", category).red());
                } else {
                    println!("\n{}", format!("Tasks in category '{}':", category).cyan().bold());
                    for task in filtered_tasks {
                        let status = if task.completed { "✓".green() } else { "○".red() };
                        println!("{}. [{}] {}", task.id.to_string().blue(), status, task.description);
                    }
                }
            },
            "7" => {
                // Show statistics
                show_statistics(&tasks, &profile);
            },
            "8" => {
                // Update profile
                println!("\n{}", "=== Update Profile ===".cyan().bold());
                println!("Current name: {}", profile.name);
                print!("Enter new name (or press Enter to skip): ");
                io::stdout().flush().unwrap();
                let mut new_name = String::new();
                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read name");
                
                if !new_name.trim().is_empty() {
                    let mut updated_profile = profile.clone();
                    updated_profile.name = new_name.trim().to_string();
                    save_profile(&updated_profile);
                    println!("{}", "Profile updated!".green());
                    return;  // Restart the program to load new profile
                }
            },
            "9" => {
                // Exit program
                println!("{}", format!("Goodbye, {}!", profile.name).green().bold());
                break;
            },
            _ => println!("{}", "Invalid choice! Please enter 1-9".red()),
        }
    }
}
