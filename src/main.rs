mod select_index;
mod todo_struct;
mod serde_func;

use crate::select_index::select_todo_index;
use crate::serde_func::{load_todos, save_todos};
use std::io::stdin;
use todo_struct::Todo;


fn main() {
    let mut todos: Vec<Todo> = load_todos();

    let mut next_id: u32 = todos
        .iter()
        .map(|todo| todo.id)
        .max()
        .unwrap_or(0) + 1;

    loop {
        let command = read_input("command: add / list / done / delete / q");
        match command.as_str() {
            "add" => {
                add_todo(&mut next_id, &mut todos);
                save_todos(&todos);
            }
            "list" => {
                list_todo(&todos);
                save_todos(&todos);
            }
            "done" => {
                finished_todo(&mut todos);
                save_todos(&todos);
            }
            "delete" => {
                delete_todo(&mut todos);
                save_todos(&todos);
            }
            "edit" => {
                edit_todo(&mut todos);
                save_todos(&todos);
            }
            "q" => {
                save_todos(&todos);
                break;
            }
            _ => {
                println!("unknown command");
            }
        }
    }
}


fn read_input(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}


fn add_todo(id: &mut u32, todos: &mut Vec<Todo>) {
    let title = read_input("title please");
    let task = read_input("please task");


    todos.push(Todo::new(*id, title, task));

    *id += 1;

    println!("Todo added");
}


fn list_todo(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos found");
        return;
    }
    for todo in todos {
        let status = if todo.status { "Done" } else { "pending" };

        println!(
            "[{}] - {} - {} - Task: {}",
            todo.id, status, todo.title, todo.task
        );
    }
}


fn finished_todo(todos: &mut Vec<Todo>) {
    let index = match select_todo_index(todos) {
        Some(i) => i,
        None => return,
    };

    todos[index].mark_done();
    println!("Task Completed");
}


fn delete_todo(todos: &mut Vec<Todo>) {
    let index = match select_todo_index(todos) {
        Some(i) => i,
        None => return,
    };

    if todos[index].is_done() {
        todos.remove(index);
        println!("Task Deleted");
    } else {
        println!("U should finish your task to delete it");
    }
}


fn edit_todo(todos: &mut Vec<Todo>) {
    let index = match select_todo_index(todos) {
        Some(i) => i,
        None => return,
    };

    let new_title = read_input("Please enter a new title");
    let new_task = read_input("Please enter a new task");

    todos[index].title = new_title;
    todos[index].task = new_task;

    println!("Task Updated");
}