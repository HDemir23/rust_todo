use std::io::stdin;

struct Todo {
    id: u32,
    title: String,
    task: String,
    status: bool,
}

fn main() {

    let mut todos: Vec<Todo> = Vec::new();
    let mut next_id: u32 = 1;
    
    loop {
        let command = read_input("command: add / list / done / delete / q");
        match command.as_str() {

            "add" => add_todo( &mut next_id,  &mut todos),
            "list" => list_todo(&todos),
            "done" => finished_todo(&mut todos),
            "delete" => delete_todo(&mut todos),
            "q" => {
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

    stdin()
        .read_line(&mut input)
        .unwrap();


    input.trim().to_string()
}
fn add_todo ( id: &mut u32, todos: &mut Vec<Todo>) {

    
                let title = read_input("title please");
                let task = read_input("please task");

                if title == "q"{
                    return;
                }
                todos.push(Todo { id: *id, title, task, status: false });

                *id += 1;

                println!("Todo added");
    }
fn list_todo (todos: &Vec<Todo>) {
    for todo in todos {
        println!("{} - {} - {} - {}", todo.id ,todo.title, todo.task, todo. status);
    }
}
fn finished_todo (todos: &mut Vec<Todo>) {
    let title = read_input("wich task finished");

    for todo in todos.iter_mut() {
        if todo.title == title {
            todo.status = true;
            println!("task completed");
        }
    }
}
fn delete_todo (todos: &mut Vec<Todo>) {
    let title = read_input("title for deletion");

    let can_del:bool = todos.iter().any(|todo: &Todo| todo.title == title && todo.status);

    if can_del {
        todos.retain(|todo| todo.title != title);
        println!("deleted")
    } else {
        println!("u can only delete finisged tasks");
    }
}