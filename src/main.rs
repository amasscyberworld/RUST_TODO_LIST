    
use std::io;  // First, import the io (input/output) library

fn main() {
    // 1. Basic input example
    println!("What's your name?");
    
    let mut input = String::new();  // Create a new empty string to store input
    
    io::stdin()
        .read_line(&mut input)      // Read a line of input into our string
        .expect("Failed to read line");  // Handle potential errors
    
    // Remove whitespace and newline at the end
    let name = input.trim();
    
    println!("Hello, {}!", name);
    
    // 2. Getting a number from user
    println!("\nWhat's your age?");
    
    let mut input = String::new();  // Reuse our input variable
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Convert string to number
    let age: u32 = input.trim().parse()
        .expect("Please enter a valid number!");
    
    println!("In 10 years, you'll be {}!", age + 10);





    // Create a vector to store our todos
    let mut todos: Vec<String> = Vec::new();
    
    loop {
        // Show menu
        println!("\n=== TODO LIST MENU ===");
        println!("1. Add task");
        println!("2. Show all tasks");
        println!("3. Exit");
        
        // Get user choice
        let mut input = String::new();
        println!("\nEnter your choice (1-3):");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        // Process choice
        match input.trim() {
            "1" => {
                // Add new task
                println!("Enter your task:");
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read task");
                
                // Add task to vector (remove newline with trim)
                todos.push(task.trim().to_string());
                println!("Task added successfully!");
            },
            "2" => {
                // Show all tasks
                println!("\nYour tasks:");
                if todos.is_empty() {
                    println!("No tasks yet!");
                } else {
                    for (index, task) in todos.iter().enumerate() {
                        println!("{}. {}", index + 1, task);
                    }
                }
            },
            "3" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice! Please enter 1, 2, or 3"),
        }
    }
}
