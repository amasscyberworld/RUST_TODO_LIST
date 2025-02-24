use std::io::{self, Write};
use std::fs;
use colored::*;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Local};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    category: String,
    priority: Priority,
    due_date: Option<String>,
    created_at: String,
    completed_at: Option<String>,
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to save tasks");
    fs::write("tasks.json", json).expect("Failed to write tasks");
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

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

fn get_due_date() -> Option<String> {
    println!("\n{}", "Enter due date (YYYY-MM-DD HH:MM or press Enter to skip):".cyan());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    
    if input.is_empty() {
        return None;
    }

    match NaiveDateTime::parse_from_str(&format!("{}", input), "%Y-%m-%d %H:%M") {
        Ok(_) => Some(input.to_string()),
        Err(_) => {
            println!("{}", "Invalid date format! No due date set.".red());
            None
        }
    }
}

fn show_statistics(tasks: &Vec<Task>) {
    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.completed).count();
    let pending = total - completed;
    
    println!("\n{}", "=== Statistics ===".cyan().bold());
    println!("Total tasks: {}", total.to_string().yellow());
    println!("Completed: {}", completed.to_string().green());
    println!("Pending: {}", pending.to_string().red());
    
    // Show tasks by priority
    let high_priority = tasks.iter().filter(|t| t.priority == Priority::High && !t.completed).count();
    println!("High priority pending: {}", high_priority.to_string().red());
}

fn main() {
    let mut tasks = load_tasks();
    let mut next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    println!("{}", "Welcome to Rust Todo List!".green().bold());

    loop {
        println!("\n{}", "=== TODO LIST MENU ===".cyan().bold());
        println!("1. {}", "Add task".green());
        println!("2. {}", "List all tasks".green());
        println!("3. {}", "Mark task as complete".green());
        println!("4. {}", "Delete task".green());
        println!("5. {}", "Edit task".yellow());
        println!("6. {}", "Filter tasks by category".blue());
        println!("7. {}", "Show statistics".cyan());
        println!("8. {}", "Exit".red());

        print!("\n{}", "Enter your choice (1-8): ".cyan());
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
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

                let priority = get_priority();
                let due_date = get_due_date();

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
                if tasks.is_empty() {
                    println!("{}", "No tasks yet!".red());
                } else {
                    println!("\n{}", "Your Tasks:".cyan().bold());
                    for task in &tasks {
                        let status = if task.completed { 
                            "✓".green() 
                        } else { 
                            "○".red() 
                        };
                        let priority_color = match task.priority {
                            Priority::High => "!".red(),
                            Priority::Medium => "!".yellow(),
                            Priority::Low => "!".green(),
                        };
                        print!("{}. [{}] {} {} ({})", 
                            task.id.to_string().blue(),
                            status,
                            task.description.white(),
                            priority_color,
                            task.category.yellow());
                        
                        if let Some(due_date) = &task.due_date {
                            print!(" Due: {}", due_date.cyan());
                        }
                        println!();
                    }
                }
            },
            "3" => {
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
                print!("{}", "Enter task ID to edit: ".cyan());
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read ID");

                if let Ok(id) = id_input.trim().parse::<usize>() {
                    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
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

                        println!("Update priority? (y/n): ");
                        let mut update_priority = String::new();
                        io::stdin()
                            .read_line(&mut update_priority)
                            .expect("Failed to read input");
                        
                        if update_priority.trim().to_lowercase() == "y" {
                            task.priority = get_priority();
                        }

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
                show_statistics(&tasks);
            },
            "8" => {
                println!("{}", "Goodbye!".green().bold());
                break;
            },
            _ => println!("{}", "Invalid choice! Please enter 1-8".red()),
        }
    }
}
