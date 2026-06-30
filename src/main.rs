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

            "add" => {
                let title = read_input("title please");
                let task = read_input("please task");

                if title == "q"{
                    break;
                }


                todos.push(Todo { id: next_id, title, task, status: false });

                next_id += 1;

                println!("Todo added");
            }

            "list" => {
                for todo in &todos {
                    println!("{} - {} - {} - Done: {}", todo.id , todo.title, todo.task, todo.status);
                }
            }


            "done" => {
                let title = read_input("which title is done");

                for todo in &mut todos {
                    if todo.title == title {
                        todo.status = true;
                        println!("task completed")
                    }
                }
            }

            "delete" => {
                let title = read_input("which task do u want to delete");

                let can_del = todos
                    .iter()
                    .any(|todo| todo.title == title && todo.status ==true);


                if can_del {
                    todos.retain(|todo| todo.title != title);
                    println!("deleted")
                } else {
                    println!("u can only delete finisged tasks");
                }
            }

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