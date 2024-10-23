use std::io;

/// We are making a ToDo list that shows on the console

fn main() {
    let mut todo_list: Vec<String> = Vec::new();
 
    loop{
        println!("To Do List: ");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. CLOSE");
        println!("----------------------------------------");
        
        if todo_list.is_empty(){
            println!("No tasks");
        }
        else {
            for (i, task) in todo_list.iter().enumerate(){
                println!("{}. {}", i+1, task);
            }
        }

        let mut todo = String::new();

        io::stdin()
        .read_line(&mut todo)
        .expect("Failed to read line");
        
        match todo.trim().parse(){
            Ok(1) => add_task(&mut todo_list),
            Ok(2) => erase_task(&mut todo_list),
            Ok(3) => break,
            _ => println!("Invalid option"),
        }
    }
}


fn add_task(todo_list: &mut Vec<String>) {
    println!("Enter the name of the task");

    let mut task = String::new();

    io::stdin()
    .read_line(&mut task)
    .expect("Failed to read line");

    let task = task.trim().to_string();  
    if !task.is_empty() {
        todo_list.push(task);  
        println!("Task added!");
    } else {
        println!("Task cannot be empty.");
    }
}
fn erase_task(todo_list: &mut Vec<String>) {
   
    if todo_list.is_empty() {
        println!("no task to erase");
        return;
    }
    println!("Enter the number of the task to erase");
   
    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    match index.trim().parse::<usize>().ok().filter(|&i| i > 0 && i<= todo_list.len()) {
        Some(i) => {
            todo_list.remove(i-1);
            println!("Task erased!");
        }
       None => println!("Invalid task number"),
    }
}

