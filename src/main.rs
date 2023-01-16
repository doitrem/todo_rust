use std::io;

struct TodoItem {
    task: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<TodoItem> = vec![];

    loop {
        println!("What would you like to do?");
        println!("1. Add a task");
        println!("2. Mark a task as completed");
        println!("3. List all tasks");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_task(&mut todos),
            2 => mark_task_completed(&mut todos),
            3 => list_tasks(&todos),
            4 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn add_task(todos: &mut Vec<TodoItem>) {
    println!("Enter the task:");

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    let task = task.trim();
    todos.push(TodoItem {
        task: task.to_string(),
        completed: false,
    });

    println!("Task added!");
}

fn mark_task_completed(todos: &mut Vec<TodoItem>) {
    println!("Enter the task number:");

    let mut task_num = String::new();
    io::stdin().read_line(&mut task_num).unwrap();

    let task_num: usize = match task_num.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if let Some(task) = todos.get_mut(task_num - 1) {
        task.completed = true;
        println!("Task marked as completed!");
    } else {
        println!("Invalid task number!");
    }
}

fn list_tasks(todos: &Vec<TodoItem>) {
    for (i, todo) in todos.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, todo.task, if todo.completed {"x"} else {" "});
    }
}
