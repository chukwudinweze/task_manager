use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id: usize,
    task: String,
}

fn main() {
    let mut todo_items = Vec::new();
    let file_path = PathBuf::from("todo_list.json");

    // Load existing todo items from file
    if let Ok(loaded_items) = load_todo_list(&file_path) {
        todo_items = loaded_items;
        println!("Loaded {} todo items from file.", todo_items.len());
    }

    loop {
        println!("Choose an option:");
        println!("1. Add Todo Item");
        println!("2. List Todo Items");
        println!("3. Remove Todo Item");
        println!("4. Save and Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter the task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                add_todo_item(&mut todo_items, task.trim().to_string());
            }
            2 => list_todo_items(&todo_items),
            3 => {
                println!("Enter the ID of the item to remove:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let id: usize = id_input.trim().parse().unwrap_or(0);
                if let Err(e) = remove_todo_item(&mut todo_items, id) {
                    println!("{}", e);
                } else {
                    println!("Removed todo item with ID {}", id);
                }
            }
            4 => {
                if let Err(e) = save_todo_list(&todo_items, &file_path) {
                    println!("Error saving todo list: {}", e);
                } else {
                    println!("Todo list saved.");
                }
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_todo_item(todo_items: &mut Vec<TodoItem>, task: String) {
    let id = todo_items.len() + 1;
    let todo_item = TodoItem { id, task };
    todo_items.push(todo_item);
}

fn list_todo_items(todo_items: &Vec<TodoItem>) {
    if todo_items.is_empty() {
        println!("No todo items found.");
    } else {
        println!("Todo List:");
        for item in todo_items {
            println!("{}: {}", item.id, item.task);
        }
    }
}

fn remove_todo_item(todo_items: &mut Vec<TodoItem>, id: usize) -> Result<(), String> {
    if let Some(pos) = todo_items.iter().position(|x| x.id == id) {
        todo_items.remove(pos);
        Ok(())
    } else {
        Err(format!("Todo item with ID {} not found.", id))
    }
}

fn save_todo_list(todo_items: &Vec<TodoItem>, file_path: &PathBuf) -> io::Result<()> {
    let json = serde_json::to_string(todo_items)?;
    fs::write(file_path, json)?;
    Ok(())
}

fn load_todo_list(file_path: &PathBuf) -> Result<Vec<TodoItem>, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let todo_items: Vec<TodoItem> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    Ok(todo_items)
}
