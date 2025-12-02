use std::{fs, net::Shutdown};

use chrono::{NaiveDate, Local};
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    completed: bool,
    due_date: Option<String>, 
}

#[derive(Debug, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
    filename: String,
}

impl TaskManager {
    fn new(path: String) -> TaskManager {
        TaskManager { tasks: Vec::new(), next_id: 0, filename: path }
    }

    fn is_existing_task(&self, title: Option<&str>, id: Option<u32>) -> (bool, Option<&Task>) {
        if let Some(title) = title {
            for t in self.tasks.iter() {
                if t.title == title { return (true, Some(t)) };
        }
        } else if let Some(id) = id {
            for t in self.tasks.iter() {
                if t.id == id { return (true, Some(t)) };
            }
        }

        (false, None)
    }

    fn add_task(&mut self, title: &str, priority: Priority, due_date: Option<String>) -> Result<(), String> {
        // Check if the title is unique
        if self.is_existing_task(Some(title), None).0 { return Err("There is an existing task with the same title.".to_string()) }

        // Check if the due date is in the future
        if let Some(due_date_str) = &due_date {
            let due_date_parsed = NaiveDate::parse_from_str(due_date_str, "%Y-%m-%d")
                .map_err(|_| "Invalid date format. Please use YYYY-MM-DD.".to_string())?;
            let today = Local::now().date_naive();
            if due_date_parsed <= today {
                return Err("The due date of the task has to be in the future.".to_string());
            }
        }

        self.tasks.push(Task {
            id: self.next_id,
            title: title.to_string(),
            priority: priority,
            completed: false,
            due_date: due_date,
        });

        self.next_id +=1;

        Ok(())
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
    for t in &mut self.tasks {
        if t.id == id {
            t.completed = true;
            return Ok(());
        }
    }
    Err("Task not found".to_string())
}

    fn remove_task(&mut self, id: u32) -> Result<(), String> {
        // Check if title or id matches & remove it if so
        let pos = self.tasks.iter().position(|t| t.id == id);
        
        match pos {
            Some(p) => {
                self.tasks.remove(t.id as usize);
                Ok(())
            },
            None => Err("Task not found".to_string()),
        }
    }

    fn list_tasks(&self, filter: Option<Priority>) -> Vec<String> {
        let mut result = Vec::new();

        for t in &self.tasks {
            let show = match &filter {
                Some(prio) => t.priority == *prio,
                None => true
            };

            if show {
                let line = format!("Task ID: {} | Title: {} | Status: {} | Prio: {:?} | Due: {:?}", t.id, t.title, t.completed, t.priority, t.due_date);
                println!("{}", line);
                &result.push(line);
            }
        }
        result
    }

    fn write_to_file(&self) -> Result<(), String> {
        let mut contnet = String::new();

        for line in self.list_tasks(None) {
            &contnet.push_str(&line);
        }
    
        fs::write(&self.filename, contnet)
            .map_err(|_| "Could not write to file".to_string())
    }

    fn load_from_file(&self) -> Result<(), String> {
        let content  = fs::read_to_string(&self.filename)
            .map_err(|_| "File could not be read to...".to_string())?;

        for line in content.lines() {
            println!("{}", line);
        }
        
        Ok(())  
    }
}   



fn main() {
    let path = String::from("C:\\Users\\jklas\\rust_tests\\first-steps\\app.log");
    let mut manager = TaskManager::new(path);

    // Try loading existing tasks
    match manager.load_from_file() {
        Ok(_) => println!("Loaded existing tasks!\n"),
        Err(_) => println!("No existing tasks found, starting fresh.\n"),
    }

    // Add some tasks
    manager.add_task("Buy groceries", Priority::Medium, Some("2024-12-15".to_string()));
    manager.add_task("Finish Rust project", Priority::High, None);
    manager.add_task("Call mom", Priority::Low, Some("2024-12-10".to_string()));
    manager.add_task("Pay bills", Priority::High, Some("2024-12-01".to_string()));

    // List all tasks
    println!("=== All Tasks ===");
    manager.list_tasks(None);

    // List only high priority
    println!("\n=== High Priority Only ===");
    manager.list_tasks(Some(Priority::High));

    // Complete a task
    println!("\n=== Completing task 1 ===");
    match manager.complete_task(1) {
        Ok(_) => println!("Task completed!"),
        Err(e) => println!("Error: {}", e),
    }

    // Try completing non-existent task
    match manager.complete_task(99) {
        Ok(_) => println!("Task completed!"),
        Err(e) => println!("Error: {}", e),
    }

    // Remove a task
    println!("\n=== Removing task 2 ===");
    match manager.remove_task(2) {
        Ok(_) => println!("Task removed!"),
        Err(e) => println!("Error: {}", e),
    }

    // List again to see changes
    println!("\n=== Updated Task List ===");
    manager.list_tasks(None);

    // Save to file
    println!("\n=== Saving ===");
    match manager.save_to_file() {
        Ok(_) => println!("Tasks saved to file!"),
        Err(e) => println!("Error saving: {}", e),
    }
}